//! File: `read_snapshot.rs`
//!
//! Deskripsi: Use case pembacaan isi tabel pada commit tertentu.
//! Layer: application/version-control/use-cases/read-snapshot
//! Tanggung jawab: Menyelesaikan referensi lalu mengembalikan byte tabel.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/dtos/snapshot_content.rs`
//!   - `domain/commit/repositories/ports/commit_repository.rs`, `ref_pointer.rs`
//!   - `domain/storage/ports/block_store.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::application::version_control::dtos::snapshot_content::SnapshotContent;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest_text::parse_hex;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan pembacaan isi tabel pada satu revisi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadSnapshotInput {
    /// Tabel yang isinya dibaca.
    pub table: TableName,
    /// Revisi: `HEAD`, nama branch, atau `CommitId` hex.
    pub revision: String,
}

/// Membaca isi tabel pada revisi tertentu tanpa memulihkan working copy.
///
/// Args:
/// - input — tabel dan revisi yang diminta.
/// - refs — port pointer branch.
/// - commits — port penyimpanan objek commit.
/// - store — port object store.
///
/// Returns:
/// - Ok(SnapshotContent) — isi tabel pada revisi tersebut.
///
/// # Errors
///
/// Mengembalikan [`InvalidRef`](VergeError::InvalidRef) bila revisi tidak dapat
/// diselesaikan atau menunjuk commit untuk tabel lain, serta
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
///     commits: &dyn verge_core::CommitRepository,
///     store: &dyn verge_core::Store,
/// ) -> verge_core::Result<()> {
///     let input = ReadSnapshotInput {
///         table: TableName::parse("users")?,
///         revision: "HEAD".to_owned(),
///     };
///     let snapshot = read_snapshot(&input, refs, commits, store)?;
///     println!("{} bytes pada commit {}", snapshot.bytes.len(), snapshot.commit);
///     Ok(())
/// }
/// ```
pub fn read_snapshot(
    input: &ReadSnapshotInput,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<SnapshotContent> {
    let id = resolve_revision(input, refs)?;
    let commit = commits.load(&id)?;
    if commit.table() != &input.table {
        return Err(VergeError::InvalidRef(input.revision.clone()));
    }
    let bytes = store.get(&commit.tree())?;
    Ok(SnapshotContent { commit: id, bytes })
}

/// Menyelesaikan `HEAD`, nama branch, atau hex `CommitId` menjadi identifier.
fn resolve_revision(input: &ReadSnapshotInput, refs: &dyn RefPointer) -> Result<CommitId> {
    let revision = input.revision.trim();
    if revision == "HEAD" {
        let branch = refs.head_branch()?;
        return refs
            .resolve(&branch)?
            .ok_or_else(|| VergeError::HeadUnborn(branch.clone()));
    }
    if let Ok(id) = parse_hex(revision) {
        return Ok(id);
    }
    refs.resolve(revision)?
        .ok_or_else(|| VergeError::InvalidRef(input.revision.clone()))
}
