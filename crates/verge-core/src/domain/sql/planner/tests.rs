//! File: `tests.rs`
//!
//! Deskripsi: Unit tes untuk SQL planner.
//! Layer: domain/sql/planner
//! Tanggung jawab: Memverifikasi resolusi kolom, error kolom tidak ada,
//!   pewarisan AS OF, dan deteksi table-valued function.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `super::mod.rs` (`ExecutionPlan`, `plan_select`, `PlanError`)
//!   - `super::super::ast::*`, `super::super::parser::parse_sql`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!   - #92 (M5 Part 2: planner dan Verge extensions)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use super::*;
use crate::domain::sql::parser::parse_sql;

/// Header tabel contoh untuk tes.
fn sample_headers() -> Vec<String> {
    vec!["id".to_owned(), "name".to_owned(), "age".to_owned()]
}

#[test]
fn select_all_memetakan_semua_kolom() {
    let stmt = parse_sql("SELECT * FROM users").unwrap();
    let headers = sample_headers();
    let plan = plan_select(&stmt, &headers).unwrap();
    assert_eq!(plan.select_indices, vec![0, 1, 2]);
    assert_eq!(plan.select_names, headers);
    assert!(!plan.is_table_function);
}

#[test]
fn select_named_memetakan_indeks_benar() {
    let stmt = parse_sql("SELECT name, age FROM users").unwrap();
    let headers = sample_headers();
    let plan = plan_select(&stmt, &headers).unwrap();
    assert_eq!(plan.select_indices, vec![1, 2]);
    assert_eq!(plan.select_names, vec!["name", "age"]);
}

#[test]
fn kolom_tidak_ada_error() {
    let stmt = parse_sql("SELECT email FROM users").unwrap();
    let result = plan_select(&stmt, &sample_headers());
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("email"));
}

#[test]
fn as_of_inline_terpajetakan() {
    let stmt = parse_sql("SELECT * FROM users AS OF 'HEAD~1'").unwrap();
    let plan = plan_select(&stmt, &sample_headers()).unwrap();
    assert_eq!(plan.as_of, Some("HEAD~1".to_owned()));
}

#[test]
fn tanpa_as_of_nil() {
    let stmt = parse_sql("SELECT * FROM users").unwrap();
    let plan = plan_select(&stmt, &sample_headers()).unwrap();
    assert_eq!(plan.as_of, None);
}

#[test]
fn table_function_terdeteksi() {
    let stmt = parse_sql("SELECT * FROM commits()").unwrap();
    let plan = plan_select(&stmt, &sample_headers()).unwrap();
    assert!(plan.is_table_function);
    assert_eq!(plan.table, "commits");
}

#[test]
fn regular_table_bukan_function() {
    let stmt = parse_sql("SELECT * FROM users").unwrap();
    let plan = plan_select(&stmt, &sample_headers()).unwrap();
    assert!(!plan.is_table_function);
    assert_eq!(plan.table, "users");
}

#[test]
fn filter_where_dipindahkan_ke_plan() {
    let stmt = parse_sql("SELECT name FROM users WHERE age > 20").unwrap();
    let plan = plan_select(&stmt, &sample_headers()).unwrap();
    assert!(plan.filter.is_some());
}

#[test]
fn header_kosong_select_all_ok() {
    let stmt = parse_sql("SELECT * FROM users").unwrap();
    let plan = plan_select(&stmt, &[]).unwrap();
    assert!(plan.select_indices.is_empty());
}
