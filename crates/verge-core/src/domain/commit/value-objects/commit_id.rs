//! File: `commit_id.rs`
//!
//! Deskripsi: Value object `CommitId` — nama content-addressed sebuah commit.
//! Layer: domain/commit/value-objects
//! Tanggung jawab: Memberi nama tipe pada digest yang menunjuk commit.
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

/// Identifier sebuah commit; sama dengan digest encoding kanoniknya.
///
/// Alias ini membuat signature port dan graph tidak memakai `Digest` mentah.
pub type CommitId = Digest;
