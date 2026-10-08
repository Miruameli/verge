//! File: `sql_commits_tests.rs`
//!
//! Deskripsi: Test table-valued function `commits()` dan inline AS OF.
//! Layer: application/version-control/use-cases/tests/sql
//! Tanggung jawab: Membuktikan `SELECT * FROM commits()`, filter WHERE
//! terhadap hasil `commits()`, dan inline AS OF yang mengalahkan --as-of CLI.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `super::sql_query_tests::run_sql` (helper bersama)
//!   - `super::super::commit` (helper)
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!   - #92 (M5 Part 2)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)
//!   - ADR-0011 (`SCAN_LIMIT` untuk traversal commit)
//!   - ADR-0013 (SQL planner dan table-valued function)

use crate::application::version_control::fakes::world::FakeWorld;

use super::super::commit;
use super::sql_query_tests::run_sql;

#[test]
fn as_of_inline_mengalahkan_cli() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id,name,age\n1,ana,30\n", "v1", 1_000);
    commit(
        &world,
        "users",
        b"id,name,age\n1,ana,30\n2,budi,25\n",
        "v2",
        2_000,
    );
    // SQL memiliki AS OF inline ke HEAD~1; CLI memakai HEAD.
    // Hasil harus dari commit pertama (v1).
    assert_eq!(
        run_sql(&world, "SELECT name FROM users AS OF 'HEAD~1'"),
        b"name\nana\n"
    );
}

#[test]
fn commits_table_function() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id,name\n1,ana\n", "v1", 1_000);
    commit(&world, "users", b"id,name\n1,ana\n2,budi\n", "v2", 2_000);
    let result = run_sql(&world, "SELECT * FROM commits()");
    let text = String::from_utf8(result).expect("output harus UTF-8");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        lines[0],
        "commit_id,parent_ids,table,author,timestamp,message"
    );
    assert_eq!(lines.len(), 3);
    // v1: first commit, no parents → parent_ids kosong → `,,users,ana,1000,v1`
    assert!(lines.iter().any(|l| l.contains(",,users,ana,1000,v1")));
    // v2: second commit, parent = v1 → `,<hash>,users,ana,2000,v2`
    assert!(lines.iter().any(|l| l.contains("users,ana,2000,v2")));
}

#[test]
fn commits_with_where_filter() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id,name\n1,ana\n", "v1", 1_000);
    commit(&world, "users", b"id,name\n1,ana\n2,budi\n", "v2", 2_000);
    let result = run_sql(
        &world,
        "SELECT message FROM commits() WHERE timestamp = 2000",
    );
    let text = String::from_utf8(result).expect("output harus UTF-8");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "message");
    assert_eq!(lines.len(), 2);
    assert!(lines[1].contains("v2"));
}
