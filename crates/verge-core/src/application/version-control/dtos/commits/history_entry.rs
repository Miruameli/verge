//! File: `history_entry.rs`
//!
//! Deskripsi: DTO satu baris riwayat commit.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Menyediakan data yang ditampilkan CLI tanpa membocorkan internal.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/commit_id.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::commit::value_objects::commit_id::CommitId;

/// Satu entri riwayat commit untuk sebuah tabel.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    /// Identifier commit.
    pub id: CommitId,
    /// Penulis commit.
    pub author: String,
    /// Baris pertama pesan commit.
    pub summary: String,
    /// Waktu commit dalam milidetik sejak epoch Unix.
    pub timestamp_unix_ms: i64,
}
