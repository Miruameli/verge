//! File: `table_codec_rows.rs`
//!
//! Deskripsi: Pembacaan, normalisasi, dan pembacaan field tabel.
//! Layer: domain/tree
//! Tanggung jawab: Mengubah byte tabel menjadi baris terurut dan unik.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `value-objects/row_key.rs`, `value-objects/table_row.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::tree::table_codec::{TableRows, SEPARATOR};

use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Mengurutkan baris dan menghilangkan kunci ganda.
///
/// Baris dengan kunci sama digantikan baris terakhir, sesuai perilaku
/// `last-write-wins` saat tabel diimpor dua kali dengan kunci yang sama.
pub(super) fn normalize(rows: Vec<TableRow>) -> Vec<TableRow> {
    let mut sorted: Vec<TableRow> = Vec::with_capacity(rows.len());
    for row in rows {
        match sorted.binary_search_by_key(&row.key(), |existing| &existing.key) {
            Ok(position) => sorted[position] = row,
            Err(position) => sorted.insert(position, row),
        }
    }
    sorted
}

/// Mengurai satu baris menjadi kunci dan nilai.
///
/// Nilai menyimpan seluruh kolom setelah kunci, termasuk pemisah kolom pertama,
/// sehingga baris dapat disusun kembali tanpa kehilangan informasi format.
pub(super) fn parse_row(line_number: usize, line: &[u8]) -> Result<TableRow> {
    let Some(separator) = line.iter().position(|byte| *byte == SEPARATOR) else {
        return Err(malformed(line_number, "row has no column separator"));
    };
    if separator == 0 {
        return Err(malformed(line_number, "row key is empty"));
    }
    if line[separator..].iter().all(|byte| *byte == SEPARATOR) {
        return Err(malformed(line_number, "row has no value column"));
    }
    TableRow::new(
        RowKey::new(line[..separator].to_vec()),
        line[separator..].to_vec(),
    )
    .ok_or_else(|| malformed(line_number, "row key is empty"))
}

/// Membangun error tabel dengan nomor baris dan alasan yang aman ditampilkan.
pub(super) fn malformed(line: usize, reason: &'static str) -> VergeError {
    VergeError::MalformedTable { line, reason }
}

impl TableRows {
    /// Mengembalikan jumlah baris data.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Melaporkan apakah tabel tidak memiliki baris data.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Mengembalikan baris pada posisi tertentu.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&TableRow> {
        self.rows.get(index)
    }

    /// Mengembalikan seluruh baris.
    #[must_use]
    pub fn rows(&self) -> &[TableRow] {
        &self.rows
    }

    /// Mengembalikan header tabel apa adanya.
    #[must_use]
    pub fn header(&self) -> &[u8] {
        &self.header
    }
}
