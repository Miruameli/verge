//! File: `mod.rs`
//!
//! Deskripsi: Subdomain `table` — data yang diver^{+}$.
//! Layer: domain/table
//! Tanggung jawab: Mendeklarasikan nama tabel dan port data kerja.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `ports/mod.rs`, `value-objects/mod.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

#[path = "ports/mod.rs"]
pub mod ports;

#[path = "value-objects/mod.rs"]
pub mod value_objects;
