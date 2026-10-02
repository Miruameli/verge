//! File: `mod.rs`
//!
//! Deskripsi: Port subdomain `table`.
//! Layer: domain/table/ports
//! Tanggung jawab: Mendeklarasikan kontrak data kerja tabel.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `table_source.rs`, `table_workspace.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

#[cfg(test)]
mod table_workspace_tests;

pub mod table_source;
pub mod table_workspace;
