//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test adapter commit berbasis berkas.
//! Layer: infrastructure/commit/file-system
//! Tanggung jawab: Mendaftarkan test yang hanya berjalan pada `cfg(test)`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

mod commit_repository_tests;
mod pointer_fixtures;
mod ref_pointer_branch_ops_tests;
mod ref_pointer_tests;
