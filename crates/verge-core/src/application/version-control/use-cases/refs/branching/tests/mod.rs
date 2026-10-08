//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test use case manipulasi branch.
//! Layer: application/version-control/use-cases/refs/branching
//! Tanggung jawab: Mendaftarkan test per operasi branch.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `branch_creation_tests.rs`, `branch_deletion_tests.rs`, `branch_listing_tests.rs`, `branch_switch_tests.rs`, `fixtures/`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

mod branch_creation_tests;
mod branch_deletion_tests;
mod branch_listing_tests;
mod branch_switch_tests;
mod fixtures;
