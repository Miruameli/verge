//! File: `tag_pointer.rs`
//!
//! Deskripsi: Port pointer tag yang menunjuk satu commit.
//! Layer: domain/commit/repositories/ports
//! Tanggung jawab: Menyimpan nama audit pada satu commit secara immutable.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/commit_id.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)
//!
//! ALTERNATIF: tag sebagai field pada objek commit; ditolak karena物件 yang
//! berubah setelah commit undermines content-addressing: identifier commit
//! harus tetap berarti byte yang sama selamanya.

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::shared::kernel::result::Result;

/// Pointer nama audit yang menunjuk tepat satu commit dan tidak pernah digeser.
///
/// Berbeda dari branch, tag tidak dapat diubah setelah dibuat. Laporan yang
/// menyebut `q2-report` harus tetap menunjuk keadaan yang sama selamanya,
/// sehingga `create` menolak nama yang sudah ada alih-alih menimpanya.
pub trait TagPointer {
    /// Menunjuk tag `name` ke commit `id`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`TagAlreadyExists`](crate::VergeError::TagAlreadyExists)
    /// bila nama sudah dipakai — tag tidak pernah ditimpa — dan error I/O bila
    /// pointer tidak dapat ditulis.
    fn create(&self, name: &str, id: CommitId) -> Result<()>;

    /// Mengembalikan commit yang ditunjuk tag `name`.
    ///
    /// Returns:
    /// - Ok(None) — tag belum ada.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila pointer tidak dapat dibaca.
    fn resolve(&self, name: &str) -> Result<Option<CommitId>>;

    /// Mengembalikan seluruh nama tag yang punya pointer, terurut menaik.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori pointer tidak dapat dibaca.
    fn tags(&self) -> Result<Vec<String>>;

    /// Menghapus pointer tag `name`.
    ///
    /// Blok data tidak ikut terhapus: commit tetap dapat dibaca lewat commit-id,
    /// jadi menghapus tag tidak menghapus data.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`UnknownTag`](crate::VergeError::UnknownTag) bila tag
    /// belum ada dan error I/O bila pointer gagal dihapus.
    fn delete(&self, name: &str) -> Result<()>;
}

impl<T: TagPointer + ?Sized> TagPointer for &T {
    fn create(&self, name: &str, id: CommitId) -> Result<()> {
        (**self).create(name, id)
    }

    fn resolve(&self, name: &str) -> Result<Option<CommitId>> {
        (**self).resolve(name)
    }

    fn tags(&self) -> Result<Vec<String>> {
        (**self).tags()
    }

    fn delete(&self, name: &str) -> Result<()> {
        (**self).delete(name)
    }
}
