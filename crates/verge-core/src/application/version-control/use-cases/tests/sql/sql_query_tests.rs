//! File: `sql_query_tests.rs`
//!
//! Deskripsi: Test use case `sql_query` — lexer, parser, executor integrasi.
//! Layer: application/version-control/use-cases/tests/sql
//! Tanggung jawab: Membuktikan filter WHERE, proyeksi kolom, dan AS OF
//! terhadap tabel yang sudah ter-commit.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `use-cases/queries/sql_query.rs`
//!   - `domain/sql/{lexer,parser,executor}.rs`
//!   - `super::super::commit` (helper)
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)
//!   - ADR-0012 (SQL parser semantics and limits)

use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::use_cases::queries::sql_query::{
    sql_query, SqlQueryInput,
};
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;

use super::super::commit;

/// Menjalankan SQL query pada tabel `users` di HEAD via `FakeWorld`.
fn run_sql(world: &FakeWorld, sql: &str) -> Vec<u8> {
    sql_query(
        &SqlQueryInput {
            table: TableName::parse("users").unwrap(),
            as_of: "HEAD".to_owned(),
            sql: sql.to_owned(),
        },
        &world,
        &world,
        &world,
        &world,
    )
    .expect("query berhasil")
}

#[test]
fn select_semua_kolom() {
    let world = FakeWorld::new();
    commit(
        &world,
        "users",
        b"id,name,age\n1,ana,30\n2,budi,25\n",
        "seed",
        1_000,
    );
    assert_eq!(
        run_sql(&world, "SELECT * FROM users"),
        b"id,name,age\n1,ana,30\n2,budi,25\n"
    );
}

#[test]
fn select_kolom_spesifik() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id,name,age\n1,ana,30\n", "seed", 1_000);
    assert_eq!(run_sql(&world, "SELECT name FROM users"), b"name\nana\n");
}

#[test]
fn where_komparasi() {
    let world = FakeWorld::new();
    commit(
        &world,
        "users",
        b"id,name,age\n1,ana,30\n2,budi,25\n3,carl,18\n",
        "seed",
        1_000,
    );
    assert_eq!(
        run_sql(&world, "SELECT name FROM users WHERE age > 20"),
        b"name\nana\nbudi\n"
    );
}

#[test]
fn where_dengan_and() {
    let world = FakeWorld::new();
    commit(
        &world,
        "users",
        b"id,name,age\n1,ana,30\n2,budi,25\n3,carl,18\n",
        "seed",
        1_000,
    );
    assert_eq!(
        run_sql(&world, "SELECT name FROM users WHERE age > 20 AND id > 1"),
        b"name\nbudi\n"
    );
}

#[test]
fn where_dengan_or() {
    let world = FakeWorld::new();
    commit(
        &world,
        "users",
        b"id,name,age\n1,ana,30\n2,budi,25\n3,carl,18\n",
        "seed",
        1_000,
    );
    assert_eq!(
        run_sql(&world, "SELECT name FROM users WHERE age = 18 OR id = 1"),
        b"name\nana\ncarl\n"
    );
}

#[test]
fn as_of_membaca_commit_lama() {
    let world = FakeWorld::new();
    let first = commit(&world, "users", b"id,name,age\n1,ana,30\n", "v1", 1_000);
    commit(
        &world,
        "users",
        b"id,name,age\n1,ana,30\n2,budi,25\n",
        "v2",
        2_000,
    );
    let input = SqlQueryInput {
        table: TableName::parse("users").unwrap(),
        as_of: first.id.to_string(),
        sql: "SELECT name FROM users".to_owned(),
    };
    let result = sql_query(&input, &world, &world, &world, &world).expect("query berhasil");
    assert_eq!(result, b"name\nana\n");
}

#[test]
fn sql_tidak_valid_error() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id,name\n1,ana\n", "seed", 1_000);
    let result = sql_query(
        &SqlQueryInput {
            table: TableName::parse("users").unwrap(),
            as_of: "HEAD".to_owned(),
            sql: "INVALID SQL".to_owned(),
        },
        &world,
        &world,
        &world,
        &world,
    );
    assert!(result.is_err(), "SQL tidak valid harus error");
}

#[test]
fn tabel_lain_error() {
    let world = FakeWorld::new();
    let users = commit(&world, "users", b"id,name\n1,ana\n", "seed", 1_000);
    let input = SqlQueryInput {
        table: TableName::parse("orders").unwrap(),
        as_of: users.id.to_string(),
        sql: "SELECT * FROM orders".to_owned(),
    };
    let error =
        sql_query(&input, &world, &world, &world, &world).expect_err("tabel lain harus error");
    assert!(
        matches!(error, VergeError::CommitBelongsToOtherTable { .. }),
        "harus error tabel lain: {error:?}"
    );
}
