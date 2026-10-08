//! File: `mod.rs`
//!
//! Deskripsi: Parser SQL untuk pernyataan SELECT.
//! Layer: domain/sql/parser
//! Tanggung jawab: Mengubah token-token menjadi `SelectStatement` AST.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `lexer.rs`, `ast.rs`, `token.rs`, `expr.rs`, `clauses.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use super::ast::{ColumnList, Expr, SelectStatement, SqlError};
use super::lexer::lex;
use super::token::{Keyword, Token, TokenKind};

mod clauses;
mod expr;

/// Batas kedalaman nesting ekspresi WHERE.
const MAX_WHERE_DEPTH: i32 = 16;

/// Mem-parse `sql` menjadi `SelectStatement`.
///
/// Melakukan dua tahap: *lex* dulu, lalu *parse*. Semua error berisi offset
/// byte dan penjelasan singkat.
///
/// # Errors
/// Mengembalikan `SqlError` jika:
/// - lexer gagal
/// - query tidak dimulai dengan SELECT
/// - klausa FROM tidak ditemukan
/// - nama tabel tidak valid
/// - klausa WHERE tidak valid
/// - klausa AS OF tidak valid
/// - token berlebih setelah akhir query
pub fn parse_sql(sql: &str) -> Result<SelectStatement, SqlError> {
    let tokens = lex(sql)?;
    let mut parser = Parser::new(&tokens);
    let stmt = parser.parse_select()?;
    parser.expect_eof()?;
    Ok(stmt)
}

/// Parser rekursi-ke-kiri untuk token-token SQL.
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    /// Membuat parser dari slice token yang dihasilkan lexer.
    #[must_use]
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Token saat ini tanpa melangkap posisi.
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&EOF_TOKEN)
    }

    /// Konsumsi token saat ini, lalu maju satu.
    ///
    /// Karena semua pemanggil (lexer/parser) tidak memerlukan nilai kembalian,
    /// metode ini mengembalikan unit untuk menghindari konflik borrow dengan
    /// `self.pos`.
    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    /// Memaksa token saat ini menjadi keyword tertentu.
    fn expect_keyword(&mut self, kw: Keyword, context: &str) -> Result<(), SqlError> {
        let tok = self.peek();
        if tok.kind == TokenKind::Keyword(kw) {
            self.advance();
            Ok(())
        } else {
            Err(SqlError::at(
                tok.offset,
                format!("{context}; diperoleh `{}`", tok.text),
            ))
        }
    }

    /// Memastikan tidak ada token setelah akhir query.
    fn expect_eof(&mut self) -> Result<(), SqlError> {
        let tok = self.peek();
        if matches!(tok.kind, TokenKind::Eof) {
            Ok(())
        } else {
            Err(SqlError::at(
                tok.offset,
                format!("token tak terduga `{}` setelah akhir query", tok.text),
            ))
        }
    }

    /// Mem-parse `SELECT ... FROM ... [WHERE ...] [AS OF ...]`.
    ///
    /// Nama tabel boleh berupa table-valued function: `commits()`.
    fn parse_select(&mut self) -> Result<SelectStatement, SqlError> {
        self.expect_keyword(Keyword::Select, "query harus dimulai dengan SELECT")?;
        let columns = self.parse_column_list()?;
        self.expect_keyword(Keyword::From, "SELECT harus diikuti FROM")?;
        let table = self.parse_identifier("nama tabel")?;
        let is_function = self.parse_optional_parens()?;
        let where_clause = self.parse_where_clause(0)?;
        let as_of = self.parse_as_of()?;
        Ok(SelectStatement {
            columns,
            table,
            where_clause,
            as_of,
            is_table_function: is_function,
        })
    }

    /// Mem-parse `*` atau `col1, col2, ...`.
    fn parse_column_list(&mut self) -> Result<ColumnList, SqlError> {
        let tok = self.peek();
        if tok.kind == TokenKind::Star {
            self.advance();
            Ok(ColumnList::All)
        } else {
            let mut cols = vec![self.parse_identifier("nama kolom")?];
            while self.peek().kind == TokenKind::Comma {
                self.advance();
                cols.push(self.parse_identifier("nama kolom")?);
            }
            Ok(ColumnList::Named(cols))
        }
    }

    /// Mem-parse klausa WHERE jika ada.
    fn parse_where_clause(&mut self, depth: i32) -> Result<Option<Expr>, SqlError> {
        if depth > MAX_WHERE_DEPTH {
            return Err(SqlError::at(0, "kedalaman WHERE melebihi batas"));
        }
        if self.peek().kind == TokenKind::Keyword(Keyword::Where) {
            self.advance();
            Ok(Some(self.parse_expression(depth)?))
        } else {
            Ok(None)
        }
    }

    /// Mem-parse identifier sederhana (kolom, tabel, dsb).
    fn parse_identifier(&mut self, context: &str) -> Result<String, SqlError> {
        let tok = self.peek();
        match &tok.kind {
            TokenKind::Identifier(s) => {
                let s = s.clone();
                self.advance();
                Ok(s)
            }
            _ => Err(SqlError::at(
                tok.offset,
                format!("{context}; diperoleh `{}`", tok.text),
            )),
        }
    }
}

/// Token EOF statis agar `peek` tidak perlu cek batas setiap kali.
static EOF_TOKEN: Token = Token {
    kind: TokenKind::Eof,
    text: String::new(),
    offset: usize::MAX,
};

/// Mem-parse ekspresi WHERE — delegasi ke modul `expr`.
impl Parser<'_> {
    /// Mem-parse ekspresi dengan precedensi OR > AND > perbandingan.
    fn parse_expression(&mut self, depth: i32) -> Result<Expr, SqlError> {
        self.parse_disjunction(depth)
    }
}

#[cfg(test)]
mod tests;
