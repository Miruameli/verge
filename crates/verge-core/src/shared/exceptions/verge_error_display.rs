//! File: `verge_error_display.rs`
//!
//! Deskripsi: Format pesan `VergeError` untuk pengguna.
//! Layer: shared/exceptions
//! Tanggung jawab: Menjaga pesan error ringkas dan tanpa detail internal.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `verge_error.rs`
//! Related issues: #1 (Milestone 1), #18 (Milestone 3)
//! Related ADR: ADR-0002 (Storage immutable content-addressed)

use core::fmt;

use crate::shared::exceptions::verge_error::VergeError;

// KENAPA: pesan tidak boleh membocorkan detail internal, dan setiap varian
//         punya bentuk kalimat yang konsisten untuk pengguna.
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
            Self::NotARepository(path) => write!(
                f,
                "no Verge repository at {}; run `verge init` first",
                path.display()
            ),
            Self::MalformedCommit { reason } => {
                write!(f, "commit object is not canonical: {reason}")
            }
            Self::InvalidTableName(name) => write!(f, "invalid table name `{name}`"),
            Self::TableNotStaged(name) => {
                write!(
                    f,
                    "table `{name}` has no staged data; run `verge import` first"
                )
            }
            Self::HeadUnborn(name) => write!(f, "branch `{name}` has no commits yet"),
            Self::MalformedPointer(text) => write!(f, "malformed reference pointer: {text}"),
            Self::MalformedTreeNode { reason } => {
                write!(f, "tree node is not canonical: {reason}")
            }
            Self::EmptyTable => write!(f, "table has no rows to version"),
            Self::MalformedTable { line, reason } => {
                write!(f, "table line {line} is malformed: {reason}")
            }
            Self::NothingToCommit(name) => {
                write!(f, "table `{name}` is unchanged since the last commit")
            }
            Self::InvalidCommitField { field, detail } => {
                write!(f, "invalid commit {field}: {detail}")
            }
        }
    }
}
