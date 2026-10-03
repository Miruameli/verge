//! File: `delete_branch.rs`
//!
//! Deskripsi: Use case penghapusan pointer branch.
//! Layer: application/version-control/use-cases/refs/branching
//! Tanggung jawab: Menghapus satu pointer dan melaporkan commit yang tersisa.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/ref_pointer.rs`
//!   - `domain/commit/value-objects/commit_id.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan penghapusan branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteBranchInput {
    /// Branch yang dihapus.
    pub name: String,
}

/// Hasil penghapusan branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeletedBranch {
    /// Branch yang dihapus.
    pub name: String,
    /// Commit terakhir branch sebelum pointer dihapus.
    ///
    /// Disimpan agar pengguna masih bisa-reaching commit tersebut lewat
    /// `verge show <commit>` walau nama branch sudah tidak ada.
    pub was_at: Option<CommitId>,
}

/// Menghapus pointer branch tanpa menghapus blok data.
///
/// KONTEKS: commit branch yang dihapus tetap ada di store karena blok bersifat
/// immutable dan content-addressed; KENAPA: menghapus blok ikutnya berarti
/// audit trail hilang dan commit yang sudah di-share tidak dapat dibaca lagi.
///
/// Args:
/// - input — branch yang dihapus.
/// - refs — port pointer branch.
///
/// Returns:
/// - Ok(DeletedBranch) — branch yang dihapus beserta ujung commit-nya.
///
/// # Errors
///
/// Mengembalikan [`BranchInUse`](VergeError::BranchInUse) bila branch adalah
/// branch aktif, [`UnknownBranch`](VergeError::UnknownBranch) bila branch tidak
/// ada, [`InvalidName`](VergeError::InvalidName) untuk nama tidak aman, dan
/// error I/O bila pointer gagal dihapus.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::use_cases::refs::branching::delete_branch::{
///     delete_branch, DeleteBranchInput,
/// };
/// use verge_core::config::repository_layout::RepositoryLayout;
/// use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;
///
/// let refs = FileRefPointer::new(RepositoryLayout::under("/tmp/verge-doc"));
/// let deleted = delete_branch(
///     &DeleteBranchInput { name: "eksperimen".to_owned() },
///     &refs,
/// )
/// .expect("branch dihapus");
/// assert_eq!(deleted.name, "eksperimen");
/// ```
pub fn delete_branch(input: &DeleteBranchInput, refs: &dyn RefPointer) -> Result<DeletedBranch> {
    if refs.head_branch().map_or(false, |head| head == input.name) {
        return Err(VergeError::BranchInUse(input.name.clone()));
    }
    let was_at = refs.resolve(&input.name)?;
    if was_at.is_none() {
        return Err(VergeError::UnknownBranch(input.name.clone()));
    }
    refs.delete(&input.name)?;
    Ok(DeletedBranch {
        name: input.name.clone(),
        was_at,
    })
}
