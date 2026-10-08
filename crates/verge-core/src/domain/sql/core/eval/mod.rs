//! File: `mod.rs`
//!
//! Deskripsi: Evaluasi ekspresi WHERE pada baris tabel.
//! Layer: domain/sql/core
//! Tanggung jawab: Menyelesaikan operand, membandingkan nilai, dan
//! mengevaluasi ekspresi biner untuk filter WHERE.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `ast.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!   - #92 (M5 Part 2: planner dan Verge extensions)
//!   - #96 (domain/sql split)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use super::ast::{Expr, Op, SqlError, SqlValue};

/// Nilai yang diselesaikan dari kolom atau literal selama evaluasi.
enum Resolved {
    Text(String),
    Number(i64),
}

impl Resolved {
    /// Mengembalikan representasi teks untuk perbandingan string.
    fn as_text(&self) -> String {
        match self {
            Self::Text(s) => s.clone(),
            Self::Number(n) => n.to_string(),
        }
    }
}

/// Menyelesaikan operand menjadi `Resolved`.
fn resolve_operand(expr: &Expr, cols: &[&[u8]], names: &[String]) -> Result<Resolved, SqlError> {
    match expr {
        Expr::Value(SqlValue::Number(n)) => Ok(Resolved::Number(*n)),
        Expr::Value(SqlValue::Text(s)) => Ok(Resolved::Text(s.clone())),
        Expr::Column(name) => {
            let value = names
                .iter()
                .position(|n| n == name)
                .and_then(|i| cols.get(i))
                .copied()
                .ok_or_else(|| SqlError::at(0, format!("kolom `{name}` tidak ditemukan")))?;
            let text = String::from_utf8_lossy(value).into_owned();
            match text.parse::<i64>() {
                Ok(n) => Ok(Resolved::Number(n)),
                Err(_) => Ok(Resolved::Text(text)),
            }
        }
        Expr::BinaryOp { .. } => Err(SqlError::at(0, "operand tidak boleh berupa ekspresi biner")),
    }
}

/// Membandingkan dua nilai yang sudah diselesaikan.
fn compare(op: Op, left: &Resolved, right: &Resolved) -> bool {
    let (a_num, b_num) = match (left, right) {
        (Resolved::Number(a), Resolved::Number(b)) => (Some(*a), Some(*b)),
        _ => (None, None),
    };
    if let (Some(a), Some(b)) = (a_num, b_num) {
        match op {
            Op::Equal => a == b,
            Op::NotEqual => a != b,
            Op::LessThan => a < b,
            Op::LessEqual => a <= b,
            Op::GreaterThan => a > b,
            Op::GreaterEqual => a >= b,
            Op::And | Op::Or => false,
        }
    } else {
        let a = left.as_text();
        let b = right.as_text();
        match op {
            Op::Equal => a == b,
            Op::NotEqual => a != b,
            Op::LessThan => a < b,
            Op::LessEqual => a <= b,
            Op::GreaterThan => a > b,
            Op::GreaterEqual => a >= b,
            Op::And | Op::Or => false,
        }
    }
}

/// Mengevaluasi ekspresi WHERE terhadap kolom baris.
pub(super) fn eval_expr(expr: &Expr, cols: &[&[u8]], names: &[String]) -> Result<bool, SqlError> {
    match expr {
        Expr::BinaryOp { op, left, right } => match op {
            Op::And => Ok(eval_expr(left, cols, names)? && eval_expr(right, cols, names)?),
            Op::Or => Ok(eval_expr(left, cols, names)? || eval_expr(right, cols, names)?),
            Op::Equal
            | Op::NotEqual
            | Op::LessThan
            | Op::LessEqual
            | Op::GreaterThan
            | Op::GreaterEqual => {
                let lv = resolve_operand(left, cols, names)?;
                let rv = resolve_operand(right, cols, names)?;
                Ok(compare(*op, &lv, &rv))
            }
        },
        _ => Err(SqlError::at(0, "ekspresi WHERE harus berupa perbandingan")),
    }
}
