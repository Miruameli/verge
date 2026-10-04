//! File: `merge_reader.rs`
//!
//! Deskripsi: Pembacaan ketiga versi tabel untuk merge.
//! Layer: application/version-control/use-cases/merging
//! Tanggung jawab: Memuat base, ours, dan theirs beserta commit-nya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/**`, `domain/tree/**`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::table_reader::read_table;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Tiga versi tabel yang menjadi bahan merge.
///
/// Immutability: penuh.
pub struct MergeSides {
    /// Baris pada merge base.
    pub base: TableRows,
    /// Baris pada branch aktif.
    pub ours: TableRows,
    /// Baris pada branch yang digabung.
    pub theirs: TableRows,
    /// Commit merge base.
    pub base_commit: Commit,
    /// Commit branch aktif.
    pub ours_commit: Commit,
    /// Commit branch yang digabung.
    pub theirs_commit: Commit,
}

/// Membaca ketiga versi tabel dan memastikan ketiganya untuk tabel yang sama.
///
/// # Errors
///
/// Mengembalikan [`InvalidRef`](VergeError::InvalidRef) bila ada commit yang
/// bukan untuk `table` dan error dari port pembacaan commit maupun tree.
pub fn read_sides(
    base: CommitId,
    ours: CommitId,
    theirs: CommitId,
    commits: &dyn CommitRepository,
    store: &dyn Store,
    table: &TableName,
) -> Result<MergeSides> {
    let base_commit = commit_of_table(base, commits, table)?;
    let ours_commit = commit_of_table(ours, commits, table)?;
    let theirs_commit = commit_of_table(theirs, commits, table)?;
    Ok(MergeSides {
        base: read_table(base_commit.tree(), store)?,
        ours: read_table(ours_commit.tree(), store)?,
        theirs: read_table(theirs_commit.tree(), store)?,
        base_commit,
        ours_commit,
        theirs_commit,
    })
}

/// Mengembalikan commit `id` setelah memastikan commit itu untuk `table`.
///
/// # Errors
///
/// Mengembalikan
/// [`CommitBelongsToOtherTable`](VergeError::CommitBelongsToOtherTable) bila
/// commit milik tabel lain dan error dari port bila commit tidak dapat dibaca.
fn commit_of_table(
    id: CommitId,
    commits: &dyn CommitRepository,
    table: &TableName,
) -> Result<Commit> {
    let commit = commits.load(&id)?;
    if commit.table() != table {
        return Err(VergeError::CommitBelongsToOtherTable {
            revision: id.to_hex(),
            commit_table: commit.table().clone(),
            requested: table.clone(),
        });
    }
    Ok(commit)
}
