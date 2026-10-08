//! File: `mod.rs`
//!
//! Deskripsi: Aturan merge tiga arah yang tidak bergantung pada penyimpanan.
//! Layer: domain/merge
//! Tanggung jawab: Menyediakan strategi, resolusi baris, dan merge base.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/commit_repository.rs`
//!   - `domain/tree/value-objects/table_row.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
//!
//! Modul ini tidak menyimpan apa pun: merge dihitung dari commit yang sudah
//! ada sehingga pohon yang sama dapat dihitung ulang kapan pun.

#[cfg(test)]
mod tests;

pub mod merge_base;
pub mod merge_strategy;
pub mod rows;
