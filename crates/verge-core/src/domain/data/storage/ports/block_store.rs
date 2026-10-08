//! File: `block_store.rs`
//!
//! Deskripsi: Port `Store` — penyimpanan blok immutable.
//! Layer: domain/storage/ports
//! Tanggung jawab: Mendefinisikan kontrak baca/tulis blok content-addressed.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/storage/value-objects/block_id.rs
//!   - shared/kernel/result.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::storage::value_objects::put_outcome::PutOutcome;
use crate::shared::kernel::result::Result;

/// Port penyimpanan blok immutable dan content-addressed.
///
/// Kontrak untuk semua backend (filesystem, S3, GCS):
/// - Menulis byte yang sama dua kali menghasilkan identifier yang sama dan
///   tidak menggandakan data.
/// - Konten yang tersimpan tidak pernah berubah setelah ditulis.
pub trait Store {
    /// Menulis `data` dan mengembalikan identifier-nya.
    ///
    /// Returns:
    /// - Ok(PutOutcome) — identifier hasil tulis; `inserted == false` bila
    ///   blok sudah ada sehingga penyimpanan tidak berubah.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila data tidak dapat disimpan secara durable.
    fn put(&self, data: &[u8]) -> Result<PutOutcome>;

    /// Membaca blok dengan identifier `id`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`BlockNotFound`](crate::VergeError::BlockNotFound) bila
    /// blok tidak ada.
    fn get(&self, id: &BlockId) -> Result<Vec<u8>>;

    /// Melaporkan apakah blok dengan identifier `id` tersedia.
    ///
    /// Performance: secepat OS untuk memeriksa keberadaan berkas.
    fn contains(&self, id: &BlockId) -> bool;
}

impl<T: Store + ?Sized> Store for &T {
    fn put(&self, data: &[u8]) -> Result<PutOutcome> {
        (**self).put(data)
    }

    fn get(&self, id: &BlockId) -> Result<Vec<u8>> {
        (**self).get(id)
    }

    fn contains(&self, id: &BlockId) -> bool {
        (**self).contains(id)
    }
}
