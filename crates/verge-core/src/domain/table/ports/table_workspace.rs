//! File: `table_workspace.rs`
//!
//! Deskripsi: Port data kerja tabel (working set).
//! Layer: domain/table/ports
//! Tanggung jawab: Menyimpan isi tabel terakhir sebelum di-commit sebagai blok.
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
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::kernel::result::Result;

/// Port yang memegang isi tabel terakhir sebelum di-commit.
///
/// Isi tabel tidak pernah disimpan sebagai berkas kerja biasa: data ditulis
/// sebagai blok immutable dan hanya digest-nya yang menjadi pointer. Dengan begitu
/// data yang sama tidak pernah ada di dua tempat.
pub trait TableWorkspace {
    /// Menulis `data` sebagai blok dan menunjukkannya sebagai data kerja.
    ///
    /// Args:
    /// - name — tabel tujuan.
    /// - data — isi tabel penuh.
    ///
    /// Returns:
    /// - Ok(BlockId) — blok yang menjadi data kerja tabel.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila blok atau pointer tidak dapat ditulis.
    fn stage(&self, name: &TableName, data: &[u8]) -> Result<BlockId>;

    /// Mengembalikan blok data kerja tabel.
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
    fn stage(&self, name: &TableName, data: &[u8]) -> Result<BlockId> {
        (**self).stage(name, data)
    }

    fn staged(&self, name: &TableName) -> Result<Option<BlockId>> {
        (**self).staged(name)
    }
}
