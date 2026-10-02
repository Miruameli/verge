//! File: `record_commit_validation.rs`
//!
//! Deskripsi: Validasi metadata commit.
//! Layer: application/version-control/use-cases
//! Tanggung jawab: Menolak penulis dan pesan yang tidak memenuhi aturan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `record_commit.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Panjang maksimum nama penulis dalam karakter.
const MAX_AUTHOR_LEN: usize = 128;

/// Panjang maksimum pesan commit dalam karakter.
const MAX_MESSAGE_LEN: usize = 4096;

/// Menolak penulis kosong, terlalu panjang, atau berisi karakter kontrol.
///
/// # Errors
///
/// Mengembalikan [`InvalidCommitField`](VergeError::InvalidCommitField) dengan
/// `field: "author"` bila aturan tidak terpenuhi.
pub(super) fn validate_author(author: &str) -> Result<()> {
    let long = author.chars().count() > MAX_AUTHOR_LEN;
    if author.is_empty() || long || author.contains(char::is_control) {
        return Err(VergeError::InvalidCommitField {
            field: "author",
            detail: "must be 1-128 characters without control characters",
        });
    }
    Ok(())
}

/// Menolak pesan kosong setelah dipangkas atau terlalu panjang.
///
/// # Errors
///
/// Mengembalikan [`InvalidCommitField`](VergeError::InvalidCommitField) dengan
/// `field: "message"` bila aturan tidak terpenuhi.
pub(super) fn validate_message(message: &str) -> Result<()> {
    let trimmed = message.trim();
    if trimmed.is_empty() || trimmed.chars().count() > MAX_MESSAGE_LEN {
        return Err(VergeError::InvalidCommitField {
            field: "message",
            detail: "must be 1-4096 characters after trimming",
        });
    }
    Ok(())
}
