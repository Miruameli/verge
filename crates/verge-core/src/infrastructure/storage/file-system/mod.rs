//! File: mod.rs
//!
//! Deskripsi: Backend penyimpanan berbasis filesystem lokal.
//! Layer: infrastructure/storage/file-system
//! Tanggung jawab: Mendeklarasikan store, factory, dan writer berkas lokal.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `file_block_store.rs`, `file_store_factory.rs`, `local_file_system.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

#[cfg(test)]
mod file_block_store_tests;

pub mod file_block_store;
pub mod file_store_factory;
pub mod local_file_system;
