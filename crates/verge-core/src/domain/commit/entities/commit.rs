//! File: commit.rs
//!
//! Deskripsi: Entitas commit sebagai snapshot versioned yang immutable.
//! Layer: domain/commit/entities
//! Tanggung jawab: Menyusun commit dan menentukan identifier-nya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/codec/commit_encoding.rs`
//!   - `domain/table/value-objects/table_name.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use crate::domain::commit::codec::commit_encoding::{encode, CommitFields};
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::value_objects::table_name::TableName;

// Pembaca field hidup di modul anak agar berkas ini tetap di bawah batas 150 baris
// tanpa melonggarkan visibilitas field yang sengaja private.
#[path = "commit_fields.rs"]
mod fields;

/// Snapshot versioned yang immutable beserta parent, tabel, dan metadatanya.
///
/// Invariants:
/// - `id` selalu sama dengan digest dari [`Commit::encode`] sehingga commit
///   dapat diverifikasi tanpa mempercayai storage.
/// - Tidak ada mutator: perubahan data berarti commit baru.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// Identifier hasil hashing encoding kanonik.
    id: CommitId,
    /// Parent commit, first-parent di depan.
    parents: Vec<CommitId>,
    /// Root block snapshot yang ditunjuk commit ini.
    tree: BlockId,
    /// Nama tabel yang di-versioning commit ini.
    table: TableName,
    /// Author yang tercatat.
    author: String,
    /// Pesan commit lengkap.
    message: String,
    /// Waktu commit dalam milidetik sejak epoch Unix.
    timestamp_unix_ms: i64,
}

impl Commit {
    /// Membuat commit dan menghitung identifier dari encoding kanoniknya.
    ///
    /// Args:
    /// - parents — daftar parent; elemen pertama adalah branch asal commit.
    /// - tree — root block snapshot tabel.
    /// - table — tabel yang di-versioning.
    /// - author, message — metadata commit.
    /// - `timestamp_unix_ms` — waktu commit dalam milidetik sejak epoch Unix.
    ///
    /// Returns:
    /// - Commit — commit baru dengan `id` yang sudah terhitung.
    ///
    /// Example:
    /// ```
    /// use verge_core::{Commit, Digest, TableName};
    ///
    /// let table = TableName::parse("users").unwrap();
    /// let commit = Commit::new(vec![], Digest::of(b"tree"), &table, "ana", "feat: seed", 0);
    /// assert_eq!(commit.id(), Digest::of(&commit.encode()));
    /// assert_eq!(commit.summary(), "feat: seed");
    /// ```
    #[must_use]
    pub fn new(
        parents: Vec<CommitId>,
        tree: BlockId,
        table: &TableName,
        author: impl Into<String>,
        message: impl Into<String>,
        timestamp_unix_ms: i64,
    ) -> Self {
        let mut commit = Self {
            id: CommitId::zeroed(),
            parents,
            tree,
            table: table.clone(),
            author: author.into(),
            message: message.into(),
            timestamp_unix_ms,
        };
        commit.id = Digest::of(&commit.encode());
        commit
    }

    /// Mengembalikan identifier content-addressed commit.
    #[must_use]
    pub fn id(&self) -> CommitId {
        self.id
    }

    /// Mengembalikan encoding kanonik commit.
    ///
    /// Returns:
    /// - Vec<u8> — byte deterministik yang di-hash menjadi identifier commit.
    ///
    /// Performance: satu alokasi, linear terhadap panjang pesan.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        encode(&CommitFields {
            parents: &self.parents,
            tree: self.tree,
            table: self.table.as_str(),
            author: &self.author,
            message: &self.message,
            timestamp_unix_ms: self.timestamp_unix_ms,
        })
    }
}
