//! File: `row_resolution.rs`
//!
//! Deskripsi: Aturan resolusi satu kunci baris saat merge.
//! Layer: domain/merge/rows
//! Tanggung jawab: Memutuskan nilai akhir atau konflik untuk satu baris.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/merge_strategy.rs`, `row_conflict.rs`
//!   - `domain/tree/value-objects/{row_key,table_row}.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::merge::merge_strategy::MergeStrategy;
use crate::domain::merge::rows::row_conflict::RowConflict;
use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Nilai ketiga sisi untuk satu kunci; `None` berarti baris tidak ada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sides {
    /// Kunci baris yang sedang diputuskan.
    pub key: RowKey,
    /// Nilai pada base, atau `None` bila belum ada.
    pub base: Option<Vec<u8>>,
    /// Nilai pada branch aktif, atau `None` bila dihapus.
    pub ours: Option<Vec<u8>>,
    /// Nilai pada branch yang digabung, atau `None` bila dihapus.
    pub theirs: Option<Vec<u8>>,
}

/// Hasil resolusi satu kunci baris.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// Nilai final; `None` berarti baris dihapus dari tabel hasil.
    Merged(Option<Vec<u8>>),
    /// Konflik yang belum terselesaikan.
    Conflict(RowConflict),
}

/// Memutuskan nilai akhir satu kunci atau mencatat konfliknya.
///
/// KONTEKS: hanya satu sisi yang berubah dari base yang bukan konflik; KENAPA:
/// jika kedua sisi berubah dengan nilai berbeda, tidak ada jawaban yang benar
/// secara semantik, sedangkan kasus lain punya satu jawaban turunan.
///
/// Args:
/// - sides — nilai base, ours, dan theirs untuk satu kunci.
/// - strategy — cara menyelesaikan konflik.
/// - `ours_is_newer` — `true` bila commit branch aktif paling baru.
///
/// Returns:
/// - Resolution — nilai gabungan atau catatan konflik.
///
/// Example:
/// ```
/// use verge_core::domain::merge::merge_strategy::MergeStrategy;
/// use verge_core::domain::merge::rows::row_resolution::{resolve, Resolution, Sides};
/// use verge_core::domain::tree::value_objects::row_key::RowKey;
///
/// let sides = Sides {
///     key: RowKey::new(b"1".to_vec()),
///     base: Some(b",ana".to_vec()),
///     ours: Some(b",ana".to_vec()),
///     theirs: Some(b",budi".to_vec()),
/// };
/// assert!(matches!(
///     resolve(&sides, MergeStrategy::Manual, true),
///     Resolution::Merged(Some(_))
/// ));
///
/// let bentrok = Sides { ours: Some(b",citra".to_vec()), ..sides };
/// assert!(matches!(
///     resolve(&bentrok, MergeStrategy::Manual, true),
///     Resolution::Conflict(_)
/// ));
/// ```
#[must_use]
pub fn resolve(sides: &Sides, strategy: MergeStrategy, ours_is_newer: bool) -> Resolution {
    if sides.ours == sides.theirs {
        return Resolution::Merged(sides.ours.clone());
    }
    if sides.ours == sides.base {
        return Resolution::Merged(sides.theirs.clone());
    }
    if sides.theirs == sides.base {
        return Resolution::Merged(sides.ours.clone());
    }
    match strategy {
        MergeStrategy::Manual => Resolution::Conflict(RowConflict {
            key: String::from_utf8_lossy(sides.key.as_bytes()).into_owned(),
            base: sides.base.clone(),
            ours: sides.ours.clone(),
            theirs: sides.theirs.clone(),
        }),
        MergeStrategy::Ours => Resolution::Merged(sides.ours.clone()),
        MergeStrategy::Theirs => Resolution::Merged(sides.theirs.clone()),
        // KONTEKS: `ours` menang hanya bila commit branch aktif paling baru;
        // kedua sisi tidak pernah dianggap seri supaya hasilnya deterministik.
        MergeStrategy::LastWriteWins if ours_is_newer => Resolution::Merged(sides.ours.clone()),
        MergeStrategy::LastWriteWins => {
            if ours_is_newer {
                Resolution::Merged(sides.ours.clone())
            } else {
                Resolution::Merged(sides.theirs.clone())
            }
        }
    }
}

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
