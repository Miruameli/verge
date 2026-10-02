//! File: mod.rs
//!
//! Deskripsi: Subdomain `storage` — port penyimpanan blok.
//! Layer: domain/storage
//! Tanggung jawab: Mendeklarasikan value object dan port storage.
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
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

#[path = "ports/mod.rs"]
pub mod ports;

#[path = "value-objects/mod.rs"]
pub mod value_objects;
