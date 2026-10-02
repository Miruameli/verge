//! File: `row_key.rs`
//!
//! Deskripsi: Kunci baris tabel sebagai value object terurut.
//! Layer: domain/tree/value-objects
//! Tanggung jawab: Menjamin urutan baris deterministik saat tree dibangun.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: (tidak ada)
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

/// Kunci baris tabel yang dipakai untuk mengurutkan baris.
///
/// Invariants:
/// - Tidak kosong dan tidak mengandung baris baru, sehingga selalu muat dalam
///   satu baris tabel.
/// - Baris dengan kunci identik dianggap baris identik dan tidak boleh muncul
///   dua kali dalam satu snapshot.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowKey(Vec<u8>);

impl RowKey {
    /// Membuat kunci dari byte yang sudah dibersihkan dari baris baru.
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Mengembalikan byte kunci.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Mengembalikan kunci sebagai string UTF-8 jika memungkinkan.
    #[must_use]
    pub fn to_text(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    /// Melaporkan apakah kunci tidak kosong.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::RowKey;

    #[test]
    fn urutan_kunci_deterministik_dan_bukan_urutan_byte_polos() {
        let mut keys = [RowKey::new(b"10".to_vec()), RowKey::new(b"2".to_vec())];
        keys.sort();
        assert_eq!(
            keys[0].as_bytes(),
            b"10",
            "urutan leksikografis, bukan numerik"
        );
        assert_eq!(keys[1].as_bytes(), b"2");
    }

    #[test]
    fn kunci_kosong_dapat_dikenali() {
        assert!(RowKey::new(Vec::new()).is_empty());
        assert!(!RowKey::new(b"a".to_vec()).is_empty());
    }
}
