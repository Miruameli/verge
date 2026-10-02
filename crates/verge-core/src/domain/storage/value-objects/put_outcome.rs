//! File: `put_outcome.rs`
//!
//! Deskripsi: Value object hasil penulisan blok.
//! Layer: domain/storage/value-objects
//! Tanggung jawab: Menyampaikan identifier hasil tulis dan apakah blok baru.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/storage/value-objects/block_id.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use crate::domain::storage::value_objects::block_id::BlockId;

/// Hasil dari menulis satu blok ke store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PutOutcome {
    /// Identifier yang diberikan kepada byte yang ditulis.
    pub id: BlockId,
    /// `true` bila blok baru dibuat, `false` bila blok sudah ada sebelumnya.
    pub inserted: bool,
}
