//! File: `block_store_factory.rs`
//!
//! Deskripsi: Port pembuatan instance `Store`.
//! Layer: domain/storage/ports
//! Tanggung jawab: Memisahkan use case dari pemilihan backend penyimpanan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/storage/ports/block_store.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::path::Path;

use crate::domain::storage::ports::block_store::Store;
use crate::shared::kernel::result::Result;

/// Port yang membuat instance [`Store`] untuk sebuah lokasi penyimpanan.
///
/// Use case tanpa ini akan bergantung pada filesystem secara langsung; dengan
/// port ini, backend S3/GCS hanya perlu menambah implementasi baru.
pub trait BlockStoreFactory {
    /// Tipe store yang dihasilkan.
    type Store: Store;

    /// Membuka (dan bila perlu membuat) store di `path`.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila lokasi penyimpanan tidak dapat disiapkan.
    fn open(&self, path: &Path) -> Result<Self::Store>;
}
