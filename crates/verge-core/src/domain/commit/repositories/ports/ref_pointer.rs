//! File: `ref_pointer.rs`
//!
//! Deskripsi: Port pointer branch dan `HEAD`.
//! Layer: domain/commit/repositories/ports
//! Tanggung jawab: Menyimpan satu commit per branch tanpa menyalin data.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/commit_graph.rs`
//!   - `domain/storage/ports/metadata_writer.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::shared::kernel::result::Result;

/// Pointer bergerak yang menunjuk commit terakhir sebuah branch.
///
/// Kontrak ini adalah alasan branching Verge tetap O(1): operasi membuat branch
/// hanya menulis satu pointer, tanpa menyalin satu blok pun.
pub trait RefPointer {
    /// Mengembalikan branch aktif yang ditunjuk `HEAD`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`MalformedPointer`](crate::VergeError::MalformedPointer)
    /// bila isi `HEAD` tidak mengikuti format `ref: refs/heads/<nama>`.
    fn head_branch(&self) -> Result<String>;

    /// Mengembalikan commit terakhir `branch`.
    ///
    /// Returns:
    /// - Ok(None) — branch belum punya commit.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) untuk nama
    /// tidak valid dan [`MalformedPointer`](crate::VergeError::MalformedPointer)
    /// bila isi pointer bukan digest yang valid.
    fn resolve(&self, branch: &str) -> Result<Option<CommitId>>;

    /// Memindahkan `branch` ke commit `id`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) untuk nama
    /// tidak valid dan error I/O bila pointer tidak dapat ditulis.
    fn advance(&self, branch: &str, id: CommitId) -> Result<()>;
}

impl<T: RefPointer + ?Sized> RefPointer for &T {
    fn head_branch(&self) -> Result<String> {
        (**self).head_branch()
    }

    fn resolve(&self, branch: &str) -> Result<Option<CommitId>> {
        (**self).resolve(branch)
    }

    fn advance(&self, branch: &str, id: CommitId) -> Result<()> {
        (**self).advance(branch, id)
    }
}
