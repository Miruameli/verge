//! File: `mod.rs`
//!
//! Deskripsi: Test fitur version control yang lintas use case.
//! Layer: application/version-control
//! Tanggung jawab: Mendeklarasikan test penyelesaian revisi dan test tag
//!   yang dikelompokkan per use case di `tagging/`.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-04
//! Version: 0.3.0 · License: Apache-2.0
//!
//! Dependencies: `instant_lookup_tests.rs`, `instant_table_filter_tests.rs`,
//!   `revision_prefix_tests.rs`, `revision_resolution_tests.rs`,
//!   `revision_shape_tests.rs`, `tagging/`
//! Related issues: #18 (Milestone 3), #25 (Milestone 4)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel), ADR-0008 (AS OF dan tag)

mod fixtures;
mod instant_lookup_tests;
mod instant_table_filter_tests;
mod revision_prefix_tests;
mod revision_resolution_tests;
mod revision_shape_tests;
mod tagging;
