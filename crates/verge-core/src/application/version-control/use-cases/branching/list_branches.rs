//! File: `list_branches.rs`
//!
//! Deskripsi: Use case pembacaan daftar branch.
//! Layer: application/version-control/use-cases/branching
//! Tanggung jawab: Mengumpulkan seluruh branch beserta ujung commit-nya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/dtos/branch_list.rs`
//!   - `domain/commit/repositories/ports/ref_pointer.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::application::version_control::dtos::branching::branch_list::{BranchInfo, BranchList};
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::shared::kernel::result::Result;

/// Membaca seluruh branch beserta commit terakhirnya.
///
/// KONTEKS: branch tanpa pointer tidak dapat dibaca datanya, jadi daftar hanya
/// memuat branch yang benar-benar ada; KENAPA: menampilkan branch hantu membuat
/// pengguna menunggu commit yang tidak pernah terjadi.
///
/// Args:
/// - refs — port pointer branch.
///
/// Returns:
/// - Ok(BranchList) — daftar branch terurut menaik dengan branch aktif ditandai.
///
/// # Errors
///
/// Mengembalikan error I/O bila pointer tidak dapat dibaca dan
/// [`InvalidName`](crate::VergeError::InvalidName) bila ada entri pointer yang
/// bukan nama branch valid.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::use_cases::branching::list_branches::list_branches;
/// use verge_core::config::repository_layout::RepositoryLayout;
/// use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;
///
/// let refs = FileRefPointer::new(RepositoryLayout::under("/tmp/verge-doc"));
/// let list = list_branches(&refs).expect("branch terbaca");
/// assert!(list.branches.iter().any(|branch| branch.name == list.current));
/// ```
pub fn list_branches(refs: &dyn RefPointer) -> Result<BranchList> {
    let current = refs.head_branch()?;
    let mut branches = Vec::new();
    for name in refs.branches()? {
        let head = refs.resolve(&name)?;
        branches.push(BranchInfo {
            current: name == current,
            name,
            head,
        });
    }
    Ok(BranchList { current, branches })
}
