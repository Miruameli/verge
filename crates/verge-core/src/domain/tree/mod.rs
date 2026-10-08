//! File: `mod.rs`
//!
//! Deskripsi: Subdomain `tree` — struktur baris tabel yang dapat berbagi blok.
//! Layer: domain/tree
//! Tanggung jawab: Mendeklarasikan codec, diff, nodes, builder, value-objects.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `codec/mod.rs`, `diff/mod.rs`, `nodes/mod.rs`, `builder/mod.rs`, `value-objects/mod.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!   - #97 (domain/tree split)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)
//!
//! Component structure:
//!   - codec/ → table_codec, table_codec_rows, table_reader
//!   - diff/ → table_diff, table_diff_tests
//!   - nodes/ → tree_node, tree_node_codec, tree_node_decoding
//!   - builder/ → tree_builder
//!   - value-objects/ → (value objects)

// Codec components
#[path = "codec/mod.rs"]
pub mod codec;

// Diff components
#[path = "diff/mod.rs"]
pub mod diff;

// Node components
#[path = "nodes/mod.rs"]
pub mod nodes;

// Builder components
#[path = "builder/mod.rs"]
pub mod builder;

// Value objects
#[path = "value-objects/mod.rs"]
pub mod value_objects;
