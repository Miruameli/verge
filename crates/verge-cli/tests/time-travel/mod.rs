//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test end-to-end time-travel.
//! Layer: interfaces/cli/tests
//! Tanggung jawab: Mendaftarkan test query `AS OF` dan perintah tag.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `queries/`
//!   - `tag_cli.rs`, `tag_fixtures.rs`, `tag_rejection_cli.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

#[path = "../support/mod.rs"]
pub(crate) mod support;

pub(crate) mod queries;
mod tag_cli;
mod tag_fixtures;
mod tag_rejection_cli;
