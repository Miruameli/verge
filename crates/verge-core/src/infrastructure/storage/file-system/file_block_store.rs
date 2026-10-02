//! File: `file_block_store.rs`
//!
//! Deskripsi: Implementasi `Store` di filesystem lokal.
//! Layer: infrastructure/storage/file-system
//! Tanggung jawab: Menyimpan blok immutable dengan layout fan-out dan tulis atomik.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/storage/ports/block_store.rs`
//!   - `config/storage_layout.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::storage_layout::StorageLayout;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::storage::value_objects::put_outcome::PutOutcome;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Block store yang akar direktorinya berada di filesystem lokal.
///
/// Path blok memakai fan-out dua tingkat (`ab/cd/<hex>`) agar tidak ada direktori
/// dengan ribuan entri. Penulisan melalui berkas sementara + fsync + rename,
/// sehingga blok tidak pernah terlihat setengah tertulis.
#[derive(Debug, Clone)]
pub struct FileBlockStore {
    /// Direktori akar yang memuat seluruh blok.
    root: PathBuf,
}

impl FileBlockStore {
    /// Membuka store di `root`, membuat direktorinya bila belum ada.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori tidak dapat dibuat.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    /// Mengembalikan direktori akar store.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Mengembalikan path on-disk untuk identifier `id`.
    #[must_use]
    pub fn path_of(&self, id: &BlockId) -> PathBuf {
        StorageLayout::block_path(&self.root, id)
    }
}

impl Store for FileBlockStore {
    /// Menulis blok secara atomik dan deduplikasi.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori atau berkas tidak dapat ditulis.
    fn put(&self, data: &[u8]) -> Result<PutOutcome> {
        let id = BlockId::of(data);
        let path = self.path_of(&id);
        if path.is_file() {
            return Ok(PutOutcome {
                id,
                inserted: false,
            });
        }
        let parent = path.parent().ok_or_else(|| {
            VergeError::Io(std::io::Error::other("path blok tanpa direktori induk"))
        })?;
        fs::create_dir_all(parent)?;

        let temp = parent.join(format!("{}.tmp-{}", id.to_hex(), std::process::id()));
        discard_temp_file(&temp);
        let written = write_and_sync(&temp, data).and_then(|()| fs::rename(&temp, &path));
        if let Err(source) = written {
            discard_temp_file(&temp);
            return Err(source.into());
        }
        sync_dir(parent);
        Ok(PutOutcome { id, inserted: true })
    }

    /// Membaca isi blok.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`BlockNotFound`](crate::VergeError::BlockNotFound) bila
    /// blok tidak ada atau tidak dapat dibaca sebagai berkas biasa.
    fn get(&self, id: &BlockId) -> Result<Vec<u8>> {
        fs::read(self.path_of(id)).map_err(|source| VergeError::BlockNotFound { id: *id, source })
    }

    /// Melaporkan ketersediaan blok di filesystem.
    fn contains(&self, id: &BlockId) -> bool {
        self.path_of(id).is_file()
    }
}

/// Menulis `data` ke `path` lalu mem-flush-nya ke storage stabil.
fn write_and_sync(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(data)?;
    file.sync_all()
}

/// Menghapus berkas sementara bila ada.
///
/// Kegagalan sengaja diabaikan: berkas `.tmp-` tidak pernah dibaca engine dan
/// akan ditimpa pada percobaan tulis berikutnya untuk identifier yang sama.
fn discard_temp_file(path: &Path) {
    drop(fs::remove_file(path));
}

/// Best-effort fsync direktori agar rename bertahan setelah crash.
fn sync_dir(dir: &Path) {
    if let Ok(handle) = File::open(dir) {
        drop(handle.sync_all());
    }
}
