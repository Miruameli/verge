//! File: `verge_error.rs`
//!
//! Deskripsi: Satu-satunya tipe error engine.
//! Layer: shared/exceptions
//! Tanggung jawab: `Meng_enumasi` kegagalan yang mungkin terjadi pada engine.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `shared/exceptions/parse_digest_error.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::path::PathBuf;

#[path = "verge_error_display.rs"]
mod display;

use crate::domain::ident::value_objects::digest::Digest;
use crate::shared::exceptions::parse_digest_error::ParseDigestError;

/// Semua kegagalan yang dapat terjadi di dalam engine.
#[derive(Debug)]
#[non_exhaustive]
pub enum VergeError {
    /// Operasi filesystem atau I/O gagal.
    Io(std::io::Error),
    /// Blok yang diminta tidak ada di store.
    BlockNotFound {
        /// Identifier blok yang hilang.
        id: Digest,
        /// Kegagalan I/O asli.
        source: std::io::Error,
    },
    /// Commit yang diminta tidak ada di graph.
    CommitNotFound(Digest),
    /// Commit dimasukkan sebelum parent-nya dikenal.
    MissingParent(Digest),
    /// Nama branch/tag kosong, diawali titik, atau berisi path separator.
    InvalidName(String),
    /// Tag dengan nama tersebut sudah ada; tag bersifat immutable.
    TagAlreadyExists(String),
    /// Referensi gagal di-parse.
    InvalidRef(String),
    /// String digest gagal di-parse.
    InvalidDigest(ParseDigestError),
    /// Repository sudah ada; bootstrap tidak menimpa data pengguna.
    RepositoryAlreadyExists(PathBuf),
    /// Folder kerja bukan repository Verge.
    NotARepository(PathBuf),
    /// Byte commit tidak kanonik atau rusak sehingga tidak dapat dipercaya.
    MalformedCommit {
        /// Penyebab ringkas tanpa detail internal.
        reason: &'static str,
    },
    /// Nama tabel di luar allowlist.
    InvalidTableName(String),
    /// Tabel belum punya data kerja untuk di-commit.
    TableNotStaged(String),
    /// Branch aktif belum memiliki commit pertama.
    HeadUnborn(String),
    /// Isi berkas pointer bukan digest yang valid.
    MalformedPointer(String),
    /// Commit ditolak karena data tabel tidak berubah sejak commit terakhir.
    NothingToCommit(String),
    /// Node tree tidak kanonik atau rusak saat dibaca.
    MalformedTreeNode {
        /// Penyebab ringkas yang aman ditampilkan ke pengguna.
        reason: &'static str,
    },
    /// Tabel tidak memiliki baris sehingga tree tidak punya akar.
    EmptyTable,
    /// Baris tabel tidak dapat diurai.
    MalformedTable {
        /// Nomor baris satu-based seperti terlihat di file.
        line: usize,
        /// Penyebab ringkas yang aman ditampilkan ke pengguna.
        reason: &'static str,
    },
    /// Metadata commit (penulis atau pesan) tidak memenuhi aturan.
    InvalidCommitField {
        /// Nama field yang bermasalah.
        field: &'static str,
        /// Alasan penolakan yang aman ditampilkan ke pengguna.
        detail: &'static str,
    },
}

impl std::error::Error for VergeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(source) | Self::BlockNotFound { source, .. } => Some(source),
            Self::InvalidDigest(source) => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for VergeError {
    fn from(source: std::io::Error) -> Self {
        Self::Io(source)
    }
}

impl From<ParseDigestError> for VergeError {
    fn from(source: ParseDigestError) -> Self {
        Self::InvalidDigest(source)
    }
}
