//! File: `mod.rs`
//!
//! Deskripsi: Test end-to-end `verge query`.
//! Layer: interfaces/cli/tests/time-travel/queries
//! Tanggung jawab: Mendaftarkan test pembacaan pada waktu tertentu beserta
//!   fixture repository-nya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `query_read_cli.rs`, `query_error_cli.rs`, `query_sql_cli.rs`, `query_fixtures.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

pub(super) use super::support;

mod query_error_cli;
pub(super) mod query_fixtures;
mod query_read_cli;
mod query_sql_cli;
