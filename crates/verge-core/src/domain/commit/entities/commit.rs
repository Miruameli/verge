//! File: commit.rs
//!
//! Deskripsi: Entitas `Commit` — unit of versioning Verge.
//! Layer: domain/commit/entities
//! Tanggung jawab: Menyimpan snapshot, parent, dan metadata commit secara immutable.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/entities/commit_encoding.rs`
//!   - domain/ident/value-objects/digest.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use crate::domain::commit::entities::commit_encoding::{encode, CommitFields};
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::storage::value_objects::block_id::BlockId;

/// Snapshot versioned yang immutable beserta parent dan metadatanya.
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
    /// - tree — root block snapshot.
    /// - author, message — metadata commit.
    /// - `timestamp_unix_ms` — waktu commit dalam milidetik sejak epoch Unix.
    ///
    /// Returns:
    /// - Commit — commit baru dengan `id` yang sudah terhitung.
    ///
    /// Example:
    /// ```
    /// use verge_core::{Commit, Digest};
    ///
    /// let commit = Commit::new(vec![], Digest::of(b"tree"), "ana", "feat: seed", 0);
    /// assert_eq!(commit.id(), Digest::of(&commit.encode()));
    /// assert_eq!(commit.summary(), "feat: seed");
    /// ```
    #[must_use]
    pub fn new(
        parents: Vec<CommitId>,
        tree: BlockId,
        author: impl Into<String>,
        message: impl Into<String>,
        timestamp_unix_ms: i64,
    ) -> Self {
        let mut commit = Self {
            id: CommitId::zeroed(),
            parents,
            tree,
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

    /// Mengembalikan daftar parent commit.
    #[must_use]
    pub fn parents(&self) -> &[CommitId] {
        &self.parents
    }

    /// Mengembalikan root block snapshot.
    #[must_use]
    pub fn tree(&self) -> BlockId {
        self.tree
    }

    /// Mengembalikan author commit.
    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }

    /// Mengembalikan pesan commit lengkap.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Mengembalikan baris pertama pesan commit.
    #[must_use]
    pub fn summary(&self) -> &str {
        self.message.lines().next().unwrap_or_default()
    }

    /// Mengembalikan waktu commit dalam milidetik sejak epoch Unix.
    #[must_use]
    pub fn timestamp_unix_ms(&self) -> i64 {
        self.timestamp_unix_ms
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
            author: &self.author,
            message: &self.message,
            timestamp_unix_ms: self.timestamp_unix_ms,
        })
    }
}
