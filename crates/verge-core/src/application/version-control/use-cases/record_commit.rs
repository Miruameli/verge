//! File: `record_commit.rs`
//!
//! Deskripsi: Use case pembuatan commit tabel.
//! Layer: application/version-control/use-cases/record-commit
//! Tanggung jawab: Mengubah data kerja menjadi commit dan menggerakkan branch.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/dtos/recorded_commit.rs`
//!   - `domain/commit/repositories/ports/commit_repository.rs`, `ref_pointer.rs`
//!   - `domain/table/ports/table_workspace.rs`
//!   - `record_commit_validation.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

// Validasi metadata dipisah ke modul anak agar berkas ini fokus pada orkestrasi.
#[path = "record_commit_validation.rs"]
mod validation;

use crate::application::version_control::dtos::recorded_commit::RecordedCommit;
use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan pembuatan commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordCommitInput {
    /// Tabel yang di-versioning.
    pub table: TableName,
    /// Pesan commit.
    pub message: String,
    /// Penulis commit.
    pub author: String,
}

/// Membuat commit dari data kerja tabel lalu menggerakkan branch aktif.
///
/// Args:
/// - input — tabel, pesan, dan penulis commit.
/// - workspace — port data kerja tabel.
/// - commits — port penyimpanan objek commit.
/// - refs — port pointer branch.
/// - `timestamp_unix_ms` — waktu commit dari sumber jam di luar domain.
///
/// Returns:
/// - Ok(RecordedCommit) — commit yang tercatat beserta branch yang bergerak.
///
/// # Errors
///
/// Mengembalikan [`InvalidCommitField`](VergeError::InvalidCommitField) untuk
/// penulis atau pesan yang tidak valid,
/// [`TableNotStaged`](VergeError::TableNotStaged) bila tabel belum punya data
/// kerja, [`NothingToCommit`](VergeError::NothingToCommit) bila isi tabel sama
/// dengan commit terakhir, serta error I/O dari port.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::dtos::recorded_commit::RecordedCommit;
/// use verge_core::application::version_control::use_cases::record_commit::{
///     record_commit, RecordCommitInput,
/// };
/// use verge_core::domain::table::value_objects::table_name::TableName;
///
/// fn run(
///     workspace: &dyn verge_core::TableWorkspace,
///     commits: &dyn verge_core::CommitRepository,
///     refs: &dyn verge_core::RefPointer,
/// ) -> verge_core::Result<RecordedCommit> {
///     let input = RecordCommitInput {
///         table: TableName::parse("users")?,
///         message: "feat: seed users".to_owned(),
///         author: "ana".to_owned(),
///     };
///     record_commit(&input, workspace, commits, refs, 1_700_000_000_000)
/// }
/// ```
pub fn record_commit(
    input: &RecordCommitInput,
    workspace: &dyn TableWorkspace,
    commits: &dyn CommitRepository,
    refs: &dyn RefPointer,
    timestamp_unix_ms: i64,
) -> Result<RecordedCommit> {
    validation::validate_author(&input.author)?;
    validation::validate_message(&input.message)?;
    let tree = workspace
        .staged(&input.table)?
        .ok_or_else(|| VergeError::TableNotStaged(input.table.to_string()))?;
    let branch = refs.head_branch()?;
    let parent = refs.resolve(&branch)?;
    if let Some(parent_id) = parent {
        let previous = commits.load(&parent_id)?;
        if previous.tree() == tree {
            return Err(VergeError::NothingToCommit(input.table.to_string()));
        }
    }
    let parents = parent.into_iter().collect::<Vec<CommitId>>();
    let commit = Commit::new(
        parents,
        tree,
        &input.table,
        input.author.clone(),
        input.message.clone(),
        timestamp_unix_ms,
    );
    let outcome = commits.save(&commit)?;
    refs.advance(&branch, commit.id())?;
    Ok(RecordedCommit {
        id: commit.id(),
        branch,
        tree,
        created: outcome.inserted,
    })
}
