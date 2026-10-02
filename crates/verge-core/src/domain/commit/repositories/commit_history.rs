//! File: `commit_history.rs`
//!
//! Deskripsi: Traversal sejarah pada `CommitGraph`.
//! Layer: domain/commit/repositories
//! Tanggung jawab: Menyediakan log first-parent dan uji ancestry.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_graph.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::collections::{BTreeSet, VecDeque};

use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::repositories::commit_graph::CommitGraph;
use crate::domain::commit::value_objects::commit_id::CommitId;

impl CommitGraph {
    /// Menelusuri first-parent mulai dari `from`, commit terbaru lebih dulu.
    ///
    /// Args:
    /// - from — commit awal penelusuran.
    /// - limit — batas jumlah commit yang dikembalikan.
    ///
    /// Returns:
    /// - Vec<&Commit> — riwayat terbaru lebih dulu, maksimal `limit` entri.
    ///
    /// Performance: O(limit) lookup dan alokasi.
    /// Thread-safe: ya (hanya membaca).
    #[must_use]
    pub fn first_parent_log(&self, from: CommitId, limit: usize) -> Vec<&Commit> {
        let mut log = Vec::new();
        let mut cursor = Some(from);
        while let Some(id) = cursor {
            if log.len() == limit {
                break;
            }
            let Some(commit) = self.commit(&id) else {
                break;
            };
            log.push(commit);
            cursor = commit.parents().first().copied();
        }
        log
    }

    /// Melaporkan apakah `ancestor` dapat dicapai dari `descendant`.
    ///
    /// Kasus/commit yang sama dianggap bukan ancestor ketat.
    #[must_use]
    pub fn is_ancestor(&self, ancestor: CommitId, descendant: CommitId) -> bool {
        ancestor != descendant && self.is_ancestor_or_self(ancestor, descendant)
    }

    /// Melaporkan apakah `ancestor` dapat dicapai dari `descendant`, atau sama
    /// dengan `descendant`.
    ///
    /// Performance: O(V + E) dengan penanda `seen`, sehingga diamond merge tidak
    /// menyebabkan penelusuran berulang.
    /// Thread-safe: ya (hanya membaca).
    #[must_use]
    pub fn is_ancestor_or_self(&self, ancestor: CommitId, descendant: CommitId) -> bool {
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([descendant]);
        while let Some(current) = queue.pop_front() {
            if current == ancestor {
                return true;
            }
            if !seen.insert(current) {
                continue;
            }
            if let Some(commit) = self.commit(&current) {
                queue.extend(commit.parents().iter().copied());
            }
        }
        false
    }
}
