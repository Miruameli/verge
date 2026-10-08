//! File: `mod.rs`
//!
//! Deskripsi: Infrastructure layer untuk query resource management.
//! Layer: infrastructure/query
//! Tanggung jawab: Export `ScanBudget` untuk memantau memori dan waktu
//!   eksekusi SQL query.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #90 (M5 Part 3: resource limits)
//!
//! Related ADR:
//!   - ADR-0014 (Executor resource limits)

pub mod budget;

#[cfg(test)]
mod tests;

pub use budget::ScanBudget;
