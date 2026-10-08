//! File: digest.rs
//!
//! Deskripsi: Value object `Digest` — identifier content-addressed 32 byte.
//! Layer: domain/ident/value-objects
//! Tanggung jawab: Menyimpan digest dan menghitungnya dari byte input.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - sha2 (perhitungan SHA-256)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)
//!   - ADR-0004 (Ketergantungan minimal, versi dipin)

use sha2::{Digest as _, Sha256};

/// Panjang digest dalam byte.
pub const DIGEST_LEN: usize = 32;

/// Digest kosong; hanya dipakai sebagai placeholder sebelum identitas dihitung.
const fn zero() -> [u8; DIGEST_LEN] {
    [0_u8; DIGEST_LEN]
}

/// Identifier content-addressed berukuran 32 byte.
///
/// Invariants:
/// - Nilainya selalu hasil SHA-256 dari encoding kanonik objek.
/// - Immutable: nilai baru hanya diperoleh dengan menghitung ulang digest.
///
/// Example:
/// ```
/// use verge_core::Digest;
///
/// assert_eq!(Digest::of(b"abc"), Digest::of(b"abc"));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest([u8; DIGEST_LEN]);

impl Digest {
    /// Menghitung identifier dari `bytes`.
    ///
    /// Args:
    /// - bytes — data yang akan di-hash.
    ///
    /// Returns:
    /// - Digest — SHA-256 dari data tersebut.
    ///
    /// Performance: O(n) terhadap panjang data.
    /// Thread-safe: ya (tidak menyimpan state bersama).
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let mut out = zero();
        out.copy_from_slice(&hasher.finalize());
        Self(out)
    }

    /// Mengembalikan identifier yang belum menunjuk objek apa pun.
    ///
    /// Hanya dipakai sebagai placeholder saat objek dibangun dan identitasnya
    /// langsung dihitung ulang pada baris berikutnya.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self(zero())
    }

    /// Membentuk digest dari byte yang sudah divalidasi oleh `parse_hex`.
    ///
    /// Hanya `parse_hex` yang memanggilnya, sehingga input selalu hex valid.
    pub(crate) const fn from_bytes(bytes: [u8; DIGEST_LEN]) -> Self {
        Self(bytes)
    }

    /// Mengembalikan byte mentah dari identifier.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; DIGEST_LEN] {
        &self.0
    }
}
