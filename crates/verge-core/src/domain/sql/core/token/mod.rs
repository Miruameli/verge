//! File: `mod.rs`
//!
//! Deskripsi: Token teks SQL yang dihasilkan lexer.
//! Layer: domain/sql/core
//! Tanggung jawab: Mendefinisikan `TokenKind`, `Keyword`, `Operator`, dan `Token`.
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
//!   - #96 (domain/sql split)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

/// Kata kunci SQL yang dikenali lexer secara case-insensitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    /// `SELECT`
    Select,
    /// `FROM`
    From,
    /// `WHERE`
    Where,
    /// `AS` (hanya dipakai dalam pasangan `AS OF`)
    As,
    /// `OF` (hanya dipakai dalam pasangan `AS OF`)
    Of,
    /// `AND`
    And,
    /// `OR`
    Or,
}

impl Keyword {
    /// Mencocokkan teks huruf kecil ke kata kunci, beri `None` bila bukan.
    #[must_use]
    pub fn from_lower(text: &str) -> Option<Self> {
        match text {
            "select" => Some(Self::Select),
            "from" => Some(Self::From),
            "where" => Some(Self::Where),
            "as" => Some(Self::As),
            "of" => Some(Self::Of),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            _ => None,
        }
    }
}

/// Operator perbandingan SQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
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

/// Jenis token hasil pemindaian lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// Kata kunci (SELECT, FROM, WHERE, AS, OF, AND, OR).
    Keyword(Keyword),
    /// Identifier: nama kolom, nama tabel, atau path refs.
    Identifier(String),
    /// Literal string berada di antara single quote.
    String(String),
    /// Literal angka bulat.
    Number(i64),
    /// Parameter `@<digit>` (misal `@1767225600000`).
    Parameter(String),
    /// Operator perbandingan.
    Operator(Operator),
    /// `,`
    Comma,
    /// `*` (semua kolom)
    Star,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// Akhir input.
    Eof,
}

/// Satu token hasil pemindaian, lengkap dengan teks asal dan posisi byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// Jenis token.
    pub kind: TokenKind,
    /// Teks asli token apa adanya di input.
    pub text: String,
    /// Offset byte di mana token mulai.
    pub offset: usize,
}

impl Token {
    /// Membuat token baru dari jenis, teks, dan offset.
    #[must_use]
    pub fn new(kind: TokenKind, text: &str, offset: usize) -> Self {
        Self {
            kind,
            text: text.to_owned(),
            offset,
        }
    }
}
