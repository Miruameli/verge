//! File: `commit_decoding.rs`
//!
//! Deskripsi: Pembacaan byte kanonik commit menjadi entitas commit.
//! Layer: domain/commit/codec
//! Tanggung jawab: Menolak byte commit yang rusak atau sudah dimanipulasi.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_decoding_cursor.rs`, `commit_encoding.rs`
//!   - `domain/commit/entities/commit.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

// Kursor dan pembaca field hidup di modul anak agar berkas ini fokus pada
// aturan integritas, bukan pada parsing byte per byte.
#[path = "commit_decoding_cursor.rs"]
mod cursor;

use crate::domain::commit::codec::commit_decoding::cursor::Cursor;
use crate::domain::commit::entities::commit::Commit;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Membaca byte kanonik `bytes` menjadi [`Commit`].
///
/// Args:
/// - bytes — byte hasil `Commit::encode` yang diambil dari storage.
///
/// Returns:
/// - Ok(Commit) — commit dengan digest yang cocok dengan byte yang dibaca.
///
/// # Errors
///
/// Mengembalikan [`MalformedCommit`](VergeError::MalformedCommit) bila panjang
/// byte tidak sesuai, tag salah, atau re-encoding tidak menghasilkan byte yang
/// sama. Pemeriksaan terakhir inilah yang menolak byte yang dimanipulasi.
///
/// Example:
/// ```
/// use verge_core::domain::commit::codec::commit_decoding::decode;
/// use verge_core::{Commit, Digest, TableName};
///
/// let table = TableName::parse("users").unwrap();
/// let commit = Commit::new(vec![], Digest::of(b"tree"), &table, "ana", "feat: seed", 7);
/// assert_eq!(decode(&commit.encode()).unwrap(), commit);
/// assert!(decode(b"bukan commit").is_err());
/// ```
pub fn decode(bytes: &[u8]) -> Result<Commit> {
    let mut cursor = Cursor::new(bytes);
    let parents = cursor.read_parents()?;
    let tree = cursor.read_digest()?;
    let table = cursor.read_table_name()?;
    let author = cursor.read_text()?;
    let message = cursor.read_text()?;
    let timestamp = cursor.read_timestamp()?;

    let commit = Commit::new(parents, tree, &table, &author, &message, timestamp);
    if commit.encode() == bytes {
        Ok(commit)
    } else {
        Err(VergeError::MalformedCommit {
            reason: "byte does not match the canonical encoding of its fields",
        })
    }
}
