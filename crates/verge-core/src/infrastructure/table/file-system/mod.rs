//! File: mod.rs
//!
//! Deskripsi: Adapter tabel di filesystem lokal.
//! Layer: infrastructure/table/file-system
//! Tanggung jawab: Mendeklarasikan workspace data kerja dan sumber data tabel.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `file_table_workspace.rs`, `file_table_source.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

#[cfg(test)]
mod file_table_source_tests;
#[cfg(test)]
mod file_table_workspace_tests;

pub mod file_table_source;
pub mod file_table_workspace;
