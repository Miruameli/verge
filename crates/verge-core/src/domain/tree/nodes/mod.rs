//! File: `mod.rs`
//!
//! Deskripsi: Node prolly tree dan codec-nya.
//! Layer: domain/tree/nodes
//! Tanggung jawab: Menyimpan baris terpartisi dengan encoding deterministik.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `tree_node.rs`, `tree_node_codec.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

pub mod tree_node;
pub mod tree_node_codec;
