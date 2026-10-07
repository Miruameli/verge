//! File: `verge_error_display.rs`
//!
//! Deskripsi: Format pesan `VergeError` untuk pengguna.
//! Layer: shared/exceptions
//! Tanggung jawab: Menjaga pesan error ringkas dan tanpa detail internal.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `verge_error.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #18 (Milestone 3)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

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
            Self::UnknownTag(name) => write!(f, "tag `{name}` does not exist"),
            Self::TagCannotPointAtInstant(text) => write!(
                f,
                "a tag cannot point at a timestamp (`{text}`); point it at a commit, branch, or existing tag"
            ),
            Self::InvalidTimestamp(text) => write!(
                f,
                "invalid timestamp `{text}`; use RFC 3339 UTC (2026-10-01T10:00:00Z) or unix milliseconds (@1767225600000)"
            ),
            Self::NoCommitAtInstant { requested, oldest } => write!(
                f,
                "no commit at or before {requested}; oldest commit is {oldest}"
            ),
            Self::SearchLimitReached {
                requested,
                boundary,
            } => write!(
                f,
                "search for {requested} stopped after the scan limit at {boundary}; use a branch or tag name instead"
            ),
            Self::InvalidRef(text) => write!(f, "invalid reference `{text}`"),
            Self::CommitBelongsToOtherTable {
                revision,
                commit_table,
                requested,
            } => write!(
                f,
                "reference `{revision}` points to table `{commit_table}`, not `{requested}`"
            ),
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
            Self::UnknownBranch(name) => write!(f, "branch `{name}` does not exist"),
            Self::BranchInUse(name) => write!(
                f,
                "branch `{name}` is the current branch; switch away before deleting it"
            ),
            Self::BranchAlreadyExists(name) => write!(f, "branch `{name}` already exists"),
            Self::NothingToMerge(name) => write!(f, "branch `{name}` has no commits to merge"),
            Self::AlreadyMerged(name) => {
                write!(
                    f,
                    "branch `{name}` is already merged into the current branch"
                )
            }
            Self::UnrelatedHistories(name) => {
                write!(
                    f,
                    "branch `{name}` shares no common ancestor with the current branch"
                )
            }
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
            Self::SqlParse { offset, message } => {
                write!(f, "SQL query error at byte {offset}: {message}")
            }
        }
    }
}
