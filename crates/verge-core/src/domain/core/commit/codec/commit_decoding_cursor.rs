//! File: `commit_decoding_cursor.rs`
//!
//! Deskripsi: Kursor baca bounds-checked untuk byte commit.
//! Layer: domain/commit/codec
//! Tanggung jawab: Menjamin decoding tidak pernah membaca melewati batas byte.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/codec/commit_decoding.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use crate::domain::commit::codec::commit_encoding::ENCODING_TAG;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest::{Digest, DIGEST_LEN};
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Batas jumlah parent satu commit; mencegah alokasi berlebihan dari byte rusak.
const MAX_PARENTS: usize = 4096;

/// Kursor baca di atas byte commit.
///
/// Invariants: `position` tidak pernah melewati `bytes.len()`.
#[derive(Debug)]
pub(super) struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    /// Membuat kursor di awal `bytes`.
    pub(super) const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    /// Mengambil `len` byte berikutnya.
    ///
    /// # Errors
    ///
    /// Error bila byte tidak cukup.
    pub(super) fn take(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self
            .position
            .checked_add(len)
            .filter(|end| *end <= self.bytes.len())
            .ok_or(VergeError::MalformedCommit {
                reason: "truncated commit object",
            })?;
        let slice = &self.bytes[self.position..end];
        self.position = end;
        Ok(slice)
    }

    /// Mengembalikan jumlah byte yang belum dibaca.
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }

    /// Membaca daftar parent beserta jumlahnya.
    ///
    /// # Errors
    ///
    /// Error bila tag salah, jumlah parent di luar batas, atau byte kurang.
    pub(super) fn read_parents(&mut self) -> Result<Vec<CommitId>> {
        if self.take(ENCODING_TAG.len())? != ENCODING_TAG {
            return Err(VergeError::MalformedCommit {
                reason: "unknown commit encoding tag",
            });
        }
        let count = usize::try_from(self.read_i64()?)
            .ok()
            .filter(|count| *count <= MAX_PARENTS)
            .ok_or(VergeError::MalformedCommit {
                reason: "parent count out of range",
            })?;
        let mut parents = Vec::with_capacity(count);
        for _ in 0..count {
            parents.push(self.read_digest()?);
        }
        Ok(parents)
    }

    /// Membaca satu digest 32 byte.
    ///
    /// # Errors
    ///
    /// Error bila byte tidak cukup.
    pub(super) fn read_digest(&mut self) -> Result<Digest> {
        let mut raw = [0_u8; DIGEST_LEN];
        raw.copy_from_slice(self.take(DIGEST_LEN)?);
        Ok(Digest::from_bytes(raw))
    }

    /// Membaca field teks berpindah prefiks panjang.
    ///
    /// # Errors
    ///
    /// Error bila panjang di luar batas atau byte bukan utf-8.
    pub(super) fn read_text(&mut self) -> Result<String> {
        let len = usize::try_from(self.read_i64()?)
            .ok()
            .filter(|len| *len <= self.remaining())
            .ok_or(VergeError::MalformedCommit {
                reason: "text field length out of range",
            })?;
        let raw = self.take(len)?;
        String::from_utf8(raw.to_vec()).map_err(|_| VergeError::MalformedCommit {
            reason: "text field is not valid utf-8",
        })
    }

    /// Membaca nama tabel yang tervalidasi.
    ///
    /// # Errors
    ///
    /// Error bila nama tabel di luar allowlist.
    pub(super) fn read_table_name(&mut self) -> Result<TableName> {
        let raw = self.read_text()?;
        TableName::parse(&raw).map_err(|_| VergeError::MalformedCommit {
            reason: "table name is not valid",
        })
    }

    /// Membaca integer 64-bit little-endian.
    ///
    /// # Errors
    ///
    /// Mengembalikan error bila byte tidak cukup.
    pub(super) fn read_timestamp(&mut self) -> Result<i64> {
        self.read_i64()
    }

    /// Membaca integer 64-bit little-endian; error bila byte tidak cukup.
    fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0_u8; 8];
        buf.copy_from_slice(self.take(8)?);
        Ok(i64::from_le_bytes(buf))
    }
}
