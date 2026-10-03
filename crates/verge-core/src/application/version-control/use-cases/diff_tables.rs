//! File: `diff_tables.rs`
//!
//! Deskripsi: Use case perbandingan isi tabel pada dua revisi.
//! Layer: application/version-control/use-cases
//! Tanggung jawab: Membaca dua snapshot lalu melaporkan perubahan per baris.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/dtos/table_diff_report.rs`
//!   - `domain/commit/repositories/ports/commit_repository.rs`, `ref_pointer.rs`
//!   - `domain/storage/ports/block_store.rs`
//!   - `domain/tree/diff/table_diff.rs`, `table_reader.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::application::version_control::dtos::snapshot_content::SnapshotContent;
use crate::application::version_control::dtos::table_diff_report::TableDiffReport;
use crate::application::version_control::use_cases::read_snapshot::{
    read_snapshot, ReadSnapshotInput,
};
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::diff::table_diff::TableDiff;
use crate::shared::kernel::result::Result;

/// Masukan perbandingan tabel pada dua revisi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffTablesInput {
    /// Tabel yang dibandingkan.
    pub table: TableName,
    /// Revisi tabel lama.
    pub from: String,
    /// Revisi tabel baru.
    pub to: String,
}

/// Membandingkan isi tabel pada dua revisi dan melaporkan perubahan per baris.
///
/// KONTEKS: kedua revisi diselesaikan dengan aturan yang sama seperti
/// `read_snapshot`; KENAPA: `from` dan `to` sudah terpisah karena pemecahan
/// `FROM..TO` milik antarmuka, sehingga lapisan ini tidak pernah tahu bentuk
/// argumen baris perintah.
///
/// Args:
/// - input — tabel dan kedua revisi pembanding.
/// - refs — port pointer branch.
/// - commits — port penyimpanan objek commit.
/// - store — port object store.
///
/// Returns:
/// - Ok(TableDiffReport) — perubahan baris dari revisi lama ke revisi baru.
///
/// # Errors
///
/// Mengembalikan [`InvalidRef`](VergeError::InvalidRef) bila salah satu revisi
/// tidak dapat diselesaikan atau menunjuk commit untuk tabel lain,
/// [`HeadUnborn`](VergeError::HeadUnborn) bila `HEAD` menunjuk branch kosong,
/// serta error dari port pembacaan commit dan tree.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::use_cases::diff_tables::{
///     diff_tables, DiffTablesInput,
/// };
/// use verge_core::domain::table::value_objects::table_name::TableName;
///
/// fn run(
///     refs: &dyn verge_core::RefPointer,
///     tags: &dyn verge_core::TagPointer,
///     commits: &dyn verge_core::CommitRepository,
///     store: &dyn verge_core::Store,
/// ) -> verge_core::Result<usize> {
///     let input = DiffTablesInput {
///         table: TableName::parse("users")?,
///         from: "HEAD~1".to_owned(),
///         to: "HEAD".to_owned(),
///     };
///     Ok(diff_tables(&input, refs, tags, commits, store)?.len())
/// }
/// ```
pub fn diff_tables(
    input: &DiffTablesInput,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<TableDiffReport> {
    let before = read_table_at(&input.table, &input.from, refs, tags, commits, store)?;
    let after = read_table_at(&input.table, &input.to, refs, tags, commits, store)?;
    let diff = TableDiff::between(&before.bytes, &after.bytes);
    Ok(TableDiffReport {
        from: before.commit,
        to: after.commit,
        changes: diff.changes,
    })
}

/// Membaca isi tabel pada satu revisi dengan aturan resolusi yang sama dengan
/// `read_snapshot`.
fn read_table_at(
    table: &TableName,
    revision: &str,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<SnapshotContent> {
    let input = ReadSnapshotInput {
        table: table.clone(),
        revision: revision.to_owned(),
    };
    read_snapshot(&input, refs, tags, commits, store)
}
