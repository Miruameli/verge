//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk use case manipulasi branch.
//! Layer: application/version-control/use-cases
//! Tanggung jawab: Mendaftarkan operasi create, switch, list, dan delete.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `create_branch.rs`
//!   - `delete_branch.rs`
//!   - `list_branches.rs`
//!   - `switch_branch.rs`
//!   - `tests/`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
//!
//! Setiap use case ada di berkasnya sendiri agar dapat dibaca, diuji, dan
//! ditinjau tanpa membuka berkas lain.

#[cfg(test)]
mod tests;

pub mod create_branch;
pub mod delete_branch;
pub mod list_branches;
pub mod switch_branch;
