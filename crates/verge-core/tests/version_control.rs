//! File: `version_control.rs`
//!
//! Deskripsi: Test integrasi alur versioning tabel di atas prolly tree.
//! Layer: tests
//! Tanggung jawab: Membuktikan import, commit, log, snapshot, dan dedup di disk nyata.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `support/mod.rs`
//! Related issues: #8 (Milestone 2), #18 (Milestone 3)
//! Related ADR: ADR-0005 (blok content-addressed), ADR-0006 (prolly tree)

mod support;

use std::path::PathBuf;

use support::flow::{import_and_commit, snapshot, users};
use support::{cleanup, count_blocks, harness, workspace, write_source};

use verge_core::application::version_control::use_cases::read_history::{
    read_history, ReadHistoryInput,
};
use verge_core::application::version_control::use_cases::stage_table::{
    stage_table, StageTableInput,
};
use verge_core::domain::storage::ports::block_store::Store;
use verge_core::domain::table::ports::table_workspace::TableWorkspace;
use verge_core::infrastructure::table::file_system::file_table_source::FileTableSource;

/// Isi tabel kecil yang dipakai test dedup dan berbagi daun.
const SMALL_TABLE: &[u8] = b"id,total\n1,10\n2,20\n";

/// Menjalankan satu tahap staging tabel `users` dari `source`.
fn stage(harness: &support::Harness, source: PathBuf) {
    stage_table(
        &StageTableInput {
            table: users(),
            source,
        },
        &FileTableSource,
        &harness.workspace_port,
        &harness.store,
    )
    .expect("stage data");
}

#[test]
fn alur_import_commit_log_show_terbukti_di_filesystem_nyata() {
    let workspace = workspace("alur");
    let harness = harness(&workspace);

    let pertama: PathBuf = write_source(&workspace, "satu.csv", b"id,name\n1,ana\n");
    let commit_satu = import_and_commit(&harness, &pertama, "feat: seed users", "ana", 1_000);
    let kedua: PathBuf = write_source(&workspace, "dua.csv", b"id,name\n1,ana\n2,budi\n");
    let commit_dua = import_and_commit(&harness, &kedua, "feat: add budi", "budi", 2_000);

    let entries = read_history(
        &ReadHistoryInput {
            table: users(),
            limit: 10,
        },
        &harness.refs,
        &harness.commits,
    )
    .expect("baca riwayat");
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].id, commit_dua);
    assert_eq!(entries[0].summary, "feat: add budi");
    assert_eq!(entries[1].id, commit_satu);
    assert_eq!(entries[1].author, "ana");

    assert_eq!(
        snapshot(&harness, &commit_satu.to_string()),
        b"id,name\n1,ana\n"
    );
    assert_eq!(snapshot(&harness, "HEAD"), b"id,name\n1,ana\n2,budi\n");
    assert_eq!(snapshot(&harness, "main"), b"id,name\n1,ana\n2,budi\n");

    cleanup(&workspace);
}

#[test]
fn staging_ulang_dengan_isi_sama_tidak_menggandakan_blok() {
    let workspace = workspace("dedup");
    let harness = harness(&workspace);
    let blocks_awal = count_blocks(&harness.layout);
    let source = write_source(&workspace, "users.csv", SMALL_TABLE);

    stage(&harness, source.clone());
    let blocks_setelah_satu = count_blocks(&harness.layout);
    stage(&harness, source.clone());
    stage(&harness, source);

    assert!(
        blocks_setelah_satu > blocks_awal,
        "satu stage menulis node tree"
    );
    assert_eq!(
        count_blocks(&harness.layout),
        blocks_setelah_satu,
        "tiga stage dengan isi sama hanya menambah blok sekali"
    );
    let root = harness
        .workspace_port
        .staged(&users())
        .expect("baca pointer data kerja")
        .expect("tabel sudah di-stage");
    assert!(harness.store.contains(&root), "akar tree harus tersimpan");
    cleanup(&workspace);
}

#[test]
fn baris_tidak_berubah_memakai_node_yang_sama_antar_import() {
    let workspace = workspace("berbagi");
    let harness = harness(&workspace);

    stage(&harness, write_source(&workspace, "satu.csv", SMALL_TABLE));
    let blocks_awal = count_blocks(&harness.layout);
    stage(
        &harness,
        write_source(&workspace, "dua.csv", b"id,total\n1,10\n2,99\n"),
    );
    let blocks_baru = count_blocks(&harness.layout) - blocks_awal;

    assert_eq!(
        blocks_baru, 2,
        "hanya daun yang berubah dan akar barunya yang ditulis; sisanya dipakai ulang"
    );
    cleanup(&workspace);
}
