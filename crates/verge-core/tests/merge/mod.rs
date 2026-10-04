//! File: `mod.rs`
//!
//! Deskripsi: Test integrasi aturan merge.
//! Layer: tests (integration)
//! Tanggung jawab: Mendaftarkan test merge base yang memakai adapter
//!   penyimpanan nyata. Dipisah dari `src/domain/merge/tests/` supaya domain
//!   tidak bergantung pada infrastruktur.
//!
//! Author: Miruameli
//! Created: 2026-10-04
//! Modified: 2026-10-04
//! Version: 0.3.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_chain_fixture.rs`, `merge_base_tests.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

mod commit_chain_fixture;
mod merge_base_tests;
