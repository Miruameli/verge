//! File: mod.rs
//!
//! Deskripsi: Subdomain `commit` — versioning Inti.
//! Layer: domain/commit
//! Tanggung jawab: Mendeklarasikan entitas commit, value object, dan graph.
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
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)
//!   - ADR-0003 (Arsitektur 7-layer)

#[path = "entities/mod.rs"]
pub mod entities;

#[path = "repositories/mod.rs"]
pub mod repositories;

#[path = "value-objects/mod.rs"]
pub mod value_objects;
