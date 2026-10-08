//! File: `mod.rs`
//!
//! Deskripsi: Konversi isi tabel mentah menjadi baris terurut.
//! Layer: domain/tree/codec
//! Tanggung jawab: Memisahkan kunci baris dan menjaga tabel dapat dibaca kembali.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `value-objects/row_key.rs`, `value-objects/table_row.rs`
//!   - `table_codec_rows.rs`, `table_reader.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!   - #97 (domain/tree split)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

// Pemisah kolom pada format tabel Verge; dipakai juga oleh parser baris.
pub(super) const SEPARATOR: u8 = b',';

// Tipe publik untuk eksternal
#[path = "table_codec_rows.rs"]
mod rows;
pub use rows::{malformed, normalize, parse_row, parse_rows};

#[path = "table_reader.rs"]
pub mod table_reader;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

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
    /// Membuat tabel kosong dengan header tertentu.
    pub fn new(header: Vec<u8>) -> Self {
        Self {
            header,
            rows: Vec::new(),
        }
    }

    /// Mengubah isi tabel mentah menjadi baris terurut.
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
    /// use verge_core::domain::tree::codec::TableRows;
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
            rows.push(rows::parse_row(offset + 1, line)?);
        }
        Ok(Self {
            header,
            rows: rows::normalize(rows),
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

    /// Menambahkan baris (untuk testing/mocking).
    pub fn push(&mut self, row: TableRow) {
        self.rows.push(row);
    }
}
