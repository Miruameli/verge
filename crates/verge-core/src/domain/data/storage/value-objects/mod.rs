//! File: mod.rs
//!
//! Deskripsi: Value object untuk penyimpanan blok.
//! Layer: domain/storage/value-objects
//! Tanggung jawab: Mendeklarasikan `BlockId` dan `PutOutcome`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/ident/value-objects/digest.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

pub mod block_id;
pub mod put_outcome;
