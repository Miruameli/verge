//! File: `block_id.rs`
//!
//! Deskripsi: Value object `BlockId` — nama content-addressed sebuah blok.
//! Layer: domain/storage/value-objects
//! Tanggung jawab: Memberi nama tipe pada digest yang di-pointer sebagai blok.
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

use crate::domain::ident::value_objects::digest::Digest;

/// Identifier sebuah blok; sama dengan digest isi blok tersebut.
///
/// Alias ini sengaja dibuat agar signature port dan commit tidak memakai
/// `Digest` secara langsung: nama tipe ikut membawa makna saat dibaca.
pub type BlockId = Digest;
