//! File: `commit_encoding.rs`
//!
//! Deskripsi: Encoding kanonik byte untuk entitas commit.
//! Layer: domain/commit/entities
//! Tanggung jawab: Menghasilkan byte deterministik yang di-hash jadi `CommitId`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/ident/value-objects/digest.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use crate::domain::ident::value_objects::digest::{Digest, DIGEST_LEN};

/// Prefix domain-separation agar digest commit tidak pernah sama dengan digest
/// blok yang kebetulan berisi byte serupa.
const ENCODING_TAG: &[u8] = b"verge-commit-v1";

/// Bagian dari commit yang ikut di-encode.
#[derive(Debug)]
pub struct CommitFields<'a> {
    /// Parent commit, urut dengan first-parent di depan.
    pub parents: &'a [Digest],
    /// Root block snapshot yang ditunjuk commit.
    pub tree: Digest,
    /// Author yang tercatat pada commit.
    pub author: &'a str,
    /// Pesan commit lengkap.
    pub message: &'a str,
    /// Waktu commit dalam milidetik sejak epoch Unix.
    pub timestamp_unix_ms: i64,
}

/// Mengencode bagian commit menjadi byte kanonik.
///
/// Setiap field variabel panjang diberi prefiks panjang (u64 little-endian)
/// sehingga dua commit berbeda tidak pernah menghasilkan byte yang sama.
///
/// Returns:
/// - Vec<u8> — encoding deterministik, stabil lintas platform.
///
/// Performance: satu alokasi; linear terhadap panjang pesan dan parent.
#[must_use]
pub fn encode(fields: &CommitFields<'_>) -> Vec<u8> {
    let capacity = ENCODING_TAG.len()
        + 8
        + fields.parents.len() * DIGEST_LEN
        + DIGEST_LEN
        + fields.author.len()
        + fields.message.len()
        + 8;
    let mut out = Vec::with_capacity(capacity);
    out.extend_from_slice(ENCODING_TAG);
    out.extend_from_slice(&(fields.parents.len() as u64).to_le_bytes());
    for parent in fields.parents {
        out.extend_from_slice(parent.as_bytes());
    }
    out.extend_from_slice(fields.tree.as_bytes());
    push_field(&mut out, fields.author.as_bytes());
    push_field(&mut out, fields.message.as_bytes());
    out.extend_from_slice(&fields.timestamp_unix_ms.to_le_bytes());
    out
}

/// Menambahkan field berpindah prefiks panjang ke `out`.
fn push_field(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    out.extend_from_slice(bytes);
}
