//! File: `read_snapshot.rs`
//!
//! Deskripsi: Use case pembacaan isi tabel pada commit tertentu.
//! Layer: application/version-control/use-cases/reading
//! Tanggung jawab: Menyelesaikan referensi lalu menyusun ulang isi tabel.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/dtos/snapshot_content.rs`
//!   - `application/version-control/revision_resolver.rs`
//!   - `domain/commit/repositories/ports/commit_repository.rs`, `ref_pointer.rs`
//!   - `domain/storage/ports/block_store.rs`
//!   - `domain/tree/table_reader.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::application::version_control::dtos::snapshot_content::SnapshotContent;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::table_reader::read_table;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan pembacaan isi tabel pada satu revisi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadSnapshotInput {
    /// Tabel yang isinya dibaca.
    pub table: TableName,
    /// Revisi: `HEAD`, nama branch, nama tag, `CommitId` hex, atau waktu UTC.
    pub revision: String,
}

/// Membaca isi tabel pada revisi tertentu tanpa memulihkan working copy.
///
/// Args:
/// - input — tabel dan revisi yang diminta.
/// - refs — port pointer branch.
/// - tags — port pointer tag.
/// - commits — port penyimpanan objek commit.
/// - store — port object store.
///
/// Returns:
/// - Ok(SnapshotContent) — isi tabel pada revisi tersebut.
///
/// # Errors
///
/// Revisi yang tidak dikenal atau ambigu menghasilkan
/// [`InvalidRef`](VergeError::InvalidRef), revisi yang menunjuk commit tabel
/// lain menghasilkan
/// [`CommitBelongsToOtherTable`](VergeError::CommitBelongsToOtherTable), dan
/// [`BlockNotFound`](VergeError::BlockNotFound) bila blok tabel hilang.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::use_cases::read_snapshot::{
///     read_snapshot, ReadSnapshotInput,
/// };
/// use verge_core::domain::table::value_objects::table_name::TableName;
///
/// fn run(
///     refs: &dyn verge_core::RefPointer,
///     tags: &dyn verge_core::TagPointer,
///     commits: &dyn verge_core::CommitRepository,
///     store: &dyn verge_core::Store,
/// ) -> verge_core::Result<()> {
///     let input = ReadSnapshotInput {
///         table: TableName::parse("users")?,
///         revision: "HEAD".to_owned(),
///     };
///     let snapshot = read_snapshot(&input, refs, tags, commits, store)?;
///     println!("{} bytes pada commit {}", snapshot.bytes.len(), snapshot.commit);
///     Ok(())
/// }
/// ```
pub fn read_snapshot(
    input: &ReadSnapshotInput,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<SnapshotContent> {
    let id = resolve_revision(&input.revision, refs, tags, commits, Some(&input.table))?;
    let commit = commits.load(&id)?;
    if commit.table() != &input.table {
        return Err(VergeError::CommitBelongsToOtherTable {
            revision: input.revision.clone(),
            commit_table: commit.table().clone(),
            requested: input.table.clone(),
        });
    }
    let rows = read_table(commit.tree(), store)?;
    Ok(SnapshotContent {
        commit: id,
        bytes: rows.to_bytes(),
    })
}
