//! File: `parse_digest_error.rs`
//!
//! Deskripsi: Error saat identifier digest gagal di-parse dari teks.
//! Layer: shared/exceptions
//! Tanggung jawab: Membawa teks masukan yang tidak valid ke boundary aplikasi.
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
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use core::fmt;

/// Error parsing digest dari representasi teksnya.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDigestError(String);

impl ParseDigestError {
    /// Membuat error baru untuk teks masukan tertentu.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    /// Mengembalikan teks masukan yang gagal di-parse.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ParseDigestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a valid SHA-256 digest: {}", self.0)
    }
}

impl std::error::Error for ParseDigestError {}
