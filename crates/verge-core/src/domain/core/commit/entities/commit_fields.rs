//! File: `commit_fields.rs`
//!
//! Deskripsi: Pembaca field entitas commit.
//! Layer: domain/commit/entities
//! Tanggung jawab: Menyediakan akses baca ke bagian commit tanpa mengubahnya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::value_objects::table_name::TableName;

impl Commit {
    /// Mengembalikan daftar parent commit.
    #[must_use]
    pub fn parents(&self) -> &[CommitId] {
        &self.parents
    }

    /// Mengembalikan root block snapshot tabel.
    #[must_use]
    pub fn tree(&self) -> BlockId {
        self.tree
    }

    /// Mengembalikan tabel yang di-versioning commit ini.
    #[must_use]
    pub fn table(&self) -> &TableName {
        &self.table
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
}
