//! File: `mod.rs`
//!
//! Deskripsi: Kumpulan use case version control.
//! Layer: application/version-control/use-cases
//! Tanggung jawab: Mendeklarasikan empat use case fitur dan test-nya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!//!   - `read_history.rs`
//!   - `read_snapshot.rs`
//!   - `record_commit.rs`
//!   - `stage_table.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

#[path = "tests/mod.rs"]
#[cfg(test)]
mod tests;

pub mod read_history;
pub mod read_snapshot;
pub mod record_commit;
pub mod stage_table;
