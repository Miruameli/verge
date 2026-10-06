//! File: `row_change.rs`
//!
//! Deskripsi: Jenis perubahan satu baris tabel.
//! Layer: domain/tree/diff
//! Tanggung jawab: Membedakan baris tambah, ubah, dan hapus.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada)
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

/// Jenis perubahan pada satu baris.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowChange {
    /// Baris hanya ada di tabel baru.
    Added {
        /// Kunci baris.
        key: String,
        /// Nilai pada tabel baru.
        value: Vec<u8>,
    },
    /// Baris hanya ada di tabel lama.
    Removed {
        /// Kunci baris.
        key: String,
        /// Nilai pada tabel lama.
        value: Vec<u8>,
    },
    /// Baris ada di kedua tabel dengan nilai berbeda.
    Modified {
        /// Kunci baris.
        key: String,
        /// Nilai lama.
        before: Vec<u8>,
        /// Nilai baru.
        after: Vec<u8>,
    },
}

impl RowChange {
    /// Mengembalikan kunci baris yang berubah.
    #[must_use]
    pub fn key(&self) -> &str {
        match self {
            Self::Added { key, .. } | Self::Removed { key, .. } | Self::Modified { key, .. } => key,
        }
    }
}
