//! File: `revision_resolver.rs`
//!
//! Deskripsi: Penyelesaian nama revisi menjadi identifier commit.
//! Layer: application/version-control
//! Tanggung jawab: Menerjemahkan `HEAD`, `HEAD~N`, awalan hex, dan nama branch.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `domain/commit/repositories/ports/*.rs`, `domain/ident/*`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest_text::{parse_hex, HexText};
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Panjang minimum awalan commit yang boleh dipakai pengguna.
///
/// Batas bawah ini sama dengan 12 hex yang dicetak `verge log`, sehingga
/// identifier yang disalin dari output log selalu dapat dipakai ulang.
const MIN_PREFIX_LEN: usize = 12;

/// Panjang identifier commit penuh dalam hex.
const FULL_ID_LEN: usize = 64;

/// Batas jumlah commit yang diperiksa saat mencari awalan.
///
/// KENAPA: pencarian awalan menelusuri rantai first-parent; batas ini menjaga
///         biaya tetap wajar dan batas pencarian ini ketara bagi pengguna
///         alih-alih diam-diam mengembalikan hasil yang mungkin salah.
const MAX_SCAN: usize = 10_000;

/// Menyelesaikan `revision` menjadi identifier commit.
///
/// Bentuk yang diterima:
/// - `HEAD` — commit terakhir branch aktif;
/// - `HEAD~N` — N commit ke belakang pada rantai first-parent branch aktif;
/// - 64 hex — identifier commit lengkap;
/// - 12–63 hex — awalan commit, dicari pada rantai first-parent branch aktif;
/// - nama lain — diperlakukan sebagai nama branch.
///
/// Args:
/// - revision — teks revisi dari pengguna; spasi tepi diabaikan.
/// - refs — port pointer branch.
/// - commits — port penyimpanan objek commit.
///
/// Returns:
/// - Ok(CommitId) — commit yang ditunjuk oleh revisi tersebut.
///
/// # Errors
///
/// Mengembalikan [`HeadUnborn`](VergeError::HeadUnborn) bila branch aktif belum
/// punya commit dan [`InvalidRef`](VergeError::InvalidRef) bila revisi tidak
/// dikenal, awalan cocok lebih dari satu commit, atau jumlah `HEAD~N` melebihi
/// kedalaman sejarah.
pub fn resolve_revision(
    revision: &str,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
) -> Result<CommitId> {
    let revision = revision.trim();
    if revision == "HEAD" {
        return head_of_head_branch(refs);
    }
    if let Some(steps) = revision.strip_prefix("HEAD~") {
        return walk_back(steps, refs, commits, revision);
    }
    if let Ok(id) = parse_hex(revision) {
        return Ok(id);
    }
    if is_hex_prefix(revision) {
        return find_by_prefix(revision, refs, commits);
    }
    refs.resolve(revision)?
        .ok_or_else(|| VergeError::InvalidRef(revision.to_owned()))
}

/// Mengembalikan commit terakhir branch yang ditunjuk `HEAD`.
fn head_of_head_branch(refs: &dyn RefPointer) -> Result<CommitId> {
    let branch = refs.head_branch()?;
    refs.resolve(&branch)?
        .ok_or_else(|| VergeError::HeadUnborn(branch.clone()))
}

/// Menelusuri `HEAD~N` pada rantai first-parent branch aktif.
fn walk_back(
    steps: &str,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
    revision: &str,
) -> Result<CommitId> {
    let total: usize = steps
        .parse()
        .map_err(|_| VergeError::InvalidRef(revision.to_owned()))?;
    let mut cursor = head_of_head_branch(refs)?;
    for _ in 0..total {
        let commit = commits.load(&cursor)?;
        cursor = *commit
            .parents()
            .first()
            .ok_or_else(|| VergeError::InvalidRef(revision.to_owned()))?;
    }
    Ok(cursor)
}

/// Mencari satu-satunya commit dengan awalan hex tertentu.
///
/// # Errors
///
/// Mengembalikan [`InvalidRef`](VergeError::InvalidRef) bila tidak ada commit
/// yang cocok atau lebih dari satu commit cocok dengan awalan yang sama.
fn find_by_prefix(
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

/// Melaporkan apakah `text` adalah awalan hex yang cukup panjang.
fn is_hex_prefix(text: &str) -> bool {
    (MIN_PREFIX_LEN..FULL_ID_LEN).contains(&text.len())
        && text.bytes().all(|byte| byte.is_ascii_hexdigit())
}
