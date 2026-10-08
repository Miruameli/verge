//! File: `mod.rs`
//!
//! Deskripsi: Codec byte untuk entitas commit.
//! Layer: domain/commit/codec
//! Tanggung jawab: Mengubah commit menjadi byte kanonik dan sebaliknya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_decoding.rs`, `commit_encoding.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

pub mod commit_decoding;
pub mod commit_encoding;
