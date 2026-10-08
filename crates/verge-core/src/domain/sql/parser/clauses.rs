//! File: `clauses.rs`
//!
//! Deskripsi: Parser klausa AS OF, nilai waktu, dan table-valued function parens.
//! Layer: domain/sql/parser
//! Tanggung jawab: `parse_as_of`, `parse_when_value`, `parse_optional_parens`.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `super::mod.rs` (Parser), `ast.rs`, `token.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use super::super::ast::SqlError;
use super::super::token::{Keyword, TokenKind};
use super::Parser;

impl Parser<'_> {
    /// Mem-parse `AS OF <value>` jika ada.
    pub(super) fn parse_as_of(&mut self) -> Result<Option<String>, SqlError> {
        if self.peek().kind == TokenKind::Keyword(Keyword::As) {
            self.advance();
            self.expect_keyword(Keyword::Of, "`AS` harus diikuti `OF`")?;
            let when = self.parse_when_value()?;
            Ok(Some(when))
        } else {
            Ok(None)
        }
    }

    /// Mem-parse nilai AS OF: string literal, parameter, atau identifier.
    fn parse_when_value(&mut self) -> Result<String, SqlError> {
        let tok = self.peek();
        match &tok.kind {
            TokenKind::String(s) | TokenKind::Identifier(s) => {
                let s = s.clone();
                self.advance();
                Ok(s)
            }
            TokenKind::Parameter(p) => {
                let p = p.clone();
                self.advance();
                Ok(p)
            }
            _ => Err(SqlError::at(
                tok.offset,
                "`AS OF` mengharapkan string, parameter, atau nama",
            )),
        }
    }

    /// Mem-parse `()` opsional setelah nama tabel untuk table-valued function.
    ///
    /// `commits()` → `true`; `users` → `false`.
    pub(super) fn parse_optional_parens(&mut self) -> Result<bool, SqlError> {
        if self.peek().kind == TokenKind::LParen {
            self.advance();
            if self.peek().kind == TokenKind::RParen {
                self.advance();
                Ok(true)
            } else {
                Err(SqlError::at(
                    0,
                    "`(` harus langsung diikuti `)` untuk table-valued function",
                ))
            }
        } else {
            Ok(false)
        }
    }
}
