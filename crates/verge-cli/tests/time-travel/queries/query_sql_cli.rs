//! File: `query_sql_cli.rs`
//!
//! Deskripsi: Test end-to-end `verge query "<SQL>"`.
//! Layer: interfaces/cli/tests/time-travel/queries
//! Tanggung jawab: Membuktikan SQL SELECT, WHERE, filter, dan proyeksi kolom
//! lewat binary CLI.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `query_fixtures.rs`, `support/mod.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)
//!   - ADR-0012 (SQL parser semantics and limits)

use super::query_fixtures::{commit_time, timeline};
use super::support::verge_stdout;

#[test]
fn query_sql_select_all() {
    let dir = timeline("sql-select-all");
    let out = verge_stdout(
        &dir,
        &[
            "query",
            "--table",
            "users",
            "--as-of",
            "HEAD",
            "SELECT * FROM users",
        ],
    );
    assert!(out.contains("1,ana"), "{out}");
    assert!(out.contains("2,budi"), "{out}");
    assert!(out.contains("3,citra"), "{out}");
}

#[test]
fn query_sql_where_and_project() {
    let dir = timeline("sql-where-project");
    let out = verge_stdout(
        &dir,
        &[
            "query",
            "--table",
            "users",
            "--as-of",
            "HEAD",
            "SELECT name FROM users WHERE id > 1",
        ],
    );
    assert!(out.contains("budi"), "{out}");
    assert!(out.contains("citra"), "{out}");
    assert!(!out.contains("ana"), "harusnya terfilter: {out}");
}

#[test]
fn query_sql_as_of_commit_lama() {
    let dir = timeline("sql-as-of");
    let oldest = commit_time(&dir, 2);

    let out = verge_stdout(
        &dir,
        &[
            "query",
            "--table",
            "users",
            "--as-of",
            &oldest,
            "SELECT name FROM users",
        ],
    );
    assert!(out.contains("ana"), "{out}");
    assert!(!out.contains("budi"), "commit lama tidak punya budi: {out}");
}
