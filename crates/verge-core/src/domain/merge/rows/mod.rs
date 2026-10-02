//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk aturan merge level baris.
//! Layer: domain/merge
//! Tanggung jawab: Mendaftarkan konflik, resolusi, dan gabungan baris.
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

pub mod row_conflict;
pub mod row_merge;
pub mod row_resolution;
