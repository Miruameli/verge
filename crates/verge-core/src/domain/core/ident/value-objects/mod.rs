//! File: mod.rs
//!
//! Deskripsi: Value object identifier (SHA-256 digest).
//! Layer: domain/ident/value-objects
//! Tanggung jawab: Mendeklarasikan `Digest` beserta representasi teksnya.
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

#[cfg(test)]
mod digest_tests;

pub mod digest;
pub mod digest_text;
