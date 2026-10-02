//! File: `table_diff_report.rs`
//!
//! Deskripsi: DTO hasil perbandingan tabel pada dua revisi.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Membawa perubahan baris beserta revisi pembandingnya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/commit_id.rs`
//!   - `domain/tree/diff/row_change.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::tree::diff::row_change::RowChange;

/// Perubahan baris tabel dari satu revisi ke revisi lain.
///
/// Invariants:
/// - Perubahan terurut menaik menurut kunci baris.
/// - Tabel identik menghasilkan daftar perubahan kosong.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableDiffReport {
    /// Commit yang dibaca sebagai tabel lama.
    pub from: CommitId,
    /// Commit yang dibaca sebagai tabel baru.
    pub to: CommitId,
    /// Perubahan baris terurut menurut kunci.
    pub changes: Vec<RowChange>,
}

impl TableDiffReport {
    /// Melaporkan apakah kedua tabel identik.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Menghitung jumlah baris yang berubah.
    #[must_use]
    pub fn len(&self) -> usize {
        self.changes.len()
    }
}
