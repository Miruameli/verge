//! File: `flow.rs`
//!
//! Deskripsi: Langkah langkah alur tabel untuk test integrasi.
//! Layer: tests
//! Tanggung jawab: Menyatukan stage, commit, pembacaan snapshot, dan diff.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `mod.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0005 (blok content-addressed)
//!   - ADR-0006 (prolly tree)

use std::path::Path;

use super::write_source;
use super::Harness;

use verge_core::application::version_control::dtos::table_diff_report::TableDiffReport;
use verge_core::application::version_control::use_cases::diff_tables::{
    diff_tables, DiffTablesInput,
};
use verge_core::application::version_control::use_cases::read_snapshot::{
    read_snapshot, ReadSnapshotInput,
};
use verge_core::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use verge_core::application::version_control::use_cases::stage_table::{
    stage_table, StageTableInput,
};
use verge_core::domain::commit::value_objects::commit_id::CommitId;
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::table::file_system::file_table_source::FileTableSource;
use verge_core::Result;

/// Nama tabel yang dipakai seluruh test integrasi.
pub fn users() -> TableName {
    TableName::parse("users").expect("nama tabel test valid")
}

/// Menyimpan `contents` sebagai data kerja lalu membuat commit atas tabel `users`.
pub fn import_and_commit(
    harness: &Harness,
    source: &Path,
    message: &str,
    author: &str,
    time: i64,
) -> CommitId {
    stage_table(
        &StageTableInput {
            table: users(),
            source: source.to_path_buf(),
        },
        &FileTableSource,
        &harness.workspace_port,
        &harness.store,
    )
    .expect("stage data");
    record_commit(
        &RecordCommitInput {
            table: users(),
            message: message.to_owned(),
            author: author.to_owned(),
        },
        &harness.workspace_port,
        &harness.commits,
        &harness.refs,
        time,
    )
    .expect("commit berhasil")
    .id
}

/// Membaca isi tabel `users` pada revisi tertentu.
pub fn snapshot(harness: &Harness, revision: &str) -> Vec<u8> {
    read_snapshot(
        &ReadSnapshotInput {
            table: users(),
            revision: revision.to_owned(),
        },
        &harness.refs,
        &harness.tags,
        &harness.commits,
        &harness.store,
    )
    .expect("snapshot terbaca")
    .bytes
}

/// Membandingkan dua revisi tabel tanpa memanik error use case.
pub fn try_diff(
    harness: &Harness,
    table: &TableName,
    from: &str,
    to: &str,
) -> Result<TableDiffReport> {
    diff_tables(
        &DiffTablesInput {
            table: table.clone(),
            from: from.to_owned(),
            to: to.to_owned(),
        },
        &harness.refs,
        &harness.tags,
        &harness.commits,
        &harness.store,
    )
}

/// Membandingkan isi tabel `users` antara dua revisi; panic bila use case gagal.
pub fn diff(harness: &Harness, from: &str, to: &str) -> TableDiffReport {
    try_diff(harness, &users(), from, to).expect("diff terbaca")
}

/// Menulis sumber data lalu membuat commit dalam satu langkah.
pub fn commit_bytes(
    harness: &Harness,
    workspace: &Path,
    contents: &[u8],
    message: &str,
) -> CommitId {
    let source = write_source(workspace, "users.csv", contents);
    import_and_commit(harness, &source, message, "ana", 1_000)
}
