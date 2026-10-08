//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk use case query.
//! Layer: application/version-control/use-cases/queries
//! Tanggung jawab: Mendaftarkan query tabel dan query SQL pada satu titik waktu.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commits_tvf.rs` (table-valued function `commits()`)
//!   - `query_table.rs`, `sql_query.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!   - #90 (M5 Part 3: resource limits)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)
//!   - ADR-0012 (SQL parser semantics and limits)

pub mod commits_tvf;
pub mod query_table;
pub mod sql_query;
