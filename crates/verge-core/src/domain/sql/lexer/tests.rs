//! File: `lexer/tests.rs`
//!
//! Deskripsi: Unit tes untuk lexer SQL.
//! Layer: domain/sql/lexer
//! Tanggung jawab: Memverifikasi tokenisasi keyword, operator, literal, parameter.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `super::mod.rs` (`lex`, `TokenKind`, `Keyword`, `Operator`)
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)
//!

use super::*;

#[test]
fn select_all() {
    let toks = lex("SELECT * FROM users").unwrap();
    assert_eq!(toks.len(), 5);
    assert_eq!(toks[0].kind, TokenKind::Keyword(Keyword::Select));
    assert_eq!(toks[1].kind, TokenKind::Star);
    assert_eq!(toks[2].kind, TokenKind::Keyword(Keyword::From));
    assert_eq!(toks[3].kind, TokenKind::Identifier("users".into()));
    assert!(matches!(toks[4].kind, TokenKind::Eof));
}

fn where_tokens() -> Vec<Token> {
    lex("WHERE age > 30").unwrap()
}

#[test]
fn operator_multi_char() {
    let toks = where_tokens();
    assert_eq!(toks[0].kind, TokenKind::Keyword(Keyword::Where));
    assert_eq!(toks[1].kind, TokenKind::Identifier("age".into()));
    assert_eq!(toks[2].kind, TokenKind::Operator(Operator::GreaterThan));
    assert_eq!(toks[3].kind, TokenKind::Number(30));
}

#[test]
fn string_literal_escape() {
    let toks = lex("'it''s'").unwrap();
    assert_eq!(toks.len(), 2);
    assert_eq!(toks[0].kind, TokenKind::String("it's".to_owned()));
}

#[test]
fn parameter_scan() {
    let toks = lex("@1767225600000").unwrap();
    assert_eq!(toks.len(), 2);
    assert_eq!(toks[0].kind, TokenKind::Parameter("@1767225600000".into()));
}

#[test]
fn angka_diluar_rentang() {
    assert!(lex("999999999999999999999999999999999999999").is_err());
}

#[test]
fn karakter_tak_dikenal() {
    assert!(lex("SELECT #@").is_err());
}

#[test]
fn string_tak_berkahir() {
    assert!(lex("'tidak selesai").is_err());
}

#[test]
fn batas_token() {
    let sql = "a,".repeat(9000);
    assert!(lex(&sql).is_err());
}
