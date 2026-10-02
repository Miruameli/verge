//! File: `table_workspace.rs`
//!
//! Deskripsi: Port data kerja tabel (working set).
//! Layer: domain/table/ports
//! Tanggung jawab: Menunjuk node akar tree tabel terakhir sebelum di-commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/table/value-objects/table_name.rs`
//!   - `domain/storage/value-objects/block_id.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::kernel::result::Result;

/// Port yang memegang isi tabel terakhir sebelum di-commit.
///
/// Isi tabel tidak pernah disimpan sebagai berkas kerja biasa: isinya ditulis
/// sebagai blok-blok immutable tree dan hanya digest akarnya yang menjadi
/// pointer. Dengan begitu baris yang tidak berubah tidak ditulis ulang.
pub trait TableWorkspace {
    /// Menunjuk `root` tree tabel sebagai isi data kerja.
    ///
    /// Args:
    /// - name — tabel tujuan.
    /// - root — identifier node akar tree tabel.
    ///
    /// Returns:
    /// - Ok(()) — pointer data kerja tabel menunjuk `root`.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila pointer tidak dapat ditulis.
    fn stage(&self, name: &TableName, root: BlockId) -> Result<()>;

    /// Mengembalikan node akar tree yang menjadi data kerja tabel.
    ///
    /// Returns:
    /// - Ok(None) — tabel belum pernah di-stage.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`MalformedPointer`](crate::VergeError::MalformedPointer)
    /// bila pointer rusak dan error I/O bila berkas tidak dapat dibaca.
    fn staged(&self, name: &TableName) -> Result<Option<BlockId>>;
}

impl<T: TableWorkspace + ?Sized> TableWorkspace for &T {
    fn stage(&self, name: &TableName, root: BlockId) -> Result<()> {
        (**self).stage(name, root)
    }

    fn staged(&self, name: &TableName) -> Result<Option<BlockId>> {
        (**self).staged(name)
    }
}
