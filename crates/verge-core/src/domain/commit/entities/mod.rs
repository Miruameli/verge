//! File: mod.rs
//!
//! Deskripsi: Entitas domain commit.
//! Layer: domain/commit/entities
//! Tanggung jawab: Mendeklarasikan `Commit` dan encoding kanoniknya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - commit.rs, `commit_encoding.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

#[cfg(test)]
mod commit_tests;

pub mod commit;
pub mod commit_encoding;
