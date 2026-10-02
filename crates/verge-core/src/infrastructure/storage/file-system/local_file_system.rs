//! File: `local_file_system.rs`
//!
//! Deskripsi: Implementasi `MetadataWriter` di filesystem lokal.
//! Layer: infrastructure/storage/file-system
//! Tanggung jawab: Menyediakan operasi direktori/berkas untuk use case bootstrap.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/storage/ports/metadata_writer.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use std::fs;
use std::path::Path;

use crate::domain::storage::ports::metadata_writer::MetadataWriter;
use crate::shared::kernel::result::Result;

/// Penulis metadata berbasis filesystem lokal tanpa state.
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalFileSystem;

impl MetadataWriter for LocalFileSystem {
    /// Membuat direktori beserta induknya bila belum ada.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori tidak dapat dibuat.
    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    /// Menulis isi berkas, menimpa bila sudah ada.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila berkas tidak dapat ditulis.
    fn write(&self, path: &Path, contents: &[u8]) -> Result<()> {
        fs::write(path, contents)?;
        Ok(())
    }

    /// Menghapus direktori beserta isinya.
    ///
    /// Direktori yang tidak ada dianggap sukses agar rollback tetap idempoten.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori tidak dapat dihapus.
    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    /// Melaporkan apakah path sudah ada.
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}
