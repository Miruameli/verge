//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test end-to-end perintah merge.
//! Layer: interfaces/cli/tests
//! Tanggung jawab: Mendaftarkan test merge bersih dan merge berkonflik.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

#[path = "../support/mod.rs"]
mod support;

mod merge_clean_cli;
mod merge_conflict_cli;
