//! File: `merge_strategy.rs`
//!
//! Deskripsi: Strategi resolusi konflik merge.
//! Layer: domain/merge
//! Tanggung jawab: Menentukan sisi mana yang menang saat baris berkonflik.
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

/// Cara konflik baris diselesaikan saat merge.
///
/// Immutability: penuh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Tidak menyelesaikan apa pun; konflik dilaporkan dan merge dibatalkan.
    Manual,
    /// Mempertahankan nilai branch aktif.
    Ours,
    /// Mengambil nilai branch yang digabung.
    Theirs,
    /// Mengambil sisi dengan waktu commit terbaru.
    LastWriteWins,
}

impl MergeStrategy {
    /// Mengembalikan nama stabil strategi untuk dicetak pengguna.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Ours => "ours",
            Self::Theirs => "theirs",
            Self::LastWriteWins => "last-write-wins",
        }
    }

    /// Mem-parse nama strategi dari argumen pengguna.
    ///
    /// Args:
    /// - text — nama strategi; `last_write_wins` diperlakukan sama dengan
    ///   `last-write-wins` agar nama dari shell juga diterima.
    ///
    /// Returns:
    /// - Option<MergeStrategy> — strategi yang dikenali atau `None`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        match text.trim().to_ascii_lowercase().replace('_', "-").as_str() {
            "manual" => Some(Self::Manual),
            "ours" => Some(Self::Ours),
            "theirs" => Some(Self::Theirs),
            "last-write-wins" => Some(Self::LastWriteWins),
            _ => None,
        }
    }

    /// Mengembalikan daftar nama yang valid untuk pesan kesalahan.
    #[must_use]
    pub fn names() -> &'static str {
        "manual, ours, theirs, last-write-wins"
    }
}
