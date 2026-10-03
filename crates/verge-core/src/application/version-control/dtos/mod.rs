//! File: `mod.rs`
//!
//! Deskripsi: DTO fitur version control.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Mendeklarasikan hasil use case untuk pemanggil.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commits/history_entry.rs`, `commits/recorded_commit.rs`
//!   - `snapshot_content.rs`
//!   - `staged_table.rs`
//!   - `table_diff_report.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

pub mod branching;
pub mod merging;

mod commits;

pub mod snapshot_content;
pub mod staged_table;
pub mod table_diff_report;
pub mod tag_list;

// DTO commit dikelompokkan agar folder ini tetap di bawah batas lima berkas;
// path publiknya sengaja tidak berubah lewat re-export ini.
pub use commits::{history_entry, recorded_commit};
