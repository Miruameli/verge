//! File: `mod.rs`
//!
//! Deskripsi: Port repository subdomain `commit`.
//! Layer: domain/commit/repositories/ports
//! Tanggung jawab: Mendeklarasikan tempat commit disimpan dan pointer dibaca.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_repository.rs`, `ref_pointer.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

pub mod commit_repository;
pub mod ref_pointer;
pub mod tag_pointer;
