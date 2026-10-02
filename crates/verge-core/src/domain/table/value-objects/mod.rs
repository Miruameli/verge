//! File: `mod.rs`
//!
//! Deskripsi: Value object subdomain `table`.
//! Layer: domain/table/value-objects
//! Tanggung jawab: Mendeklarasikan nama tabel tervalidasi.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `table_name.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

#[cfg(test)]
mod table_name_tests;

pub mod table_name;
