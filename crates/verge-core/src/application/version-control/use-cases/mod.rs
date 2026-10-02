//! File: `mod.rs`
//!
//! Deskripsi: Kumpulan use case version control.
//! Layer: application/version-control/use-cases
//! Tanggung jawab: Mendeklarasikan lima use case fitur dan test-nya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `diff_tables.rs`
//!   - `read_history.rs`
//!   - `read_snapshot.rs`
//!   - `recording/record_commit.rs`
//!   - `stage_table.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

#[path = "tests/mod.rs"]
#[cfg(test)]
mod tests;

pub mod branching;
pub mod merging;

mod recording;

pub mod diff_tables;
pub mod read_history;
pub mod read_snapshot;
pub mod stage_table;

// Use case commit tinggal di subfolder agar folder ini tetap di bawah batas
// lima berkas; path publiknya sengaja tidak berubah lewat re-export ini.
pub use recording::record_commit;
