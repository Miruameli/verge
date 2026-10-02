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

use core::fmt;

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
}

impl fmt::Display for VergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(source) | Self::BlockNotFound { source, .. } => {
                write!(f, "i/o error: {source}")
            }
            Self::CommitNotFound(id) => write!(f, "commit {id} not found"),
            Self::MissingParent(id) => write!(f, "parent commit {id} is unknown to the graph"),
            Self::InvalidName(name) => write!(f, "invalid reference name `{name}`"),
            Self::TagAlreadyExists(name) => write!(f, "tag `{name}` already exists"),
            Self::InvalidRef(text) => write!(f, "invalid reference `{text}`"),
            Self::InvalidDigest(source) => source.fmt(f),
            Self::RepositoryAlreadyExists(path) => {
                write!(f, "a Verge repository already exists at {}", path.display())
            }
        }
    }
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
