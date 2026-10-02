//! File: `mod.rs`
//!
//! Deskripsi: Value object subdomain `tree`.
//! Layer: domain/tree/value-objects
//! Tanggung jawab: Mendeklarasikan kunci dan baris tabel.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `row_key.rs`, `table_row.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

#[cfg(test)]
mod row_key_tests;
#[cfg(test)]
mod table_codec_tests;

pub mod row_key;
pub mod table_row;
