//! File: `mod.rs`
//!
//! Deskripsi: Fitur version control tabel.
//! Layer: application/version-control
//! Tanggung jawab: Mendeklarasikan use case dan DTO fitur.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!//!   - `dtos/mod.rs`
//!   - `use-cases/mod.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

#[path = "dtos/mod.rs"]
pub mod dtos;

#[path = "fakes/mod.rs"]
pub mod fakes;

#[path = "use-cases/mod.rs"]
pub mod use_cases;
