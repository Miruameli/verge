//! File: mod.rs
//!
//! Deskripsi: Perintah CLI yang hanya membaca isi repository.
//! Layer: interfaces/cli/commands/table-versioning/queries
//! Tanggung jawab: Mendeklarasikan `log`, `show`, dan `diff`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `diff_tables.rs`, `read_history.rs`, `read_snapshot.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

pub mod diff_tables;
pub mod query_table;
pub mod read_history;
pub mod read_snapshot;
