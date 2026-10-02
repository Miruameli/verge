//! File: `file_ref_pointer.rs`
//!
//! Deskripsi: Implementasi `RefPointer` di atas pointer berkas.
//! Layer: infrastructure/commit/file-system
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
//!   - `config/repository_layout.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::fs;
use std::path::{Path, PathBuf};

use crate::config::repository_layout::RepositoryLayout;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::commit::value_objects::commit_ref::validate_name;
use crate::domain::ident::value_objects::digest_text::{parse_hex, HexText};
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Prefix yang wajib ada pada isi `HEAD` sebelum nama branch.
const HEAD_REF_PREFIX: &str = "ref: refs/heads/";

/// Pointer branch berupa berkas berisi 64 hex karakter.
///
/// Satu branch = satu berkas kecil, jadi membuat branch hanya menambah satu
/// inode: tidak ada blok yang disalin.
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
    /// Mengembalikan [`InvalidName`](VergeError::InvalidName) bila nama branch
    /// tidak aman dipakai sebagai satu segmen path.
    fn pointer_path(&self, branch: &str) -> Result<PathBuf> {
        validate_name(branch)?;
        Ok(self.layout.heads().join(branch))
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
        let raw = read_pointer(&self.layout.head_file())?
            .ok_or_else(|| VergeError::NotARepository(self.layout.root.clone()))?;
        let branch = raw
            .strip_prefix(HEAD_REF_PREFIX)
            .ok_or_else(|| VergeError::MalformedPointer(raw.clone()))?;
        validate_name(branch)?;
        Ok(branch.to_owned())
    }

    /// Membaca ujung `branch`.
    ///
    /// Returns:
    /// - Ok(None) — branch belum punya commit.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](VergeError::InvalidName) untuk nama tidak
    /// valid dan [`MalformedPointer`](VergeError::MalformedPointer) bila isi
    /// pointer bukan digest yang valid.
    fn resolve(&self, branch: &str) -> Result<Option<CommitId>> {
        let raw = read_pointer(&self.pointer_path(branch)?)?;
        raw.map(|text| parse_pointer(&text)).transpose()
    }

    /// Memindahkan `branch` ke `id`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](VergeError::InvalidName) untuk nama tidak
    /// valid dan error I/O bila pointer tidak dapat ditulis.
    fn advance(&self, branch: &str, id: CommitId) -> Result<()> {
        write_pointer(&self.pointer_path(branch)?, &id.to_hex())
    }
}

/// Membaca isi pointer; `Ok(None)` bila berkasnya belum ada.
fn read_pointer(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(raw) => Ok(Some(raw.trim_end_matches(['\n', '\r']).to_owned())),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(source.into()),
    }
}

/// Mem-parse isi pointer menjadi `CommitId`.
///
/// # Errors
///
/// Mengembalikan [`MalformedPointer`](VergeError::MalformedPointer) bila isi
/// bukan hex huruf kecil sepanjang tepat 64 karakter.
fn parse_pointer(raw: &str) -> Result<CommitId> {
    parse_hex(raw).map_err(|_| VergeError::MalformedPointer(raw.to_owned()))
}

/// Menulis pointer secara atomik: temp file + rename di direktori yang sama.
/// KONTEKS: `fs::write` langsung bisa meninggalkan pointer kosong bila proses
/// mati di tengah menulis; ALTERNATIF: berkas `.tmp-` yang tertinggal saat rename
/// gagal tidak pernah dibaca engine dan ditimpa percobaan berikutnya.
fn write_pointer(path: &Path, hex: &str) -> Result<()> {
    let Some(parent) = path.parent() else {
        return Err(VergeError::Io(std::io::Error::other("tanpa induk")));
    };
    fs::create_dir_all(parent)?;
    let temp = parent.join(format!("{hex}.tmp-{}", std::process::id()));
    fs::write(&temp, format!("{hex}\n")).and_then(|()| fs::rename(&temp, path))?;
    Ok(())
}
