//! File: `table_codec.rs`
//!
//! Deskripsi: Konversi isi tabel mentah menjadi baris terurut.
//! Layer: domain/tree
//! Tanggung jawab: Memisahkan kunci baris dan menjaga tabel dapat dibaca kembali.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `value-objects/row_key.rs`, `value-objects/table_row.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Pemisah kolom pada format tabel Verge.
const SEPARATOR: u8 = b',';

/// Tabel yang sudah diurai menjadi baris terurut.
///
/// Invariants:
/// - Baris terurut menaik menurut kunci dan kuncinya unik; baris dengan kunci
///   sama digantikan oleh baris terakhir (perilaku `last-write-wins`).
/// - Baris tanpa pemisah kolom ditolak dengan nomor baris, bukan dilewati diam-diam.
/// - Header disimpan terpisah dan ditulis ulang apa adanya saat tabel disusun.
///
/// Immutability: penuh.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TableRows {
    /// Baris pertama file apa adanya; ditulis ulang tanpa perubahan.
    header: Vec<u8>,
    /// Baris tabel terurut menurut kunci.
    rows: Vec<TableRow>,
}

impl TableRows {
    /// Mengubah isi tabel mentah menjadi baris terurut.
    ///
    /// Args:
    /// - raw — isi tabel apa adanya; baris pertama diperlakukan sebagai header.
    ///
    /// Returns:
    /// - Ok(TableRows) — baris terurut dan unik.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`MalformedTable`](VergeError::MalformedTable) bila ada
    /// baris tanpa pemisah kolom atau kunci kosong.
    ///
    /// Example:
    /// ```
    /// use verge_core::domain::tree::table_codec::TableRows;
    ///
    /// let rows = TableRows::parse(b"id,name\n2,budi\n1,ana\n").unwrap();
    /// assert_eq!(rows.len(), 2);
    /// assert_eq!(rows.get(0).unwrap().key().as_bytes(), b"1");
    /// assert_eq!(rows.to_bytes(), b"id,name\n1,ana\n2,budi\n");
    /// ```
    pub fn parse(raw: &[u8]) -> Result<Self> {
        let mut lines = raw.split(|byte| *byte == b'\n').enumerate();
        let Some((_, header)) = lines.next() else {
            return Ok(Self::default());
        };
        let header = header.to_vec();
        let mut rows: Vec<TableRow> = Vec::new();
        for (offset, line) in lines {
            if line.is_empty() {
                continue;
            }
            rows.push(parse_row(offset + 1, line)?);
        }
        Ok(Self {
            header,
            rows: normalize(rows),
        })
    }

    /// Menyusun tabel dari header dan baris yang sudah terurut.
    ///
    /// Args:
    /// - header — baris kolom apa adanya.
    /// - rows — baris data terurut menurut kunci.
    ///
    /// Returns:
    /// - `TableRows` — tabel siap ditulis ulang.
    #[must_use]
    pub fn from_parts(header: Vec<u8>, rows: Vec<TableRow>) -> Self {
        Self { header, rows }
    }

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

    /// Menyusun isi tabel dari header dan baris, siap ditulis sebagai snapshot.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.rows.iter().map(|row| row.value().len() + 4).sum());
        out.extend_from_slice(&self.header);
        out.push(b'\n');
        for row in &self.rows {
            // `value` sudah memuat pemisah kolom pertama, sehingga digabung
            // apa adanya agar baris dapat dibaca kembali persis seperti semula.
            out.extend_from_slice(row.key().as_bytes());
            out.extend_from_slice(row.value());
            out.push(b'\n');
        }
        out
    }
}

/// Mengurutkan baris dan menghilangkan kunci ganda.
fn normalize(rows: Vec<TableRow>) -> Vec<TableRow> {
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
fn parse_row(line_number: usize, line: &[u8]) -> Result<TableRow> {
    let Some(separator) = line.iter().position(|byte| *byte == SEPARATOR) else {
        return Err(malformed(line_number, "row has no column separator"));
    };
    if separator == 0 {
        return Err(malformed(line_number, "row key is empty"));
    }
    if line[separator..].iter().all(|byte| *byte == SEPARATOR) {
        return Err(malformed(line_number, "row has no value column"));
    }
    let row = TableRow::new(
        RowKey::new(line[..separator].to_vec()),
        line[separator..].to_vec(),
    );
    row.ok_or_else(|| malformed(line_number, "row key is empty"))
}

/// Membangun error tabel dengan nomor baris dan alasan yang aman ditampilkan.
fn malformed(line: usize, reason: &'static str) -> VergeError {
    VergeError::MalformedTable { line, reason }
}
