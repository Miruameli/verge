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

    /// Mengembalikan seluruh nama branch yang punya pointer.
    ///
    /// Returns:
    /// - Ok(Vec<String>) — nama branch terurut menaik agar keluaran stabil.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori pointer tidak dapat dibaca dan
    /// [`MalformedPointer`](crate::VergeError::MalformedPointer) bila ada
    /// entri yang bukan nama branch valid.
    fn branches(&self) -> Result<Vec<String>>;

    /// Mengalihkan `HEAD` ke `branch` yang sudah punya pointer.
    ///
    /// KONTEKS: switch tidak menulis ulang isi tabel apa pun; hanya berkas
    /// `HEAD` yang berubah sehingga tetap O(1).
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) untuk
    /// nama tidak valid, [`UnknownBranch`](crate::VergeError::UnknownBranch)
    /// bila branch belum ada, dan error I/O bila `HEAD` tidak dapat ditulis.
    fn switch(&self, branch: &str) -> Result<()>;

    /// Menghapus pointer `branch`.
    ///
    /// Blok data tidak ikut terhapus: commit tetap dapat dibaca lewat branch
    /// lain atau lewat identifier, jadi penghapusan branch tidak menghapus data.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) untuk
    /// nama tidak valid, [`UnknownBranch`](crate::VergeError::UnknownBranch)
    /// bila branch belum ada, [`BranchInUse`](crate::VergeError::BranchInUse)
    /// bila branch adalah branch aktif, dan error I/O bila pointer gagal dihapus.
    fn delete(&self, branch: &str) -> Result<()>;
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

    fn branches(&self) -> Result<Vec<String>> {
        (**self).branches()
    }

    fn switch(&self, branch: &str) -> Result<()> {
        (**self).switch(branch)
    }

    fn delete(&self, branch: &str) -> Result<()> {
        (**self).delete(branch)
    }
}
