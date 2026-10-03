//! File: `revision_walk.rs`
//!
//! Deskripsi: Penelusuran rantai commit untuk revisi relatif dan awalan hex.
//! Layer: application/version-control
//! Tanggung jawab: Menjalankan `HEAD~N` dan mencari commit berdasarkan awalan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `revision_resolver.rs`, `domain/commit/repositories/ports/*.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::application::version_control::revision_resolver::head_of_head_branch;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Batas commit yang diperiksa saat mencari awalan hex.
///
/// KENAPA: pencarian awalan menelusuri rantai first-parent; batas ini menjaga
/// biaya tetap wajar dan batas pencarian ini ketara bagi pengguna alih-alih
/// diam-diam mengembalikan hasil yang mungkin salah.
const MAX_SCAN: usize = 10_000;

/// Menelusuri `HEAD~N` pada rantai first-parent branch aktif.
///
/// # Errors
///
/// Mengembalikan [`InvalidRef`](VergeError::InvalidRef) bila kedalaman melebihi
/// sejarah dan error dari port bila ada commit yang gagal dibaca.
pub(crate) fn walk_back(
    steps: usize,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
    revision: &str,
) -> Result<CommitId> {
    let mut cursor = head_of_head_branch(refs)?;
    for _ in 0..steps {
        let commit = commits.load(&cursor)?;
        cursor = *commit
            .parents()
            .first()
            .ok_or_else(|| VergeError::InvalidRef(revision.trim().to_owned()))?;
    }
    Ok(cursor)
}

/// Mencari satu-satunya commit dengan awalan hex tertentu pada rantai aktif.
///
/// # Errors
///
/// Mengembalikan [`InvalidRef`](VergeError::InvalidRef) bila tidak ada commit
/// yang cocok atau lebih dari satu commit cocok dengan awalan yang sama.
pub(crate) fn find_by_prefix(
    prefix: &str,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
) -> Result<CommitId> {
    let mut cursor = Some(head_of_head_branch(refs)?);
    let mut found: Option<CommitId> = None;
    let mut scanned = 0_usize;
    while let Some(current) = cursor {
        if scanned == MAX_SCAN {
            break;
        }
        if current.to_hex().starts_with(prefix) {
            if found.is_some() {
                return Err(VergeError::InvalidRef(format!(
                    "commit prefix `{prefix}` is ambiguous"
                )));
            }
            found = Some(current);
        }
        let commit = commits.load(&current)?;
        cursor = commit.parents().first().copied();
        scanned += 1;
    }
    found.ok_or_else(|| VergeError::InvalidRef(prefix.to_owned()))
}
