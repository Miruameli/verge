//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test end-to-end versioning tabel.
//! Layer: interfaces/cli/tests
//! Tanggung jawab: Mendaftarkan test stage, commit, pembacaan, dan diff.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `stage_and_commit_cli.rs`, `read_cli.rs`, `diff_cli.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

#[path = "../support/mod.rs"]
mod support;

mod diff_cli;
mod read_cli;
mod stage_and_commit_cli;
