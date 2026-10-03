//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk perintah merge pada CLI.
//! Layer: interfaces/cli/commands
//! Tanggung jawab: Mendaftarkan perintah `verge merge`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `merge_tables.rs`
//!   - `merge_report_printer.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

pub mod merge_report_printer;
pub mod merge_tables;
