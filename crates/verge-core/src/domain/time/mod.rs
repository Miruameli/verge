//! File: `mod.rs`
//!
//! Deskripsi: Layer `domain` — nilai waktu sebagai domain value object.
//! Layer: domain
//! Tanggung jawab: Mendeklarasikan subdomain waktu.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi eksternal)
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

pub mod value_objects;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
