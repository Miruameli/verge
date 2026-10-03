//! File: `file_tag_pointer.rs`
//!
//! Deskripsi: Implementasi `TagPointer` di atas pointer berkas.
//! Layer: infrastructure/commit/file-system/refs
//! Tanggung jawab: Menyimpan tag sebagai pointer yang tidak pernah ditimpa.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/tag_pointer.rs`
//!   - `infrastructure/commit/file-system/refs/{pointer_dir,pointer_file}.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::config::repository_layout::RepositoryLayout;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::infrastructure::commit::file_system::refs::{pointer_dir, pointer_file};
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Pointer tag berupa berkas berisi 64 hex di `refs/tags/`.
///
/// ALTERNATIF: satu berkas berisi seluruh tag; ditolak karena dua proses yang
/// membuat tag berbeda akan berebut berkas yang sama, sementara pointer per tag
/// membuat `create` dan `delete` tidak saling ganggu.
#[derive(Debug, Clone)]
pub struct FileTagPointer {
    /// Layout repository tempat `refs/tags/` berada.
    layout: RepositoryLayout,
}

impl FileTagPointer {
    /// Membuat pointer tag untuk repository pada `layout`.
    ///
    /// Returns:
    /// - Self — pointer tanpa I/O; direktori dibuat saat tag pertama ditulis.
    #[must_use]
    pub fn new(layout: RepositoryLayout) -> Self {
        Self { layout }
    }

    /// Mengembalikan path pointer tag `name` setelah nama divalidasi.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](VergeError::InvalidName) bila nama tidak
    /// aman sebagai satu segmen path.
    fn tag_path(&self, name: &str) -> Result<std::path::PathBuf> {
        pointer_dir::path_for(&self.layout.tags(), name)
    }
}

impl TagPointer for FileTagPointer {
    /// Menunjuk tag `name` ke commit `id`, menolak nama yang sudah dipakai.
    ///
    /// KONTEKS: pemeriksaan ada dilakukan sebelum penulisan; KENAPA: tag yang
    /// bergeser diam-diam membuat laporan yang menyebut nama tag itu menyesatkan,
    /// sehingga penolakan harus terjadi sebelum apa pun ditulis ke disk.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`TagAlreadyExists`](VergeError::TagAlreadyExists) bila
    /// nama sudah dipakai dan error I/O bila pointer gagal ditulis.
    fn create(&self, name: &str, id: CommitId) -> Result<()> {
        let path = self.tag_path(name)?;
        if pointer_file::read(&path)?.is_some() {
            return Err(VergeError::TagAlreadyExists(name.to_owned()));
        }
        pointer_file::write(&path, &id.to_hex())
    }

    /// # Errors
    ///
    /// Mengembalikan error I/O bila pointer tidak dapat dibaca.
    fn resolve(&self, name: &str) -> Result<Option<CommitId>> {
        let raw = pointer_file::read(&self.tag_path(name)?)?;
        raw.map(|text| pointer_file::parse(&text)).transpose()
    }

    /// # Errors
    ///
    /// Mengembalikan error I/O bila direktori pointer tidak dapat dibaca.
    fn tags(&self) -> Result<Vec<String>> {
        pointer_dir::list(&self.layout.tags())
    }

    /// # Errors
    ///
    /// Mengembalikan [`UnknownTag`](VergeError::UnknownTag) bila tag belum ada
    /// dan error I/O bila pointer gagal dihapus.
    fn delete(&self, name: &str) -> Result<()> {
        let path = self.tag_path(name)?;
        if pointer_file::read(&path)?.is_none() {
            return Err(VergeError::UnknownTag(name.to_owned()));
        }
        pointer_file::remove(&path)
    }
}
