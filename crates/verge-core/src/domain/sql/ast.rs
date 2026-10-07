//! File: `ast.rs`
//!
//! Deskripsi: Tipe AST dan error untuk SQL query engine.
//! Layer: domain/sql
//! Tanggung jawab: Mendefinisikan `SelectStatement`, `Expr`, `SqlValue`, dan `SqlError`.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada)
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use core::fmt;

/// Error dari lexer atau parser SQL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlError {
    /// Offset byte di input SQL tempat error terdeteksi.
    pub offset: usize,
    /// Penjelasan singkat yang aman ditampilkan ke pengguna.
    pub message: String,
}

impl SqlError {
    /// Membuat error baru pada `offset` dengan `message`.
    #[must_use]
    pub fn at(offset: usize, message: impl Into<String>) -> Self {
        Self {
            offset,
            message: message.into(),
        }
    }
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SQL parse error at byte {}: {}",
            self.offset, self.message
        )
    }
}

impl std::error::Error for SqlError {}

/// Nilai literal dalam ekspresi SQL.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    /// Literal string (dari single-quoted literal).
    Text(String),
    /// Literal angka bulat.
    Number(i64),
}

/// Operator logika/boolean pada ekspresi WHERE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    /// `AND`
    And,
    /// `OR`
    Or,
    /// `=`
    Equal,
    /// `!=` atau `<>`
    NotEqual,
    /// `<`
    LessThan,
    /// `<=`
    LessEqual,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterEqual,
}

/// Ekspresi dalam klausa WHERE.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Referensi kolom, misalnya `age`.
    Column(String),
    /// Literal nilai, misalnya `30` atau `'budi'`.
    Value(SqlValue),
    /// Operasi biner: `left op right`.
    BinaryOp {
        /// Operator logika atau perbandingan.
        op: Op,
        /// Operan kiri.
        left: Box<Expr>,
        /// Operan kanan.
        right: Box<Expr>,
    },
}

/// Daftar kolom SELECT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnList {
    /// `SELECT *` — semua kolom.
    All,
    /// `SELECT col1, col2` — kolom yang tercantum.
    Named(Vec<String>),
}

/// AST untuk pernyataan SELECT.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectStatement {
    /// Kolom yang dipilih.
    pub columns: ColumnList,
    /// Nama tabel.
    pub table: String,
    /// Kondisi WHERE, bila ada.
    pub where_clause: Option<Expr>,
    /// Nilai `AS OF`, bila ada (di-format yang sama dengan `--as-of` CLI).
    pub as_of: Option<String>,
}
