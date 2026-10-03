//! File: mod.rs
//!
//! Deskripsi: Layer `domain` — aturan bisnis murni tanpa I/O.
//! Layer: domain
//! Tanggung jawab: Mendeklarasikan subdomain ident, commit, dan storage.
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
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

#[path = "commit/mod.rs"]
pub mod commit;
pub mod ident;
pub mod merge;
pub mod storage;
#[path = "table/mod.rs"]
pub mod table;
pub mod time;
#[path = "tree/mod.rs"]
pub mod tree;
