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
//!   - `history_entry.rs`, `recorded_commit.rs`, `snapshot_content.rs`, `staged_table.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

pub mod history_entry;
pub mod recorded_commit;
pub mod snapshot_content;
pub mod staged_table;
