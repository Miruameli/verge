//! File: `create_branch.rs`
//!
//! Deskripsi: Use case pembuatan branch baru.
//! Layer: application/version-control/use-cases/refs/branching
//! Tanggung jawab: Menulis satu pointer branch tanpa menyalin data.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/ref_pointer.rs`
//!   - `domain/commit/value-objects/branch_name_policy.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::branch_name_policy::validate_name;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan pembuatan branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateBranchInput {
    /// Nama branch yang akan dibuat.
    pub name: String,
}

/// Hasil pembuatan branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedBranch {
    /// Nama branch yang dibuat.
    pub name: String,
    /// Commit tempat branch baru bermula.
    pub at: CommitId,
}

/// Membuat branch yang menunjuk commit terakhir branch aktif.
///
/// KONTEKS: branch baru lahir di commit yang sama dengan branch aktif agar
/// eksperimen dimulai dari data terkini; KENAPA: menyalin blok tabel akan
/// melanggar janji branching O(1), sedangkan menulis satu pointer kecil tidak.
///
/// Args:
/// - input — nama branch baru.
/// - refs — port pointer branch.
///
/// Returns:
/// - Ok(CreatedBranch) — branch baru beserta commit awal.
///
/// # Errors
///
/// Mengembalikan [`InvalidName`](VergeError::InvalidName) bila nama tidak aman,
/// [`BranchAlreadyExists`](VergeError::BranchAlreadyExists) bila nama sudah
/// dipakai, [`HeadUnborn`](VergeError::HeadUnborn) bila branch aktif belum punya
/// commit pertama, dan error I/O dari port.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::use_cases::refs::branching::create_branch::{
///     create_branch, CreateBranchInput,
/// };
/// use verge_core::config::repository_layout::RepositoryLayout;
/// use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;
///
/// let refs = FileRefPointer::new(RepositoryLayout::under("/tmp/verge-doc"));
/// let created = create_branch(
///     &CreateBranchInput { name: "eksperimen".to_owned() },
///     &refs,
/// )
/// .expect("branch dibuat");
/// assert_eq!(created.name, "eksperimen");
/// ```
pub fn create_branch(input: &CreateBranchInput, refs: &dyn RefPointer) -> Result<CreatedBranch> {
    validate_name(&input.name)?;
    if refs.resolve(&input.name)?.is_some() {
        return Err(VergeError::BranchAlreadyExists(input.name.clone()));
    }
    let current = refs.head_branch()?;
    let at = refs
        .resolve(&current)?
        .ok_or_else(|| VergeError::HeadUnborn(current.clone()))?;
    refs.advance(&input.name, at)?;
    Ok(CreatedBranch {
        name: input.name.clone(),
        at,
    })
}
