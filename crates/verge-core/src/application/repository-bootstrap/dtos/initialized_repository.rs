//! File: `initialized_repository.rs`
//!
//! Deskripsi: DTO hasil use case inisialisasi repository.
//! Layer: application/repository-bootstrap/dtos
//! Tanggung jawab: Menyampaikan layout dan store yang siap dipakai pemanggil.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `config/repository_layout.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use crate::config::repository_layout::RepositoryLayout;
use crate::domain::storage::ports::block_store::Store;

/// Repository yang baru dinisialisasi beserta store backend-nya.
///
/// Immutability: penuh; metadata repository tidak berubah lewat DTO ini.
#[derive(Debug)]
pub struct InitializedRepository<S: Store> {
    /// Layout path repository.
    layout: RepositoryLayout,
    /// Store blok yang sudah terbuka.
    store: S,
}

impl<S: Store> InitializedRepository<S> {
    /// Membungkus hasil bootstrap.
    #[must_use]
    pub fn new(layout: RepositoryLayout, store: S) -> Self {
        Self { layout, store }
    }

    /// Mengembalikan layout repository.
    #[must_use]
    pub fn layout(&self) -> &RepositoryLayout {
        &self.layout
    }

    /// Mengembalikan store blok yang terbuka.
    #[must_use]
    pub fn store(&self) -> &S {
        &self.store
    }
}
