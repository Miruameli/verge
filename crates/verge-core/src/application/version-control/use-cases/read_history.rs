//! File: `read_history.rs`
//!
//! Deskripsi: Use case pembacaan riwayat commit sebuah tabel.
//! Layer: application/version-control/use-cases/read-history
//! Tanggung jawab: Menelusuri first-parent dan menyaring commit per tabel.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/dtos/history_entry.rs`
//!   - `domain/commit/repositories/ports/commit_repository.rs`, `ref_pointer.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::application::version_control::dtos::history_entry::HistoryEntry;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan pembacaan riwayat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadHistoryInput {
    /// Tabel yang riwayatnya dibaca.
    pub table: TableName,
    /// Jumlah entri maksimum.
    pub limit: usize,
}

/// Membaca riwayat commit tabel pada branch aktif, terbaru lebih dulu.
///
/// Args:
/// - input — tabel dan batas jumlah entri.
/// - refs — port pointer branch.
/// - commits — port penyimpanan objek commit.
///
/// Returns:
/// - Ok(Vec<HistoryEntry>) — entri terbaru lebih dulu; kosong bila branch belum
///   punya commit untuk tabel tersebut.
///
/// # Errors
///
/// Mengembalikan [`HeadUnborn`](VergeError::HeadUnborn) bila branch aktif belum
/// memiliki commit, [`CommitNotFound`](VergeError::CommitNotFound) bila objek
/// commit hilang dari storage, serta error I/O dari port.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::use_cases::read_history::{
///     read_history, ReadHistoryInput,
/// };
/// use verge_core::domain::table::value_objects::table_name::TableName;
///
/// fn run(
///     refs: &dyn verge_core::RefPointer,
///     commits: &dyn verge_core::CommitRepository,
/// ) -> verge_core::Result<Vec<verge_core::application::version_control::dtos::history_entry::HistoryEntry>>
/// {
///     let input = ReadHistoryInput {
///         table: TableName::parse("users")?,
///         limit: 20,
///     };
///     read_history(&input, refs, commits)
/// }
/// ```
pub fn read_history(
    input: &ReadHistoryInput,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
) -> Result<Vec<HistoryEntry>> {
    let branch = refs.head_branch()?;
    let mut cursor = refs
        .resolve(&branch)?
        .ok_or(VergeError::HeadUnborn(branch.clone()))?;
    let mut entries = Vec::new();
    while entries.len() < input.limit {
        let commit = commits.load(&cursor)?;
        if commit.table() == &input.table {
            entries.push(HistoryEntry {
                id: commit.id(),
                author: commit.author().to_owned(),
                summary: commit.summary().to_owned(),
                timestamp_unix_ms: commit.timestamp_unix_ms(),
            });
        }
        match commit.parents().first().copied() {
            Some(parent) => cursor = parent,
            None => break,
        }
    }
    Ok(entries)
}
