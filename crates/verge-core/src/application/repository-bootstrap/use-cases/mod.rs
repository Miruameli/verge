//! File: mod.rs
//!
//! Deskripsi: Kumpulan use case repository bootstrap.
//! Layer: application/repository-bootstrap/use-cases
//! Tanggung jawab: Mendeklarasikan use case inisialisasi repository.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `initialize_repository.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

#[cfg(test)]
mod initialize_repository_fakes;
#[cfg(test)]
mod initialize_repository_tests;

pub mod initialize_repository;
