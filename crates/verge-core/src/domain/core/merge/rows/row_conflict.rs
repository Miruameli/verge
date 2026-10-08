//! File: `row_conflict.rs`
//!
//! Deskripsi: Catatan konflik satu baris saat merge.
//! Layer: domain/merge/rows
//! Tanggung jawab: Menyimpan ketiga sisi nilai baris yang tidak dapat digabung.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi eksternal)
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

/// Baris yang diubah kedua sisi dari nilai base yang sama.
///
/// `None` berarti baris tidak ada pada sisi tersebut; itu mencakup kasus
/// penghapusan yang bentrok dengan perubahan di sisi lain.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowConflict {
    /// Kunci baris yang berkonflik.
    pub key: String,
    /// Nilai pada base, atau `None` bila baris belum ada di base.
    pub base: Option<Vec<u8>>,
    /// Nilai pada branch aktif, atau `None` bila dihapus.
    pub ours: Option<Vec<u8>>,
    /// Nilai pada branch yang digabung, atau `None` bila dihapus.
    pub theirs: Option<Vec<u8>>,
}

impl RowConflict {
    /// Mengembalikan nilai pada `base` atau byte kosong bila baris belum ada.
    ///
    /// Returns:
    /// - &[u8] — nilai base, atau slice kosong bila baris belum ada.
    #[must_use]
    pub fn base_or_empty(&self) -> &[u8] {
        self.base.as_deref().unwrap_or_default()
    }

    /// Mengembalikan nilai pada branch aktif atau byte kosong bila dihapus.
    #[must_use]
    pub fn ours_or_empty(&self) -> &[u8] {
        self.ours.as_deref().unwrap_or_default()
    }

    /// Mengembalikan nilai pada branch yang digabung atau byte kosong.
    #[must_use]
    pub fn theirs_or_empty(&self) -> &[u8] {
        self.theirs.as_deref().unwrap_or_default()
    }
}
