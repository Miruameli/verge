//! File: `recorded_commit.rs`
//!
//! Deskripsi: DTO hasil pembuatan commit.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Membawa identitas commit baru dan branch yang bergerak.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/commit_id.rs`
//!   - `domain/storage/value-objects/block_id.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::storage::value_objects::block_id::BlockId;

/// Commit yang baru dibuat beserta dampaknya terhadap branch.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedCommit {
    /// Identifier commit yang tercatat.
    pub id: CommitId,
    /// Branch yang sekarang menunjuk commit ini.
    pub branch: String,
    /// Blok data tabel yang di-versioning.
    pub tree: BlockId,
    /// `false` bila commit dengan isi identik sudah pernah tersimpan.
    pub created: bool,
}
