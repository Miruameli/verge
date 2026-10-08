//! File: `lexer/mod.rs`
//!
//! Deskripsi: Lexer untuk kode SQL.
//! Layer: domain/sql/lexer
//! Tanggung jawab: Memindai teks SQL menjadi token-token.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `super::ast::SqlError`, `super::token::{Keyword, Operator, Token, TokenKind}`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use super::ast::SqlError;
use super::token::{Keyword, Operator, Token, TokenKind};

/// Batas jumlah token maksimal (cegah input berjalan memakan memori).
const MAX_TOKENS: usize = 8_192;

/// Memindai `input` menjadi token-token termasuk `Eof` di ujung.
///
/// # Errors
/// Mengembalikan `SqlError` bila:
/// - string tidak diakhiri (unterminated string)
/// - karakter tidak dikenali
/// - angka di luar rentang `i64`
/// - token melebihi batas `MAX_TOKENS`
pub fn lex(input: &str) -> Result<Vec<Token>, SqlError> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut pos = 0usize;

    while pos < bytes.len() {
        if bytes[pos].is_ascii_whitespace() {
            pos += 1;
            continue;
        }
        let start = pos;
        let (kind, end) = scan_token(input, bytes, pos)?;
        let text = input[start..end].to_owned();
        tokens.push(Token::new(kind, &text, start));
        pos = end;
        if tokens.len() > MAX_TOKENS {
            return Err(SqlError::at(start, "jumlah token melebihi batas"));
        }
    }

    tokens.push(Token::new(TokenKind::Eof, "", pos));
    Ok(tokens)
}

/// Memindai satu token pada `pos`.
fn scan_token(input: &str, bytes: &[u8], pos: usize) -> Result<(TokenKind, usize), SqlError> {
    let ch = bytes[pos];
    Ok(match ch {
        b',' => (TokenKind::Comma, pos + 1),
        b'*' => (TokenKind::Star, pos + 1),
        b'(' => (TokenKind::LParen, pos + 1),
        b')' => (TokenKind::RParen, pos + 1),
        b'@' => return scan_parameter(bytes, pos),
        b'\'' => return scan_string_literal(input, bytes, pos),
        c if c.is_ascii_digit() => return scan_number(bytes, pos),
        c if is_ident_start(c) => return Ok(scan_identifier(input, bytes, pos)),
        b'=' => (TokenKind::Operator(Operator::Equal), pos + 1),
        b'!' if peek_eq(bytes, pos, b'=') => (TokenKind::Operator(Operator::NotEqual), pos + 2),
        b'<' if peek_eq(bytes, pos, b'>') => (TokenKind::Operator(Operator::NotEqual), pos + 2),
        b'<' if peek_eq(bytes, pos, b'=') => (TokenKind::Operator(Operator::LessEqual), pos + 2),
        b'<' => (TokenKind::Operator(Operator::LessThan), pos + 1),
        b'>' if peek_eq(bytes, pos, b'=') => (TokenKind::Operator(Operator::GreaterEqual), pos + 2),
        b'>' => (TokenKind::Operator(Operator::GreaterThan), pos + 1),
        _ => {
            return Err(SqlError::at(
                pos,
                format!("karakter tak dikenal `{}`", ch as char),
            ))
        }
    })
}

/// Membantu memeriksa karakter berikutnya sama dengan `expected`.
fn peek_eq(bytes: &[u8], pos: usize, expected: u8) -> bool {
    pos + 1 < bytes.len() && bytes[pos + 1] == expected
}

/// Melaporkan apakah `byte` dapat memulai identifier.
fn is_ident_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic()
}

/// Melaporkan apakah `byte` dapat melanjutkan identifier.
fn is_ident_cont(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-' || byte == b'/'
}

/// Memindai `@<digit>` sebagai parameter.
fn scan_parameter(bytes: &[u8], pos: usize) -> Result<(TokenKind, usize), SqlError> {
    let start = pos;
    let mut end = pos + 1;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    if end == start + 1 {
        return Err(SqlError::at(
            start,
            "`@` harus diikuti setidaknya satu digit",
        ));
    }
    let text = String::from_utf8(bytes[start..end].to_vec())
        .map_err(|_| SqlError::at(start, "parameter bukan UTF-8 valid"))?;
    Ok((TokenKind::Parameter(text), end))
}

/// Memindai angka bulat sebagai `i64`.
fn scan_number(bytes: &[u8], pos: usize) -> Result<(TokenKind, usize), SqlError> {
    let start = pos;
    let mut end = pos;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    let value: i64 = std::str::from_utf8(&bytes[start..end])
        .unwrap_or("0")
        .parse()
        .map_err(|_| SqlError::at(start, "angka di luar rentang i64"))?;
    Ok((TokenKind::Number(value), end))
}

/// Memindai identifier atau keyword (case-insensitive).
fn scan_identifier(input: &str, bytes: &[u8], pos: usize) -> (TokenKind, usize) {
    let start = pos;
    let mut end = pos;
    while end < bytes.len() && is_ident_cont(bytes[end]) {
        end += 1;
    }
    let text = &input[start..end];
    match Keyword::from_lower(&text.to_lowercase()) {
        Some(kw) => (TokenKind::Keyword(kw), end),
        None => (TokenKind::Identifier(text.to_owned()), end),
    }
}

/// Memindai string literal single-quote dengan `''` sebagai escape kutip.
fn scan_string_literal(
    input: &str,
    bytes: &[u8],
    start: usize,
) -> Result<(TokenKind, usize), SqlError> {
    let mut pos = start + 1;
    let mut value = String::new();
    while pos < bytes.len() {
        if bytes[pos] == b'\'' {
            if pos + 1 < bytes.len() && bytes[pos + 1] == b'\'' {
                value.push('\'');
                pos += 2;
            } else {
                return Ok((TokenKind::String(value), pos + 1));
            }
        } else {
            let ch = input[pos..].chars().next().unwrap();
            value.push(ch);
            pos += ch.len_utf8();
        }
    }
    Err(SqlError::at(start, "string literal tidak berkahir"))
}

#[cfg(test)]
mod tests;
