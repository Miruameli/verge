//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test use case tag.
//! Layer: application/version-control/tests/tagging
//! Tanggung jawab: Mendeklarasikan test pembuatan, penghapusan, dan daftar tag
//!   yang sebelumnya berada di satu berkas.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `tag_creation_tests.rs`
//!   - `tag_deletion_tests.rs`
//!   - `tag_listing_tests.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

mod tag_creation_tests;
mod tag_deletion_tests;
mod tag_listing_tests;
