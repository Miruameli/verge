//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test pointer ref.
//! Layer: infrastructure/commit/file-system/refs
//! Tanggung jawab: Mendaftarkan test pointer branch dan tag.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `pointer_fixtures.rs`, `ref_pointer_tests.rs`,
//!     `ref_pointer_branch_ops_tests.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

pub(crate) mod pointer_fixtures;
mod ref_pointer_branch_ops_tests;
mod ref_pointer_tests;
