//! File: `revision_resolver.rs`
//!
//! Deskripsi: Penyelesaian nama revisi menjadi identifier commit.
//! Layer: application/version-control
//! Tanggung jawab: Menerjemahkan bentuk revisi yang sudah diklasifikasi menjadi
//!   commit melalui port ref, tag, dan commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `revision_target.rs`, `instant_commit_lookup.rs`, `domain/commit/repositories/ports/*.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::application::version_control::instant_commit_lookup::{
    find_commit_at_or_before, InstantLookup,
};
use crate::application::version_control::revision_target::{self, RevisionTarget};
use crate::application::version_control::revision_walk::{find_by_prefix, walk_back};
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::time::value_objects::timestamp::Timestamp;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Menyelesaikan `revision` menjadi identifier commit.
///
/// Bentuk yang diterima dijelaskan pada [`revision_target::classify`]; nama
/// yang tidak diawali `refs/` dicoba sebagai branch lalu sebagai tag.
///
/// Args:
/// - revision — teks revisi dari pengguna; spasi tepi diabaikan.
/// - refs — port pointer branch.
/// - tags — port pointer tag.
/// - commits — port penyimpanan objek commit.
/// - table — tabel yang ditanyakan; [`None`] untuk pemanggil yang tidak
///   menanyakan waktu, misalnya pembuatan tag.
///
/// Returns:
/// - Ok(CommitId) — commit yang ditunjuk oleh revisi tersebut.
///
/// # Errors
///
/// Mengembalikan [`HeadUnborn`](VergeError::HeadUnborn) bila branch aktif belum
/// punya commit, [`InvalidRef`](VergeError::InvalidRef) bila revisi tidak
/// dikenal atau ambigu, [`InvalidTimestamp`](VergeError::InvalidTimestamp) untuk
/// teks waktu yang tidak valid, dan
/// [`NoCommitAtInstant`](VergeError::NoCommitAtInstant) bila tidak ada commit
/// tabel tersebut pada waktu yang diminta.
pub fn resolve_revision(
    revision: &str,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
    table: Option<&TableName>,
) -> Result<CommitId> {
    match revision_target::classify(revision)? {
        RevisionTarget::Head => head_of_head_branch(refs),
        RevisionTarget::WalkBack(steps) => walk_back(steps, refs, commits, revision),
        RevisionTarget::Commit(id) => Ok(id),
        RevisionTarget::Prefix(prefix) => find_by_prefix(&prefix, refs, commits),
        RevisionTarget::Branch(name) => resolve_branch(&name, refs, tags),
        RevisionTarget::Tag(name) => resolve_tag(&name, tags),
        RevisionTarget::Instant(at) => {
            let table = table.ok_or_else(|| VergeError::InvalidRef(revision.trim().to_owned()))?;
            resolve_instant(at, refs, commits, table)
        }
    }
}

/// Mengembalikan commit terakhir branch yang ditunjuk `HEAD`.
///
/// # Errors
///
/// Mengembalikan [`HeadUnborn`](VergeError::HeadUnborn) bila branch aktif belum
/// punya commit dan error I/O bila pointer tidak dapat dibaca.
pub(crate) fn head_of_head_branch(refs: &dyn RefPointer) -> Result<CommitId> {
    let branch = refs.head_branch()?;
    refs.resolve(&branch)?
        .ok_or_else(|| VergeError::HeadUnborn(branch.clone()))
}

/// Menyelesaikan nama branch, dengan tag sebagai cadangan.
///
/// # Errors
///
/// Mengembalikan [`InvalidRef`](VergeError::InvalidRef) bila nama dikenal
/// bukan sebagai branch maupun sebagai tag.
fn resolve_branch(name: &str, refs: &dyn RefPointer, tags: &dyn TagPointer) -> Result<CommitId> {
    if let Some(id) = refs.resolve(name)? {
        return Ok(id);
    }
    tags.resolve(name)?
        .ok_or_else(|| VergeError::InvalidRef(name.to_owned()))
}

/// Menyelesaikan nama tag secara eksplisit.
///
/// # Errors
///
/// Mengembalikan [`UnknownTag`](VergeError::UnknownTag) bila tag belum ada.
fn resolve_tag(name: &str, tags: &dyn TagPointer) -> Result<CommitId> {
    tags.resolve(name)?
        .ok_or_else(|| VergeError::UnknownTag(name.to_owned()))
}

/// Menyelesaikan waktu menjadi commit terbaru pada atau sebelum waktu itu.
///
/// # Errors
///
/// Mengembalikan error dari port dan
/// [`NoCommitAtInstant`](VergeError::NoCommitAtInstant) bila tidak ada commit
/// tabel tersebut pada waktu yang diminta.
fn resolve_instant(
    at: Timestamp,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
    table: &TableName,
) -> Result<CommitId> {
    let head = head_of_head_branch(refs)?;
    match find_commit_at_or_before(head, table, at, commits)? {
        InstantLookup::Found { commit, .. } => Ok(commit),
    }
}
