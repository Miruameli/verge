//! File: `table_codec_rows.rs`
//!
//! Deskripsi: Pembacaan, normalisasi, dan pembacaan field tabel.
//! Layer: domain/tree/codec
//! Tanggung jawab: Mengubah byte tabel menjadi baris terurut dan unik.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `value-objects/row_key.rs`, `value-objects/table_row.rs`
//!   - `table_codec.rs` (super module)
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!   - #97 (domain/tree split)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use super::{TableRows, SEPARATOR};

use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Mengurutkan baris dan menghilangkan kunci ganda.
///
/// Baris dengan kunci sama digantikan baris terakhir, sesuai perilaku
/// `last-write-wins` saat tabel diimpor dua kali dengan kunci yang sama.
pub fn normalize(rows: Vec<TableRow>) -> Vec<TableRow> {
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
pub fn parse_row(line_number: usize, line: &[u8]) -> Result<TableRow> {
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
pub fn malformed(line: usize, reason: &'static str) -> VergeError {
    VergeError::MalformedTable { line, reason }
}

/// Membaca baris tabel dari byte mentah.
///
/// Format:
/// - Baris 0: header (kolom dipisah ',')
/// - Baris 1..N: data (kolom dipisah ',')
/// - Setiap baris diakhiri newline '\n'
///
/// Returns (header_vec, rows_vec)
pub fn parse_rows(data: &[u8]) -> Result<TableRows> {
    let mut lines = data.split(|&b| b == b'\n');
    let header_line = lines.next().ok_or_else(|| VergeError::MalformedTable {
        line: 0,
        reason: "Tabel kosong: tidak ada header",
    })?;
    let header = header_line.to_vec();

    let mut rows = Vec::new();
    for (line_num, line) in lines.enumerate() {
        if line.is_empty() {
            continue;
        }
        // Parse row using parse_row
        let row = parse_row(line_num + 1, line)?;
        rows.push(row);
    }

    Ok(TableRows::new(header))
}
