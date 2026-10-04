//! File: `tag_list.rs`
//!
//! Deskripsi: DTO daftar tag beserta commit yang ditunjuknya.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Menyediakan data tag yang ditampilkan CLI tanpa membocorkan
//!   objek commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/commit_id.rs`
//!   - `domain/table/value-objects/table_name.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::table::value_objects::table_name::TableName;

/// Satu tag beserta commit yang ditunjuknya.
///
/// KENAPA commit, tabel, dan ringkasan ikut dicetak: tag tanpa commit yang
/// ditunjuknya mudah disalahartikan sebagai nama branch biasa, dan tanpa nama
/// tabel pengguna tidak dapat tahu tag tersebut berlaku untuk tabel mana —
/// kesalahan yang membuat `verge query --as-of <tag> --table lain` gagal tanpa
/// petunjuk.
///
/// Immutability: penuh; tag bersifat immutable dan commit tidak pernah berubah.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagEntry {
    /// Nama tag seperti disimpan di `refs/tags/`.
    pub name: String,
    /// Commit yang ditunjuk tag.
    pub commit: CommitId,
    /// Tabel yang dimiliki commit tersebut.
    pub table: TableName,
    /// Baris pertama pesan commit.
    pub summary: String,
}

/// Seluruh tag yang ada, terurut menaik menurut nama.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagList {
    /// Satu entri per tag.
    pub entries: Vec<TagEntry>,
}
