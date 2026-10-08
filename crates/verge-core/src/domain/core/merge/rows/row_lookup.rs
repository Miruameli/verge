//! File: `row_lookup.rs`
//!
//! Deskripsi: Pembacaan kumpulan baris pada tiga sisi merge.
//! Layer: domain/merge/rows
//! Tanggung jawab: Memberi akses kunci gabungan dan nilai satu baris.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/tree/value-objects/{row_key,table_row}.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
//!
//! KONTEKS: helper ini dipisah dari `row_resolution.rs`; KENAPA: resolusi satu
//! kunci adalah keputusan bisnis, sedangkan pengumpulan kunci dan pencarian
//! nilai hanya bergantung pada bentuk baris terurut.

use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Mengembalikan seluruh kunci dari tiga sisi dalam urutan menaik tanpa duplikat.
///
/// Args:
/// - base — baris pada merge base.
/// - ours — baris pada branch aktif.
/// - theirs — baris pada branch yang digabung.
///
/// Returns:
/// - Vec<RowKey> — kunci unik terurut menaik.
///
/// Performance: satu alokasi; `dedup` berjalan setelah pengurutan sehingga
/// jumlah perbandingan tetap O(n log n).
#[must_use]
pub fn all_keys(base: &[TableRow], ours: &[TableRow], theirs: &[TableRow]) -> Vec<RowKey> {
    let mut keys: Vec<RowKey> = base
        .iter()
        .chain(ours)
        .chain(theirs)
        .map(TableRow::key)
        .cloned()
        .collect();
    keys.sort_unstable();
    keys.dedup();
    keys
}

/// Mengembalikan nilai baris pada kunci tertentu, atau `None` bila tidak ada.
///
/// Args:
/// - rows — baris terurut menurut kunci.
/// - key — kunci yang dicari.
///
/// Returns:
/// - Option<Vec<u8>> — salinan nilai baris bila kunci ditemukan.
///
/// Performance: pencarian biner karena `rows` dijamin terurut.
#[must_use]
pub fn value_of(rows: &[TableRow], key: &RowKey) -> Option<Vec<u8>> {
    rows.binary_search_by_key(&key, TableRow::key)
        .ok()
        .map(|position| rows[position].value().to_vec())
}
