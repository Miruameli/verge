//! File: `switch_branch.rs`
//!
//! Deskripsi: Use case perpindahan branch aktif.
//! Layer: application/version-control/use-cases/branching
//! Tanggung jawab: Mengalihkan `HEAD` tanpa mengubah data.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/ref_pointer.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan perpindahan branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchBranchInput {
    /// Branch yang menjadi branch aktif.
    pub name: String,
}

/// Hasil perpindahan branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchedBranch {
    /// Branch yang sebelumnya aktif.
    pub previous: String,
    /// Branch yang sekarang aktif.
    pub current: String,
}

/// Memindahkan branch aktif ke `input.name`.
///
/// KONTEKS: data kerja tabel sengaja tidak ikut dipindahkan; KENAPA: isi data
/// kerja menunjuk tree terakhir commit aktif, jadi ikut memindahkannya berarti
/// menulis ulang blok yang justru seharusnya dipakai ulang.
///
/// Args:
/// - input — branch tujuan.
/// - refs — port pointer branch.
///
/// Returns:
/// - Ok(SwitchedBranch) — branch sebelum dan sesudah perpindahan.
///
/// # Errors
///
/// Mengembalikan [`UnknownBranch`](VergeError::UnknownBranch) bila branch tujuan
/// belum ada, [`InvalidName`](VergeError::InvalidName) untuk nama tidak aman, dan
/// error I/O bila `HEAD` tidak dapat ditulis.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::use_cases::branching::switch_branch::{
///     switch_branch, SwitchBranchInput,
/// };
/// use verge_core::config::repository_layout::RepositoryLayout;
/// use verge_core::infrastructure::commit::file_system::file_ref_pointer::FileRefPointer;
///
/// let refs = FileRefPointer::new(RepositoryLayout::under("/tmp/verge-doc"));
/// let switched = switch_branch(
///     &SwitchBranchInput { name: "eksperimen".to_owned() },
///     &refs,
/// )
/// .expect("branch dialihkan");
/// assert_eq!(switched.current, "eksperimen");
/// ```
pub fn switch_branch(input: &SwitchBranchInput, refs: &dyn RefPointer) -> Result<SwitchedBranch> {
    let previous = refs.head_branch()?;
    if previous == input.name {
        return Ok(SwitchedBranch {
            previous,
            current: input.name.clone(),
        });
    }
    if refs.resolve(&input.name)?.is_none() {
        return Err(VergeError::UnknownBranch(input.name.clone()));
    }
    refs.switch(&input.name)?;
    Ok(SwitchedBranch {
        previous,
        current: input.name.clone(),
    })
}
