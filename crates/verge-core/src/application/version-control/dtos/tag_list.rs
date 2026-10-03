//! File: `tag_list.rs`
//!
//! Deskripsi: DTO daftar tag beserta commit yang ditunjuknya.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Menyediakan data tag yang ditampilkan CLI tanpa membocorkan
//!   objek commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/commit_id.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::domain::commit::value_objects::commit_id::CommitId;

/// Nama tag, commit yang ditunjuknya, dan ringkasan commit tersebut.
///
/// KENAPA ringkasan ikut dicetak: tag tanpa commit yang ditunjuknya mudah
/// disalahartikan sebagai nama branch biasa.
pub type TagEntry = (String, CommitId, String);

/// Seluruh tag yang ada, terurut menaik menurut nama.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagList {
    /// Satu entri per tag.
    pub entries: Vec<TagEntry>,
}
