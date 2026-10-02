//! File: mod.rs
//!
//! Deskripsi: Adapter tabel di filesystem lokal.
//! Layer: infrastructure/table
//! Tanggung jawab: Mendeklarasikan data kerja tabel dan pembaca sumber tabel.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - infrastructure/table/file-system/mod.rs
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

#[path = "file-system/mod.rs"]
pub mod file_system;
