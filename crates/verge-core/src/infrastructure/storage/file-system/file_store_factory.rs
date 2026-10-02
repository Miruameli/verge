//! File: `file_store_factory.rs`
//!
//! Deskripsi: Implementasi `BlockStoreFactory` untuk filesystem lokal.
//! Layer: infrastructure/storage/file-system
//! Tanggung jawab: Menyediakan store blok dari path tanpa membocorkan detail.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - infrastructure/storage/file-system/file_block_store.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::path::Path;

use crate::domain::storage::ports::block_store_factory::BlockStoreFactory;
use crate::infrastructure::storage::file_system::file_block_store::FileBlockStore;
use crate::shared::kernel::result::Result;

/// Factory filesystem lokal tanpa state, sehingga bisa dipakai bersama.
#[derive(Debug, Clone, Copy, Default)]
pub struct FileStoreFactory;

impl BlockStoreFactory for FileStoreFactory {
    type Store = FileBlockStore;

    /// Membuka store blok di `path`.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori tidak dapat dibuat.
    fn open(&self, path: &Path) -> Result<FileBlockStore> {
        FileBlockStore::open(path)
    }
}
