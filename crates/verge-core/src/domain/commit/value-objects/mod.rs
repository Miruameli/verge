//! File: mod.rs
//!
//! Deskripsi: Value object domain commit.
//! Layer: domain/commit/value-objects
//! Tanggung jawab: Mendeklarasikan `CommitId` dan `Ref`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_id.rs`, `commit_ref.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

pub mod branch_name_policy;
pub mod commit_id;
pub mod commit_ref;
