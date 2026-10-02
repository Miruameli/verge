//! File: `file_table_workspace.rs`
//!
//! Deskripsi: Implementasi `TableWorkspace` di atas pointer digest.
//! Layer: infrastructure/table/file-system
//! Tanggung jawab: Menyimpan akar tree sebagai pointer tanpa menyalin isinya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/table/ports/table_workspace.rs`
//!   - `config/repository_layout.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)
//!   - ADR-0006 (Prolly tree untuk tabel)

use std::fs;
use std::path::Path;

use crate::config::repository_layout::RepositoryLayout;
use crate::domain::ident::value_objects::digest_text::{parse_hex, HexText};
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Data kerja tabel yang hanya menyimpan digest akar, bukan salinan byte.
///
/// KENAPA: node tree ditulis sebagai blok immutable oleh use case yang sedang
/// men-stage, sehingga adapter ini cukup menunjuk akar lewat digest. Baris yang
/// tidak berubah antar import tetap menunjuk daun yang sama dan tidak pernah
/// diduplikasi di pointer maupun di block store.
#[derive(Debug, Clone)]
pub struct FileTableWorkspace {
    /// Layout repository tempat pointer data kerja berada.
    layout: RepositoryLayout,
}

impl FileTableWorkspace {
    /// Membuat workspace data kerja untuk repository pada `layout`.
    ///
    /// Args:
    /// - layout — path repository hasil [`RepositoryLayout::under`].
    ///
    /// Returns:
    /// - Self — workspace tanpa state turunan.
    #[must_use]
    pub fn new(layout: RepositoryLayout) -> Self {
        Self { layout }
    }
}

impl TableWorkspace for FileTableWorkspace {
    /// Menunjuk `root` sebagai data kerja tabel tanpa menulis blok apa pun.
    ///
    /// Args:
    /// - name — tabel tujuan.
    /// - root — identifier node akar tree.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila pointer tidak dapat ditulis.
    fn stage(&self, name: &TableName, root: BlockId) -> Result<()> {
        write_pointer(&self.layout.working_file(name), &root.to_hex())
    }

    /// Membaca pointer data kerja tabel.
    ///
    /// Returns:
    /// - Ok(None) — tabel belum pernah di-stage.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`MalformedPointer`](VergeError::MalformedPointer) bila
    /// pointer rusak dan error I/O bila berkasnya tidak dapat dibaca.
    fn staged(&self, name: &TableName) -> Result<Option<BlockId>> {
        let path = self.layout.working_file(name);
        let Some(raw) = read_pointer(&path)? else {
            return Ok(None);
        };
        let id = parse_hex(&raw).map_err(|_| VergeError::MalformedPointer(raw.clone()))?;
        Ok(Some(id))
    }
}

/// Membaca isi pointer; `Ok(None)` bila tabel belum pernah di-stage.
fn read_pointer(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(raw) => Ok(Some(raw.trim_end_matches(['\n', '\r']).to_owned())),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(source.into()),
    }
}

/// Menulis pointer secara atomik: temp file + rename di direktori yang sama.
/// KONTEKS: `fs::write` langsung bisa meninggalkan pointer kosong bila proses
/// mati di tengah menulis; ALTERNATIF: berkas `.tmp-` yang tertinggal saat rename
/// gagal tidak pernah dibaca engine dan ditimpa `stage` berikutnya.
fn write_pointer(path: &Path, hex: &str) -> Result<()> {
    let Some(parent) = path.parent() else {
        return Err(VergeError::Io(std::io::Error::other("tanpa induk")));
    };
    fs::create_dir_all(parent)?;
    let temp = parent.join(format!("{hex}.tmp-{}", std::process::id()));
    fs::write(&temp, format!("{hex}\n")).and_then(|()| fs::rename(&temp, path))?;
    Ok(())
}
