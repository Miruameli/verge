//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test resolusi revisi.
//! Layer: application/version-control/tests/revision
//! Tanggung jawab: Mendeklarasikan test navigasi commit, aturan awalan hex, dan
//!   pemilah bentuk teks revisi. Folder ini dipisah dari test yang menyeberang
//!   use case supaya folder induk tidak menumpuk berkas langsung.
//!
//! Author: Miruameli
//! Created: 2026-10-04
//! Modified: 2026-10-04
//! Version: 0.3.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `revision_prefix_tests.rs`
//!   - `revision_resolution_tests.rs`
//!   - `revision_shape_tests.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (AS OF, nama revisi, dan tag immutable)

mod revision_prefix_tests;
mod revision_resolution_tests;
mod revision_shape_tests;
