//! File: `branch_list.rs`
//!
//! Deskripsi: DTO daftar branch beserta ujung commit-nya.
//! Layer: application/version-control/dtos/branching
//! Tanggung jawab: Membawa hasil baca daftar branch tanpa port maupun storage.
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
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::commit::value_objects::commit_id::CommitId;

/// Satu branch beserta keadaan ujungnya.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchInfo {
    /// Nama branch.
    pub name: String,
    /// Commit terakhir branch, atau `None` bila pointer hilang saat dibaca.
    pub head: Option<CommitId>,
    /// `true` bila branch adalah branch aktif yang ditunjuk `HEAD`.
    pub current: bool,
}

/// Daftar seluruh branch dalam repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchList {
    /// Branch aktif, tercantum juga di dalam `branches`.
    pub current: String,
    /// Seluruh branch terurut menaik menurut nama.
    pub branches: Vec<BranchInfo>,
}

impl BranchList {
    /// Mengembalikan commit yang ditunjuk branch aktif.
    ///
    /// Returns:
    /// - Option<CommitId> — ujung branch aktif, `None` bila belum ada commit.
    #[must_use]
    pub fn current_head(&self) -> Option<CommitId> {
        self.branches
            .iter()
            .find(|branch| branch.current)
            .and_then(|branch| branch.head)
    }
}
