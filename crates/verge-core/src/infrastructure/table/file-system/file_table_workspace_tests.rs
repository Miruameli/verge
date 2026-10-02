//! File: `file_table_workspace_tests.rs`
//!
//! Deskripsi: Test integrasi `FileTableWorkspace` pada repository sementara.
//! Layer: infrastructure/table/file-system
//! Tanggung jawab: Membuktikan staging hanya menunjuk akar dan menjaga pointer.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0 · Dependencies: `file_table_workspace.rs`
//! Related issues: #8 (Milestone 2), #18 (Milestone 3)
//! Related ADR: ADR-0005 (blok content-addressed), ADR-0006 (prolly tree)

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::file_table_workspace::FileTableWorkspace;
use crate::config::repository_layout::RepositoryLayout;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;

/// Direktori sementara yang unik untuk satu pengujian.
fn scratch(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    std::env::temp_dir().join(format!("verge-ws-{name}-{}-{nanos}", std::process::id()))
}

/// Bundle fixture: folder kerja, layout, workspace, dan tabel `users`.
struct Fixture {
    /// Folder kerja sementara yang dihapus setelah pengujian.
    dir: PathBuf,
    /// Layout repository di atas `dir`.
    layout: RepositoryLayout,
    /// Workspace yang diuji.
    workspace: FileTableWorkspace,
    /// Tabel yang selalu dipakai di semua pengujian.
    table: TableName,
}

/// Menyiapkan repository, workspace, dan tabel `users` di direktori sementara.
fn fixture(name: &str) -> Fixture {
    let dir = scratch(name);
    let layout = RepositoryLayout::under(&dir);
    let workspace = FileTableWorkspace::new(layout.clone());
    let table = TableName::parse("users").expect("nama tabel valid");
    Fixture {
        dir,
        layout,
        workspace,
        table,
    }
}

/// Membaca pointer data kerja tabel pada fixture.
fn staged(fix: &Fixture) -> Option<BlockId> {
    fix.workspace
        .staged(&fix.table)
        .expect("baca pointer data kerja")
}

/// Path berkas pointer data kerja tabel pada fixture.
fn pointer(fix: &Fixture) -> PathBuf {
    fix.layout.working_file(&fix.table)
}

/// Memastikan folder object store belum dibuat sama sekali.
fn assert_no_objects(fix: &Fixture) {
    assert!(
        !fix.layout.objects().exists(),
        "adapter hanya menunjuk akar; blok node ditulis use case"
    );
}

#[test]
fn stage_menunjuk_akar_tanpa_menulis_blok_sendiri() {
    let fix = fixture("stage");
    let root = Digest::of(b"node-akar");

    fix.workspace.stage(&fix.table, root).expect("stage tabel");

    assert_eq!(staged(&fix), Some(root));
    let raw = fs::read_to_string(pointer(&fix)).expect("baca pointer");
    assert_eq!(raw, format!("{}\n", root.to_hex()));
    assert_no_objects(&fix);
    drop(fs::remove_dir_all(&fix.dir));
}

#[test]
fn staging_ulang_dengan_akar_sama_tidak_menggandakan_pointer() {
    let fix = fixture("idempoten");
    let root = Digest::of(b"node-akar");

    fix.workspace.stage(&fix.table, root).expect("stage 1");
    let first = fs::read(pointer(&fix)).expect("baca pointer");
    fix.workspace.stage(&fix.table, root).expect("stage 2");

    assert_eq!(fs::read(pointer(&fix)).expect("baca pointer"), first);
    assert_eq!(staged(&fix), Some(root));
    drop(fs::remove_dir_all(&fix.dir));
}

#[test]
fn staging_akar_baru_menimpa_pointer_sebelumnya() {
    let fix = fixture("ganti");
    let first = Digest::of(b"akar-satu");
    let second = Digest::of(b"akar-dua");

    fix.workspace.stage(&fix.table, first).expect("stage 1");
    fix.workspace.stage(&fix.table, second).expect("stage 2");

    assert_eq!(staged(&fix), Some(second), "akar terakhir jadi data kerja");
    drop(fs::remove_dir_all(&fix.dir));
}

#[test]
fn tabel_belum_di_stage_menghasilkan_none() {
    let fix = fixture("belum");
    assert_eq!(staged(&fix), None);
    drop(fs::remove_dir_all(&fix.dir));
}

#[test]
fn pointer_kerja_rusak_ditolak_sebagai_malformed_pointer() {
    let fix = fixture("rusak");
    fs::create_dir_all(fix.layout.table_dir(&fix.table)).expect("buat direktori tabel");
    fs::write(pointer(&fix), "bukan-hex\n").expect("tulis pointer rusak");

    assert!(matches!(
        fix.workspace.staged(&fix.table),
        Err(VergeError::MalformedPointer(_))
    ));
    drop(fs::remove_dir_all(&fix.dir));
}
