//! File: `file_table_source.rs`
//!
//! Deskripsi: Implementasi `TableSource` untuk berkas lokal.
//! Layer: infrastructure/table/file-system
//! Tanggung jawab: Membaca isi tabel apa adanya dengan batas ukuran.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/table/ports/table_source.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::fs;
use std::path::Path;

use crate::domain::table::ports::table_source::{TableSource, MAX_TABLE_BYTES};
use crate::shared::kernel::result::Result;

/// Sumber data tabel berupa berkas biasa di filesystem lokal.
#[derive(Debug, Clone, Copy, Default)]
pub struct FileTableSource;

impl TableSource for FileTableSource {
    /// Membaca seluruh isi `path` tanpa menormalisasi satu byte pun.
    ///
    /// Args:
    /// - path — berkas sumber, misalnya hasil `verge import`.
    ///
    /// Returns:
    /// - Ok(Vec<u8>) — isi mentah apa adanya; newline dan line ending asli
    ///   dipertahankan supaya digest blok mencerminkan berkas yang diimpor.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila berkas tidak dapat dibaca atau panjangnya
    /// melebihi [`MAX_TABLE_BYTES`].
    ///
    /// Performance: satu syscall `stat` lalu satu `read`; berkas melebihi batas
    /// ditolak tanpa isinya pernah dimuat ke memori.
    fn read_all(&self, path: &Path) -> Result<Vec<u8>> {
        let length = fs::metadata(path)?.len();
        if length > MAX_TABLE_BYTES as u64 {
            return Err(oversized(length).into());
        }
        Ok(fs::read(path)?)
    }
}

/// Membangun error penolakan untuk berkas sebesar `length` byte.
fn oversized(length: u64) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("tabel sebesar {length} byte melebihi batas {MAX_TABLE_BYTES} byte"),
    )
}
