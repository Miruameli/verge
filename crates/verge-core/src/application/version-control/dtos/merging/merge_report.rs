//! File: `merge_report.rs`
//!
//! Deskripsi: DTO hasil merge branch.
//! Layer: application/version-control/dtos/merging
//! Tanggung jawab: Membawa ringkasan merge dan konflik yang tersisa.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/{row_conflict,merge_strategy}.rs`
//!   - `domain/commit/value-objects/commit_id.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::merge::merge_strategy::MergeStrategy;
use crate::domain::merge::rows::row_conflict::RowConflict;

/// Ringkasan satu operasi merge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeReport {
    /// Branch yang menjadi branch aktif setelah merge.
    pub current: String,
    /// Branch yang datanya digabung.
    pub source: String,
    /// Strategi yang dipakai.
    pub strategy: MergeStrategy,
    /// Commit merge base yang dipakai; `None` bila kedua sisi identik.
    pub base: Option<CommitId>,
    /// Commit merge yang ditulis; `None` bila merge dibatalkan karena konflik.
    pub commit: Option<CommitId>,
    /// Jumlah baris pada tabel hasil.
    pub rows: usize,
    /// Konflik yang belum terselesaikan.
    pub conflicts: Vec<RowConflict>,
}

impl MergeReport {
    /// Mengembalikan `true` bila merge menghasilkan commit baru.
    #[must_use]
    pub fn merged(&self) -> bool {
        self.commit.is_some()
    }

    /// Mengembalikan `true` bila ada konflik yang belum terselesaikan.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}
