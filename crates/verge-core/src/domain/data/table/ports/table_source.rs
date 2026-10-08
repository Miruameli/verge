//! File: `table_source.rs`
//!
//! Deskripsi: Port sumber data tabel dari luar repository.
//! Layer: domain/table/ports
//! Tanggung jawab: Membaca isi tabel sebelum disimpan sebagai blok.
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

use std::path::Path;

use crate::shared::kernel::result::Result;

/// Port pembacaan data tabel dari berkas, stream, atau input lain.
///
/// Data yang dikembalikan harus sudah utuh di memori: use case hanya
/// menuliskan byte ke storage, bukan Memorstream.
pub trait TableSource {
    /// Membaca seluruh isi `path`.
    ///
    /// Returns:
    /// - Ok(Vec<u8>) — isi mentah, apa adanya tanpa normalisasi.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila sumber tidak dapat dibaca atau melebihi
    /// [`MAX_TABLE_BYTES`](crate::domain::table::ports::table_source::MAX_TABLE_BYTES).
    fn read_all(&self, path: &Path) -> Result<Vec<u8>>;
}

impl<T: TableSource + ?Sized> TableSource for &T {
    fn read_all(&self, path: &Path) -> Result<Vec<u8>> {
        (**self).read_all(path)
    }
}

/// Batas ukuran data tabel yang diterima dalam satu operasi.
///
/// Alasannya: satu commit menyalin isi tabel ke memori, sehingga tanpa batas
/// file besar dapat menghabiskan memori proses.
pub const MAX_TABLE_BYTES: usize = 256 * 1024 * 1024;
