//! File: `digest_text.rs`
//!
//! Deskripsi: Representasi teks (hex) untuk `Digest`.
//! Layer: domain/ident/value-objects
//! Tanggung jawab: Konversi digest ke/dari teks hex kanonik.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/ident/value-objects/digest.rs
//!   - `shared/exceptions/parse_digest_error.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use core::fmt;

use crate::domain::ident::value_objects::digest::{Digest, DIGEST_LEN};
use crate::shared::exceptions::parse_digest_error::ParseDigestError;

/// Trait untuk merender identifier sebagai teks hex huruf kecil.
pub trait HexText {
    /// Merender identifier sebagai hex kanonik sepanjang 64 karakter.
    fn to_hex(&self) -> String;
}

impl HexText for Digest {
    fn to_hex(&self) -> String {
        let mut out = String::with_capacity(DIGEST_LEN * 2);
        for byte in self.as_bytes() {
            out.push(hex_digit(byte >> 4));
            out.push(hex_digit(byte & 0x0f));
        }
        out
    }
}

/// Mem-parse identifier dari teks hex huruf kecil.
///
/// Args:
/// - text — kandidat hex sepanjang tepat 64 karakter huruf kecil.
///
/// Returns:
/// - Ok(Digest) — bila format valid.
/// - Err(ParseDigestError) — bila panjang atau karakter tidak valid.
///
/// # Errors
///
/// Mengembalikan [`ParseDigestError`] bila teks bukan hex huruf lowercase
/// sepanjang tepat 64 karakter.
///
/// Example:
/// ```
/// use verge_core::Digest;
/// use verge_core::domain::ident::value_objects::digest_text::{parse_hex, HexText};
///
/// let digest = Digest::of(b"verge");
/// assert_eq!(parse_hex(&digest.to_hex()).expect("valid hex"), digest);
/// assert!(parse_hex("verge").is_err());
/// ```
///
/// Thread-safe: ya (fungsi murni, tanpa state bersama).
pub fn parse_hex(text: &str) -> Result<Digest, ParseDigestError> {
    let bytes = text.as_bytes();
    if bytes.len() != DIGEST_LEN * 2 {
        return Err(ParseDigestError::new(text));
    }
    let mut out = [0_u8; DIGEST_LEN];
    for (index, slot) in out.iter_mut().enumerate() {
        match (hex_value(bytes[index * 2]), hex_value(bytes[index * 2 + 1])) {
            (Some(high), Some(low)) => *slot = (high << 4) | low,
            _ => return Err(ParseDigestError::new(text)),
        }
    }
    Ok(Digest::from_bytes(out))
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest({})", self.to_hex())
    }
}

const fn hex_digit(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        _ => (b'a' + nibble - 10) as char,
    }
}

const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}
