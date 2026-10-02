//! File: `initialize_repository_fakes.rs`
//!
//! Deskripsi: Port palsu untuk test use case bootstrap repository.
//! Layer: application/repository-bootstrap/use-cases (test support)
//! Tanggung jawab: Menyediakan implementasi in-memory port tanpa filesystem.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/storage/ports/* (hanya trait-nya)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::ports::block_store_factory::BlockStoreFactory;
use crate::domain::storage::ports::metadata_writer::MetadataWriter;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::storage::value_objects::put_outcome::PutOutcome;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Metadata writer palsu yang mencatat path dan isi berkas di memori.
///
/// `create_dir_all` mendaftarkan seluruh direktori induk, meniru filesystem
/// sungguhan; `fail_on` menyimulasikan kegagalan I/O pada path tertentu.
#[derive(Default)]
pub struct FakeFileSystem {
    /// Path yang dianggap ada.
    files: RefCell<HashSet<PathBuf>>,
    /// Isi berkas yang pernah ditulis.
    contents: RefCell<HashMap<PathBuf, Vec<u8>>>,
    /// Path yang engineered gagal saat diakses.
    fail_on: Option<PathBuf>,
}

impl FakeFileSystem {
    /// Membuat fake yang gagal pada `path` tertentu.
    pub fn failing_on(path: PathBuf) -> Self {
        Self {
            fail_on: Some(path),
            ..Self::default()
        }
    }

    /// Mengembalikan salinan isi berkas pada `path` bila ada.
    pub fn contents_of(&self, path: &Path) -> Option<Vec<u8>> {
        self.contents.borrow().get(path).cloned()
    }

    /// Mengembalikan error bila `path` adalah titik kegagalan yang disimulasikan.
    fn guard(&self, path: &Path) -> Result<()> {
        if self.fail_on.as_deref() == Some(path) {
            return Err(VergeError::Io(std::io::Error::other("disk penuh")));
        }
        Ok(())
    }
}

impl MetadataWriter for FakeFileSystem {
    fn create_dir_all(&self, path: &Path) -> Result<()> {
        self.guard(path)?;
        let mut current = Some(path.to_path_buf());
        while let Some(dir) = current {
            self.files.borrow_mut().insert(dir.clone());
            current = dir.parent().map(Path::to_path_buf);
        }
        Ok(())
    }

    fn write(&self, path: &Path, contents: &[u8]) -> Result<()> {
        self.guard(path)?;
        self.files.borrow_mut().insert(path.to_path_buf());
        self.contents
            .borrow_mut()
            .insert(path.to_path_buf(), contents.to_vec());
        Ok(())
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        self.files
            .borrow_mut()
            .retain(|item| !item.starts_with(path));
        self.contents
            .borrow_mut()
            .retain(|item, _| !item.starts_with(path));
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.borrow().contains(path)
    }
}

/// Factory palsu yang mengembalikan store kosong tanpa menyentuh disk.
pub struct FakeFactory {
    /// Bila `true`, `open` selalu gagal.
    pub fail: bool,
}

impl BlockStoreFactory for FakeFactory {
    type Store = EmptyStore;

    fn open(&self, _path: &Path) -> Result<EmptyStore> {
        if self.fail {
            return Err(VergeError::Io(std::io::Error::other(
                "store tidak tersedia",
            )));
        }
        Ok(EmptyStore)
    }
}

/// Store kosong untuk membuktikan use case tidak bergantung implementasi nyata.
#[derive(Debug, Default)]
pub struct EmptyStore;

impl Store for EmptyStore {
    fn put(&self, data: &[u8]) -> Result<PutOutcome> {
        Ok(PutOutcome {
            id: Digest::of(data),
            inserted: true,
        })
    }

    fn get(&self, id: &BlockId) -> Result<Vec<u8>> {
        Err(VergeError::CommitNotFound(*id))
    }

    fn contains(&self, _id: &BlockId) -> bool {
        false
    }
}
