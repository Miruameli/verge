//! File: mod.rs
//!
//! Deskripsi: Layer `domain` — aturan bisnis murni tanpa I/O.
//! Layer: domain
//! Tanggung jawab: Mendeklarasikan subdomain core, data, sql, tree.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi eksternal)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #8 (Milestone 2)
//!   - #95 (domain layer split)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)
//!
//! Subdomain structure:
//!   - core/ → ident, commit, merge, time (domain inti)
//!   - data/ → storage, table (domain data)
//!   - sql/ → query engine (domain khusus)
//!   - tree/ → prolly tree (domain khusus)

// Core subdomain: identitas, commit, merge, time
#[path = "core/commit/mod.rs"]
pub mod commit;
#[path = "core/ident/mod.rs"]
pub mod ident;
#[path = "core/merge/mod.rs"]
pub mod merge;
#[path = "core/time/mod.rs"]
pub mod time;

// Data subdomain: storage, table
#[path = "data/storage/mod.rs"]
pub mod storage;
#[path = "data/table/mod.rs"]
pub mod table;

// Domain khusus
pub mod sql;
#[path = "tree/mod.rs"]
pub mod tree;
