//! File: `commit_integrity.rs`
//!
//! Deskripsi: Test integrasi integritas objek commit di disk.
//! Layer: tests
//! Tanggung jawab: Membuktikan byte yang dimanipulasi ditolak saat dibaca.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `support/mod.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

mod support;

use std::fs;

use support::{cleanup, harness, workspace, write_source, Harness};

use verge_core::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use verge_core::application::version_control::use_cases::stage_table::{
    stage_table, StageTableInput,
};
use verge_core::config::storage_layout::StorageLayout;
use verge_core::domain::commit::repositories::ports::commit_repository::CommitRepository;
use verge_core::domain::commit::repositories::ports::ref_pointer::RefPointer;
use verge_core::domain::commit::value_objects::commit_id::CommitId;
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::table::file_system::file_table_source::FileTableSource;
use verge_core::VergeError;

/// Membuat satu commit tabel di repository hasil bootstrap.
fn seed_commit(workspace: &std::path::Path, harness: &Harness, contents: &[u8]) -> CommitId {
    let table = TableName::parse("users").expect("nama tabel valid");
    stage_table(
        &StageTableInput {
            table: table.clone(),
            source: write_source(workspace, "users.csv", contents),
        },
        &FileTableSource,
        &harness.workspace_port,
        &harness.store,
    )
    .expect("stage data");
    record_commit(
        &RecordCommitInput {
            table,
            message: "feat: seed".to_owned(),
            author: "ana".to_owned(),
        },
        &harness.workspace_port,
        &harness.commits,
        &harness.refs,
        1_000,
    )
    .expect("commit pertama")
    .id
}

#[test]
fn byte_commit_yang_diubah_di_disk_ditolak_saat_dibaca() {
    let workspace = workspace("manipulasi");
    let harness = harness(&workspace);
    let recorded = seed_commit(&workspace, &harness, b"id,name\n1,ana\n");

    let path = StorageLayout::block_path(&harness.layout.objects(), &recorded);
    fs::write(&path, b"diubah tangan").expect("tulis byte palsu");

    let error = harness
        .commits
        .load(&recorded)
        .expect_err("byte yang tidak kanonik harus ditolak");
    assert!(
        matches!(error, VergeError::MalformedCommit { .. }),
        "error yang diharapkan: malformed commit, didapat: {error:?}"
    );

    cleanup(&workspace);
}

#[test]
fn pointer_branch_yang_rusak_ditolak_saat_dibaca() {
    let workspace = workspace("pointer");
    let harness = harness(&workspace);
    let branch_file = harness.layout.heads().join("main");
    fs::write(&branch_file, "bukan-hex\n").expect("tulis pointer palsu");

    let error = harness
        .refs
        .resolve("main")
        .expect_err("pointer rusak harus ditolak");
    assert!(
        matches!(error, VergeError::MalformedPointer(_)),
        "error yang diharapkan: malformed pointer, didapat: {error:?}"
    );

    cleanup(&workspace);
}
