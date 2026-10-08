//! File: `mod.rs`
//!
//! Deskripsi: Value object waktu.
//! Layer: domain/time
//! Tanggung jawab: Mendaftarkan nilai waktu yang dipakai time-travel.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `civil_calendar.rs`, `timestamp.rs`, `timestamp_parsing.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

pub mod civil_calendar;
pub mod timestamp;
pub mod timestamp_parsing;
