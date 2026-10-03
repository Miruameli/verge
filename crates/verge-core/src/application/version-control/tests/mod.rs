//! File: `mod.rs`
//!
//! Deskripsi: Test fitur version control yang lintas use case.
//! Layer: application/version-control
//! Tanggung jawab: Mendeklarasikan test penyelesaian revisi.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `instant_lookup_tests.rs`, `revision_prefix_tests.rs`,
//!   `revision_resolution_tests.rs`, `tag_command_tests.rs`
//! Related issues: #18 (Milestone 3), #25 (Milestone 4)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

mod fixtures;
mod instant_lookup_tests;
mod revision_prefix_tests;
mod revision_resolution_tests;
mod tag_command_tests;
