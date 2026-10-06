//! File: `revision_target.rs`
//!
//! Deskripsi: Klasifikasi teks revisi menjadi bentuk yang dikenali.
//! Layer: application/version-control
//! Tanggung jawab: Memisahkan bentuk revisi dari penyebutan port, sehingga
//!   klasifikasi dapat diuji tanpa storage.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/ident/value_objects/parse_hex_error.rs`, `domain/time/value_objects/timestamp.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::ident::value_objects::digest_text::parse_hex;
use crate::domain::time::value_objects::timestamp::Timestamp;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Panjang minimum awalan commit yang boleh dipakai pengguna.
///
/// Batas bawah ini sama dengan 12 hex yang dicetak `verge log`, sehingga
/// identifier yang disalin dari output log selalu dapat dipakai ulang.
const MIN_PREFIX_LEN: usize = 12;

/// Panjang identifier commit penuh dalam hex.
const FULL_ID_LEN: usize = 64;

/// Awalan yang menandai referensi tag secara eksplisit.
const TAG_PREFIX: &str = "refs/tags/";

/// Awalan yang menandai referensi branch secara eksplisit.
const BRANCH_PREFIX: &str = "refs/heads/";

/// Awalan unix milidetik yang dipakai `Timestamp::parse`.
const UNIX_PREFIX: char = '@';

/// Panjang tanggal `YYYY-MM-DD` pada RFC 3339.
const CIVIL_DATE_LEN: usize = 10;

/// Bentuk revisi yang sudah diklasifikasi, belum menyentuh storage.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevisionTarget {
    /// Commit terakhir branch aktif.
    Head,
    /// N langkah ke belakang pada rantai first-parent branch aktif.
    WalkBack(usize),
    /// Identifier commit lengkap.
    Commit(Digest),
    /// Awalan hex commit yang harus unik.
    Prefix(String),
    /// Nama branch.
    Branch(String),
    /// Nama tag.
    Tag(String),
    /// Waktu UTC untuk time-travel.
    Instant(Timestamp),
}

/// Mengklasifikasi `text` menjadi bentuk revisi yang dikenal.
///
/// Bentuk yang diterima: `HEAD`, `HEAD~N`, `refs/tags/<nama>`, `refs/heads/<nama>`,
/// 64 hex, 12–63 hex, RFC 3339 UTC, `@<unix_ms>`, dan nama lain sebagai branch.
///
/// Returns:
/// - `RevisionTarget` — klasifikasi; nama branch dan tag belum dibedakan, karena
///   keduanya sama-sama nama pointer dan hanya dapat dibedakan dengan storage.
///
/// # Errors
///
/// Mengembalikan [`InvalidRef`](VergeError::InvalidRef) untuk `HEAD~x` dengan
/// langkah bukan angka, dan [`InvalidTimestamp`](VergeError::InvalidTimestamp)
/// untuk teks yang menyerupai waktu namun tidak valid.
pub fn classify(text: &str) -> Result<RevisionTarget> {
    let revision = text.trim();
    if revision == "HEAD" {
        return Ok(RevisionTarget::Head);
    }
    if let Some(steps) = revision.strip_prefix("HEAD~") {
        let total = steps
            .parse()
            .map_err(|_| VergeError::InvalidRef(revision.to_owned()))?;
        return Ok(RevisionTarget::WalkBack(total));
    }
    if let Some(name) = revision.strip_prefix(TAG_PREFIX) {
        return Ok(RevisionTarget::Tag(name.to_owned()));
    }
    if let Some(name) = revision.strip_prefix(BRANCH_PREFIX) {
        return Ok(RevisionTarget::Branch(name.to_owned()));
    }
    if let Ok(id) = parse_hex(revision) {
        return Ok(RevisionTarget::Commit(id));
    }
    if is_hex_prefix(revision) {
        return Ok(RevisionTarget::Prefix(revision.to_owned()));
    }
    if looks_like_time(revision) {
        return Ok(RevisionTarget::Instant(Timestamp::parse(revision)?));
    }
    Ok(RevisionTarget::Branch(revision.to_owned()))
}

/// Melaporkan apakah `text` adalah awalan hex yang cukup panjang.
fn is_hex_prefix(text: &str) -> bool {
    (MIN_PREFIX_LEN..FULL_ID_LEN).contains(&text.len())
        && text.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// Melaporkan apakah `text` dituliskan sebagai waktu.
///
/// KENAPA pemeriksaan bentuk ada sebelum parse: nama branch seperti
/// `2026-q1-report` sah sebagai nama berkas dan tidak boleh dialihkan ke parser
/// waktu. Hanya dua bentuk yang diperiksa: prefiks `@` dan awalan tanggal
/// `YYYY-MM-DD`. Teks berbentuk tanggal yang tidak lengkap — misalnya
/// `2026-10-01` — memang ditolak, dan pesan galatnya menyebut bentuk yang
/// benar; nama pointer seperti itu tetap dapat disebut lewat `refs/heads/<nama>`.
fn looks_like_time(text: &str) -> bool {
    text.starts_with(UNIX_PREFIX) || starts_with_civil_date(text)
}

/// Melaporkan apakah `text` diawali `YYYY-MM-DD`.
fn starts_with_civil_date(text: &str) -> bool {
    let Some(date) = text.as_bytes().get(..CIVIL_DATE_LEN) else {
        return false;
    };
    date.iter().enumerate().all(|(index, byte)| match index {
        4 | 7 => *byte == b'-',
        _ => byte.is_ascii_digit(),
    })
}
