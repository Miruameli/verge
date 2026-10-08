//! File: `metadata_writer.rs`
//!
//! Deskripsi: Port penulisan metadata repository (direktori dan berkas kecil).
//! Layer: domain/storage/ports
//! Tanggung jawab: Menjaga seluruh I/O tetap berada di infrastructure.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - shared/kernel/result.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use std::path::Path;

use crate::shared::kernel::result::Result;

/// Port untuk menulis metadata repository: direktori dan berkas kecil.
///
/// Dipakai use case bootstrap repository agar use case tetap murni orkestrasi
/// dan dapat diuji tanpa filesystem.
pub trait MetadataWriter {
    /// Membuat `path` beserta seluruh direktori induknya bila belum ada.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori tidak dapat dibuat.
    fn create_dir_all(&self, path: &Path) -> Result<()>;

    /// Menulis `contents` ke `path`, menimpa bila sudah ada.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila berkas tidak dapat ditulis.
    fn write(&self, path: &Path, contents: &[u8]) -> Result<()>;

    /// Menghapus direktori `path` beserta seluruh isinya.
    ///
    /// Dipakai untuk rollback ketika bootstrap gagal di tengah jalan.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori tidak dapat dihapus.
    fn remove_dir_all(&self, path: &Path) -> Result<()>;

    /// Melaporkan apakah `path` sudah ada.
    fn exists(&self, path: &Path) -> bool;
}
