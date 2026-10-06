//! File: `file_ref_pointer.rs`
//!
//! Deskripsi: Implementasi `RefPointer` di atas pointer berkas.
//! Layer: infrastructure/commit/file-system/refs
//! Tanggung jawab: Menyimpan branch aktif dan ujung tiap branch secara atomik.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/ref_pointer.rs`
//!   - `infrastructure/commit/file-system/refs/pointer_file.rs`
//!   - `config/repository_layout.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2), #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0005, ADR-0007

use crate::config::repository_layout::RepositoryLayout;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::branch_name_policy::validate_name;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::infrastructure::commit::file_system::refs::{pointer_dir, pointer_file};
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Prefix yang wajib ada pada isi `HEAD` sebelum nama branch.
const HEAD_REF_PREFIX: &str = "ref: refs/heads/";

/// Pointer branch berupa berkas berisi 64 hex karakter.
///
/// Satu branch = satu berkas kecil, jadi membuatnya tidak menyalin blok.
#[derive(Debug, Clone)]
pub struct FileRefPointer {
    /// Layout repository tempat `HEAD` dan `refs/heads/` berada.
    layout: RepositoryLayout,
}

impl FileRefPointer {
    /// Membuat pointer untuk repository pada `layout`.
    ///
    /// Args:
    /// - layout — path repository hasil [`RepositoryLayout::under`].
    ///
    /// Returns:
    /// - Self — pointer tanpa I/O; direktori baru dibuat saat pointer ditulis.
    #[must_use]
    pub fn new(layout: RepositoryLayout) -> Self {
        Self { layout }
    }

    /// Mengembalikan path pointer `branch` setelah nama divalidasi.
    ///
    /// # Errors
    ///
    /// [`InvalidName`](VergeError::InvalidName) bila nama tidak aman sebagai path.
    fn pointer_path(&self, branch: &str) -> Result<std::path::PathBuf> {
        pointer_dir::path_for(&self.layout.heads(), branch)
    }
}

impl RefPointer for FileRefPointer {
    /// Membaca branch aktif dari berkas `HEAD`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`NotARepository`](VergeError::NotARepository) bila
    /// `HEAD` belum ada dan
    /// [`MalformedPointer`](VergeError::MalformedPointer) bila isinya bukan
    /// `ref: refs/heads/<nama>` atau nama branch-nya tidak valid.
    fn head_branch(&self) -> Result<String> {
        let raw = pointer_file::read(&self.layout.head_file())?
            .ok_or_else(|| VergeError::NotARepository(self.layout.root.clone()))?;
        let branch = raw
            .strip_prefix(HEAD_REF_PREFIX)
            .ok_or_else(|| VergeError::MalformedPointer(raw.clone()))?;
        validate_name(branch)?;
        Ok(branch.to_owned())
    }

    /// Membaca ujung `branch`.
    ///
    /// # Errors
    ///
    /// [`InvalidName`](VergeError::InvalidName) untuk nama tidak valid dan
    /// [`MalformedPointer`](VergeError::MalformedPointer) bila isi bukan digest.
    fn resolve(&self, branch: &str) -> Result<Option<CommitId>> {
        let raw = pointer_file::read(&self.pointer_path(branch)?)?;
        raw.map(|text| pointer_file::parse(&text)).transpose()
    }

    /// Memindahkan `branch` ke `id`.
    ///
    /// # Errors
    ///
    /// [`InvalidName`](VergeError::InvalidName) untuk nama tidak valid dan
    /// error I/O bila pointer tidak dapat ditulis.
    fn advance(&self, branch: &str, id: CommitId) -> Result<()> {
        pointer_file::write(&self.pointer_path(branch)?, &id.to_hex())
    }

    /// Mengembalikan seluruh nama branch yang punya pointer.
    ///
    /// # Errors
    ///
    /// Error I/O bila direktori pointer tidak dapat dibaca dan
    /// [`MalformedPointer`](VergeError::MalformedPointer) bila ada entri rusak.
    fn branches(&self) -> Result<Vec<String>> {
        pointer_dir::list(&self.layout.heads())
    }

    /// Mengalihkan `HEAD` ke `branch` yang sudah punya pointer.
    ///
    /// KONTEKS: hanya `HEAD` yang berubah sehingga perpindahan tetap O(1).
    ///
    /// # Errors
    ///
    /// Mengembalikan [`UnknownBranch`](VergeError::UnknownBranch) bila branch
    /// belum ada dan error I/O bila `HEAD` tidak dapat ditulis.
    fn switch(&self, branch: &str) -> Result<()> {
        let path = self.pointer_path(branch)?;
        if pointer_file::read(&path)?.is_none() {
            return Err(VergeError::UnknownBranch(branch.to_owned()));
        }
        pointer_file::write(
            &self.layout.head_file(),
            &format!("{HEAD_REF_PREFIX}{branch}"),
        )
    }

    /// Menghapus pointer `branch`.
    ///
    /// # Errors
    ///
    /// [`BranchInUse`](VergeError::BranchInUse) bila branch aktif dan
    /// [`UnknownBranch`](VergeError::UnknownBranch) bila pointer tidak ada.
    fn delete(&self, branch: &str) -> Result<()> {
        let path = self.pointer_path(branch)?;
        if self.head_branch().is_ok_and(|head| head == branch) {
            return Err(VergeError::BranchInUse(branch.to_owned()));
        }
        pointer_file::remove(&path)
    }
}
