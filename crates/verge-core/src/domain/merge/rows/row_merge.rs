//! File: `row_merge.rs`
//!
//! Deskripsi: Merge tiga arah pada level baris tabel.
//! Layer: domain/merge/rows
//! Tanggung jawab: Menggabungkan base, ours, dan theirs menurut kunci baris.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/{merge_strategy,row_lookup,row_resolution}.rs`
//!   - `domain/tree/value-objects/table_row.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::merge::merge_strategy::MergeStrategy;
use crate::domain::merge::rows::row_conflict::RowConflict;
use crate::domain::merge::rows::row_lookup::{all_keys, value_of};
use crate::domain::merge::rows::row_resolution::{resolve, Resolution, Sides};
use crate::domain::tree::value_objects::table_row::TableRow;

/// Hasil merge baris: baris gabungan dan daftar konflik yang tersisa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowMerge {
    /// Baris hasil gabungan, terurut menurut kunci.
    pub rows: Vec<TableRow>,
    /// Konflik yang tidak dapat diselesaikan oleh strategi terpilih.
    pub conflicts: Vec<RowConflict>,
}

/// Menggabungkan tiga versi tabel menurut kunci baris.
///
/// KONTEKS: baris yang hanya berubah di satu sisi langsung diambil dari sisi itu
/// tanpa guessing; KENAPA: merge dua arah tanpa base tidak bisa membedakan "diubah di
/// satu sisi" dari "diubah di kedua sisi" sehingga setiap perbedaan jadi konflik
/// palsu.
///
/// Args:
/// - `base` — baris pada merge base.
/// - `ours` — baris pada branch aktif.
/// - `theirs` — baris pada branch yang digabung.
/// - `strategy` — cara menyelesaikan konflik.
/// - `ours_is_newer` — `true` bila commit branch aktif paling baru; dipakai
///   [`MergeStrategy::LastWriteWins`].
///
/// Returns:
/// - `RowMerge` — baris gabungan dan konflik yang tersisa.
///
/// Example:
/// ```
/// use verge_core::domain::merge::merge_strategy::MergeStrategy;
/// use verge_core::domain::merge::rows::row_merge::merge_rows;
/// use verge_core::domain::tree::table_codec::TableRows;
///
/// let base = TableRows::parse(b"id,name\n1,ana\n").unwrap();
/// let ours = TableRows::parse(b"id,name\n1,ana\n2,budi\n").unwrap();
/// let theirs = TableRows::parse(b"id,name\n1,ana\n").unwrap();
///
/// let merged = merge_rows(base.rows(), ours.rows(), theirs.rows(), MergeStrategy::Manual, true);
/// assert!(merged.conflicts.is_empty());
/// assert_eq!(merged.rows.len(), 2);
/// ```
#[must_use]
pub fn merge_rows(
    base: &[TableRow],
    ours: &[TableRow],
    theirs: &[TableRow],
    strategy: MergeStrategy,
    ours_is_newer: bool,
) -> RowMerge {
    let mut rows = Vec::with_capacity(ours.len().max(theirs.len()));
    let mut conflicts = Vec::new();
    for key in all_keys(base, ours, theirs) {
        let sides = Sides {
            key: key.clone(),
            base: value_of(base, &key),
            ours: value_of(ours, &key),
            theirs: value_of(theirs, &key),
        };
        match resolve(&sides, strategy, ours_is_newer) {
            Resolution::Merged(Some(value)) => {
                if let Some(row) = TableRow::new(key, value) {
                    rows.push(row);
                }
            }
            Resolution::Merged(None) => {}
            Resolution::Conflict(conflict) => conflicts.push(conflict),
        }
    }
    RowMerge { rows, conflicts }
}
