//! File: `table_row.rs`
//!
//! Deskripsi: Baris tabel sebagai pasangan kunci dan nilai.
//! Layer: domain/tree/value-objects
//! Tanggung jawab: Menyimpan isi satu baris tanpa menafsirkan kolomnya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `row_key.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::tree::value_objects::row_key::RowKey;

/// Satu baris tabel: kunci unik dan nilai mentahnya.
///
/// Nilai disimpan apa adanya tanpa menafsirkan kolom, sehingga diff dapat
/// membandingkan nilai sebelum dan sesudah tanpa kehilangan informasi format.
///
/// Invariants:
/// - Kunci tidak kosong.
/// - Nilai tidak mengandung baris baru, karena batasnya satu baris tabel.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow {
    /// Kunci baris.
    pub key: RowKey,
    /// Isi baris setelah kunci, apa adanya.
    pub value: Vec<u8>,
}

impl TableRow {
    /// Membuat baris dari kunci dan nilai.
    ///
    /// Returns:
    /// - Option<`TableRow`> — `None` bila kunci kosong.
    #[must_use]
    pub fn new(key: RowKey, value: Vec<u8>) -> Option<Self> {
        if key.is_empty() {
            return None;
        }
        Some(Self { key, value })
    }

    /// Mengembalikan kunci baris.
    #[must_use]
    pub fn key(&self) -> &RowKey {
        &self.key
    }

    /// Mengembalikan nilai baris.
    #[must_use]
    pub fn value(&self) -> &[u8] {
        &self.value
    }
}
