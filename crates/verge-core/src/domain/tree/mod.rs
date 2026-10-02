//! File: `mod.rs`
//!
//! Deskripsi: Subdomain `tree` — struktur baris tabel yang dapat berbagi blok.
//! Layer: domain/tree
//! Tanggung jawab: Mendeklarasikan baris, node tree, builder, dan diff.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `diff/mod.rs`, `nodes/mod.rs`, `value-objects/mod.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

#[path = "diff/mod.rs"]
pub mod diff;

#[path = "nodes/mod.rs"]
pub mod nodes;

#[path = "value-objects/mod.rs"]
pub mod value_objects;

pub mod table_codec;
pub mod table_reader;
pub mod tree_builder;
