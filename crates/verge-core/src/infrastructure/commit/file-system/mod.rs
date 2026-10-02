//! File: mod.rs
//!
//! Deskripsi: Adapter commit di filesystem lokal.
//! Layer: infrastructure/commit/file-system
//! Tanggung jawab: Mendeklarasikan repository commit dan pointer branch.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `file_commit_repository.rs`, `file_ref_pointer.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

#[cfg(test)]
mod file_commit_repository_tests;
#[cfg(test)]
mod file_ref_pointer_tests;

pub mod file_commit_repository;
pub mod file_ref_pointer;
