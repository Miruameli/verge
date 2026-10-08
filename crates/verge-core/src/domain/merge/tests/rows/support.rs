//! File: `support.rs`
//!
//! Deskripsi: Helper bersama test merge level baris.
//! Layer: domain/merge/tests/rows
//! Tanggung jawab: Menyediakan bentuk hasil merge ringkas untuk assertion.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/{merge_strategy,row_merge}.rs`
//!   - `domain/tree/table_codec.rs`, `value-objects/table_row.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
//!
//! KONTEKS: helper ini sebelumnya ditulis ulang di dua berkas test baris;
//! KENAPA: bentuk hasil merge harus sama di semua assertion supaya kegagalan
//! dari berkas berbeda tidak dibandingkan dengan definisi yang berbeda.

use crate::domain::merge::merge_strategy::MergeStrategy;
use crate::domain::merge::rows::row_merge::merge_rows;
use crate::domain::tree::codec::TableRows;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Bentuk hasil merge agar assertion test tetap pendek.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Outcome {
    /// Jumlah baris hasil gabungan.
    pub(super) rows: usize,
    /// Kunci konflik yang tersisa.
    pub(super) conflicts: Vec<String>,
}

/// Mengurai tabel fixture menjadi baris terurut.
pub(super) fn rows(text: &[u8]) -> Vec<TableRow> {
    TableRows::parse(text)
        .expect("tabel fixture valid")
        .rows()
        .to_vec()
}

/// Menjalankan merge dengan `ours_is_newer` sesuai argumen.
pub(super) fn outcome(
    base: &[u8],
    ours: &[u8],
    theirs: &[u8],
    strategy: MergeStrategy,
    ours_newer: bool,
) -> Outcome {
    let merged = merge_rows(
        &rows(base),
        &rows(ours),
        &rows(theirs),
        strategy,
        ours_newer,
    );
    Outcome {
        rows: merged.rows.len(),
        conflicts: merged.conflicts.iter().map(|c| c.key.clone()).collect(),
    }
}

/// Menjalankan merge dengan branch aktif dianggap lebih baru.
pub(super) fn manual(base: &[u8], ours: &[u8], theirs: &[u8]) -> Outcome {
    outcome(base, ours, theirs, MergeStrategy::Manual, true)
}
