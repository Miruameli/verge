//! File: mod.rs
//!
//! Deskripsi: Port (kontrak) untuk penyimpanan blok.
//! Layer: domain/storage/ports
//! Tanggung jawab: Mendeklarasikan trait yang diimplementasi infrastructure.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/storage/value-objects/*
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)
//!   - ADR-0003 (Arsitektur 7-layer)

#[cfg(test)]
mod block_store_tests;

pub mod block_store;
pub mod block_store_factory;
pub mod metadata_writer;
