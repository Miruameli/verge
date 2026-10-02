//! File: mod.rs
//!
//! Deskripsi: Repository domain untuk commit, branch, dan tag.
//! Layer: domain/commit/repositories
//! Tanggung jawab: Mendeklarasikan graph commit dan traversal sejarahnya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_graph.rs`, `commit_history.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

#[cfg(test)]
mod commit_graph_tests;

pub mod commit_graph;
pub mod commit_history;
