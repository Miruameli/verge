//! File: mod.rs
//!
//! Deskripsi: Kumpulan perintah CLI.
//! Layer: interfaces/cli/commands
//! Tanggung jawab: Mendeklarasikan satu berkas per perintah.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `init_repository.rs`
//!   - `table-versioning/`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

pub mod branches;
pub mod init_repository;
pub mod merging;

#[path = "table-versioning/mod.rs"]
pub mod table_versioning;
