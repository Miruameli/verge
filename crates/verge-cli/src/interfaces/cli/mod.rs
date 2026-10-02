//! File: mod.rs
//!
//! Deskripsi: Adapter command-line untuk engine Verge.
//! Layer: interfaces/cli
//! Tanggung jawab: Mendeklarasikan dispatcher dan perintah CLI.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `cli_dispatcher.rs`, commands/mod.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

#[path = "commands/mod.rs"]
pub mod commands;

pub mod cli_dispatcher;
