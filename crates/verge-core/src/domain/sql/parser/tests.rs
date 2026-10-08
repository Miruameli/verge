//! File: `tests.rs`
//!
//! Deskripsi: Unit tes untuk parser SQL SELECT.
//! Layer: domain/sql/parser
//! Tanggung jawab: Memverifikasi parsing kolom, tabel, WHERE, dan AS OF.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `super::mod.rs` (`parse_sql`, `Parser`)
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)
//!

use super::*;

fn parse(sql: &str) -> SelectStatement {
    parse_sql(sql).expect("query harus valid")
}

#[test]
fn select_all_dari_tabel() {
    let stmt = parse("SELECT * FROM users");
    match &stmt.columns {
        ColumnList::All => {}
        ColumnList::Named(_) => panic!("harus All"),
    }
    assert_eq!(stmt.table, "users");
    assert!(stmt.where_clause.is_none());
    assert!(stmt.as_of.is_none());
}

#[test]
fn select_kolom_spesifik() {
    let stmt = parse("SELECT name, age FROM users");
    match &stmt.columns {
        ColumnList::Named(cols) => assert_eq!(cols, &["name", "age"]),
        ColumnList::All => panic!("harus Named"),
    }
}

#[test]
fn where_sederhana() {
    let stmt = parse("SELECT * FROM users WHERE age > 30");
    assert!(stmt.where_clause.is_some());
}

#[test]
fn where_dengan_and() {
    let stmt = parse("SELECT * FROM users WHERE age > 30 AND name = 'budi'");
    assert!(stmt.where_clause.is_some());
}

#[test]
fn as_of_timestamp() {
    let stmt = parse("SELECT * FROM users AS OF '2026-10-01T10:00:00Z'");
    assert_eq!(stmt.as_of, Some("2026-10-01T10:00:00Z".to_owned()));
}

#[test]
fn as_of_parameter() {
    let stmt = parse("SELECT * FROM users AS OF @1767225600000");
    assert_eq!(stmt.as_of, Some("@1767225600000".to_owned()));
}

#[test]
fn query_kosong_error() {
    assert!(parse_sql("").is_err());
}

#[test]
fn token_tak_terduga_error() {
    assert!(parse_sql("SELECT * FROM users WHERE").is_err());
}

#[test]
fn keyword_case_insensitive() {
    let stmt = parse("select * from users");
    assert_eq!(stmt.table, "users");
}

#[test]
fn table_valued_function() {
    let stmt = parse("SELECT * FROM commits()");
    assert_eq!(stmt.table, "commits");
    assert!(stmt.is_table_function);
}

#[test]
fn regular_table_not_function() {
    let stmt = parse("SELECT * FROM users");
    assert_eq!(stmt.table, "users");
    assert!(!stmt.is_table_function);
}

#[test]
fn function_dengan_where() {
    let stmt = parse("SELECT commit_id FROM commits() WHERE table = 'users'");
    assert!(stmt.is_table_function);
    assert!(stmt.where_clause.is_some());
}
