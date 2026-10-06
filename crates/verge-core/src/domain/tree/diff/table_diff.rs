//! File: `table_diff.rs`
//!
//! Deskripsi: Perbedaan baris terurut antara dua tabel.
//! Layer: domain/tree/diff
//! Tanggung jawab: Menghasilkan tambah, ubah, dan hapus secara deterministik.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `../table_codec.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::tree::diff::row_change::RowChange;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Hasil perbandingan dua tabel.
///
/// Invariants:
/// - Perubahan terurut menaik menurut kunci.
/// - Tabel yang identik menghasilkan daftar kosong.
///
/// Immutability: penuh.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TableDiff {
    /// Perubahan yang ditemukan, terurut menurut kunci.
    pub changes: Vec<RowChange>,
}

impl TableDiff {
    /// Melaporkan apakah kedua tabel identik.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Menghitung perubahan antara tabel lama dan tabel baru.
    ///
    /// Args:
    /// - before — isi tabel lama.
    /// - after — isi tabel baru.
    ///
    /// Returns:
    /// - `TableDiff` — perubahan terurut menurut kunci.
    ///
    /// Performance: O(n + m) terhadap jumlah baris kedua tabel.
    ///
    /// Example:
    /// ```
    /// use verge_core::domain::tree::diff::table_diff::TableDiff;
    ///
    /// let diff = TableDiff::between(b"id,name\n1,ana\n", b"id,name\n1,budi\n2,sari\n");
    /// assert_eq!(diff.changes.len(), 2);
    /// assert!(TableDiff::between(b"id\n1,a\n", b"id\n1,a\n").is_empty());
    /// ```
    #[must_use]
    pub fn between(before: &[u8], after: &[u8]) -> Self {
        let before = parse_rows(before);
        let after = parse_rows(after);
        let mut changes = Vec::new();
        let mut left = 0;
        let mut right = 0;
        while left < before.len() || right < after.len() {
            let ordering = match (before.get(left), after.get(right)) {
                (Some(l), Some(r)) => l.key().cmp(r.key()),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => break,
            };
            match ordering {
                std::cmp::Ordering::Less => {
                    changes.push(removed(&before[left]));
                    left += 1;
                }
                std::cmp::Ordering::Greater => {
                    changes.push(added(&after[right]));
                    right += 1;
                }
                std::cmp::Ordering::Equal => {
                    let old = &before[left];
                    let new = &after[right];
                    if old.value() != new.value() {
                        changes.push(RowChange::Modified {
                            key: new.key().to_text(),
                            before: old.value().to_vec(),
                            after: new.value().to_vec(),
                        });
                    }
                    left += 1;
                    right += 1;
                }
            }
        }
        Self { changes }
    }
}

/// Mengurai isi tabel; tabel rusak dianggap kosong agar diff tidak berhenti total.
fn parse_rows(raw: &[u8]) -> Vec<TableRow> {
    TableRows::parse(raw)
        .map(|rows| rows.rows().to_vec())
        .unwrap_or_default()
}

/// Membentuk perubahan "baris dihapus".
fn removed(row: &TableRow) -> RowChange {
    RowChange::Removed {
        key: row.key().to_text(),
        value: row.value().to_vec(),
    }
}

/// Membentuk perubahan "baris ditambahkan".
fn added(row: &TableRow) -> RowChange {
    RowChange::Added {
        key: row.key().to_text(),
        value: row.value().to_vec(),
    }
}
