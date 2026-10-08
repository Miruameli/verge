//! File: `table_name.rs`
//!
//! Deskripsi: Nama tabel sebagai value object tervalidasi.
//! Layer: domain/table/value-objects
//! Tanggung jawab: Menolak nama tabel yang bisa keluar dari direktori tabel.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `shared/kernel/result.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Panjang maksimum nama tabel dalam byte.
const MAX_LEN: usize = 64;

/// Nama tabel yang aman dipakai sebagai nama direktori di dalam `.verge`.
///
/// Invariants:
/// - Panjang 1..=64 byte.
/// - Karakter pertama huruf ASCII atau digit, sehingga direktori tersembunyi
///   (`.foo`) tidak mungkin terjadi.
/// - Karakter berikutnya hanya `a-z`, `0-9`, `_`, dan `-`, sehingga tidak ada
///   path separator, spasi, atau karakter kontrol yang lolos ke filesystem.
///
/// Immutability: penuh; nilai dibuat sekali lewat [`TableName::parse`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableName(String);

impl TableName {
    /// Memvalidasi `raw` menjadi nama tabel.
    ///
    /// Args:
    /// - raw — nama tabel dari luar (CLI, file pointer, atau input eksternal).
    ///
    /// Returns:
    /// - Ok(TableName) — nama valid.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidTableName`](VergeError::InvalidTableName) bila
    /// panjang di luar 1..=64 atau karakter di luar allowlist.
    ///
    /// Example:
    /// ```
    /// use verge_core::domain::table::value_objects::table_name::TableName;
    ///
    /// assert_eq!(TableName::parse("users-2026").unwrap().as_str(), "users-2026");
    /// assert!(TableName::parse("../etc/passwd").is_err());
    pub fn parse(raw: &str) -> Result<Self> {
        if raw.is_empty() || raw.len() > MAX_LEN {
            return Err(VergeError::InvalidTableName(raw.to_owned()));
        }
        let mut bytes = raw.bytes();
        let first = bytes.next().unwrap_or_default();
        // KONTEKS: nama tabel selalu huruf kecil agar mengikuti konvensi SQL dan
        // tidak menimbulkan dua tabel yang hanya berbeda kapital.
        // KENAPA: `Users` dan `users` akan menjadi dua direktori berbeda.
        let head_ok = first.is_ascii_lowercase() || first.is_ascii_digit();
        let tail_ok = bytes.all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
        });
        if head_ok && tail_ok {
            Ok(Self(raw.to_owned()))
        } else {
            Err(VergeError::InvalidTableName(raw.to_owned()))
        }
    }

    /// Mengembalikan nama tabel sebagai string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TableName {
    /// Menampilkan nama tabel tanpa tanda kutip.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
