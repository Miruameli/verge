//! File: `mod.rs`
//!
//! Deskripsi: Perbandingan dua snapshot tabel.
//! Layer: domain/tree/diff
//! Tanggung jawab: Menghitung perubahan baris antara dua tree.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `table_diff.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

#[cfg(test)]
mod table_diff_tests;

pub mod row_change;
pub mod table_diff;
