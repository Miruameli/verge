//! File: `executor.rs`
//!
//! Deskripsi: Evaluasi WHERE dan proyeksi kolom pada table rows.
//! Layer: domain/sql
//! Tanggung jawab: Menyaring baris berdasarkan ekspresi WHERE dan memilih kolom.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `ast.rs`
//!   - `eval.rs` (evaluasi WHERE)
//!   - `domain/tree/table_codec.rs`, `domain/tree/value-objects/table_row.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)
//!   - ADR-0012 (SQL parser semantics and limits)

use super::ast::{ColumnList, SelectStatement, SqlError};
use super::eval::eval_expr;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Memecah header CSV menjadi nama kolom.
///
/// # Errors
/// Returns `SqlError` if header contains non-UTF-8 bytes or is empty.
pub fn header_columns(header: &[u8]) -> Result<Vec<String>, SqlError> {
    let text =
        std::str::from_utf8(header).map_err(|_| SqlError::at(0, "header berisi byte non-UTF-8"))?;
    if text.is_empty() {
        return Err(SqlError::at(0, "tabel tidak memiliki header kolom"));
    }
    Ok(text.split(',').map(|s| s.trim().to_owned()).collect())
}

/// Memecah satu baris menjadi nilai kolom berdasarkan jumlah kolom header.
///
/// Nilai kolom terakhir dapat mengandung koma, karena format CSV Verge tidak
/// memakai quoting (ADR-0006).  `splitn` memastikan koma ekstra jatuh ke kolom
/// terakhir.
fn row_values(row: &TableRow, num_cols: usize) -> Vec<&[u8]> {
    let key = row.key().as_bytes();
    if num_cols <= 1 {
        return vec![key];
    }
    let rest = row.value();
    let rest = if rest.starts_with(b",") {
        &rest[1..]
    } else {
        rest
    };
    let mut cols: Vec<&[u8]> = Vec::with_capacity(num_cols);
    cols.push(key);
    cols.extend(rest.splitn(num_cols - 1, |&b| b == b','));
    while cols.len() < num_cols {
        cols.push(b"");
    }
    cols.truncate(num_cols);
    cols
}

/// Menulis kolom yang dipilih ke buffer CSV.
fn write_selected<'a>(out: &mut Vec<u8>, indices: &[usize], resolve: impl Fn(usize) -> &'a [u8]) {
    for (i, &idx) in indices.iter().enumerate() {
        if i > 0 {
            out.push(b',');
        }
        out.extend_from_slice(resolve(idx));
    }
    out.push(b'\n');
}

/// Menjalankan query SQL terhadap `rows` dan mengembalikan CSV hasil.
///
/// - Header dipakai untuk memetakan nama kolom ke indeks.
/// - WHERE (jika ada) menyaring baris.
/// - Proyeksi kolom memilih kolom berdasarkan daftar SELECT.
///
/// # Errors
/// Mengembalikan `SqlError` bila header kolom tidak valid UTF-8,
/// kolom yang dipilih tidak ada di header, kolom duplikat dalam proyeksi,
/// atau evaluasi WHERE gagal.
pub fn execute_query(rows: &TableRows, stmt: &SelectStatement) -> Result<Vec<u8>, SqlError> {
    let names = header_columns(rows.header())?;
    let num_cols = names.len();

    let indices: Vec<usize> = match &stmt.columns {
        ColumnList::All => (0..num_cols).collect(),
        ColumnList::Named(cols) => cols
            .iter()
            .map(|c| {
                names
                    .iter()
                    .position(|n| n == c)
                    .ok_or_else(|| SqlError::at(0, format!("kolom `{c}` tidak ditemukan")))
            })
            .collect::<Result<Vec<_>, _>>()?,
    };

    let mut out = Vec::new();
    write_selected(&mut out, &indices, |i| names[i].as_bytes());

    for row in rows.rows() {
        let cols = row_values(row, num_cols);
        let pass = match &stmt.where_clause {
            Some(expr) => eval_expr(expr, &cols, &names)?,
            None => true,
        };
        if !pass {
            continue;
        }
        write_selected(&mut out, &indices, |i| cols.get(i).copied().unwrap_or(b""));
    }

    Ok(out)
}

/// Menjalankan `ExecutionPlan` terhadap `rows` dan mengembalikan CSV hasil.
///
/// Berbeda dengan `execute_query`, kolom sudah diselesaikan ke indeks
/// pada planner sehingga eksekutor tidak perlu mencari nama lagi.
///
/// # Errors
///
/// Mengembalikan `SqlError` bila header tidak valid UTF-8, atau evaluasi
/// WHERE gagal.
pub fn execute_plan(
    rows: &TableRows,
    plan: &super::planner::ExecutionPlan,
) -> Result<Vec<u8>, SqlError> {
    let names = header_columns(rows.header())?;
    let num_cols = names.len();
    let indices = &plan.select_indices;
    let filter = &plan.filter;
    let mut out = Vec::new();
    write_selected(&mut out, indices, |i| names[i].as_bytes());
    for row in rows.rows() {
        let cols = row_values(row, num_cols);
        let pass = match filter {
            Some(expr) => eval_expr(expr, &cols, &names)?,
            None => true,
        };
        if !pass {
            continue;
        }
        write_selected(&mut out, indices, |i| cols.get(i).copied().unwrap_or(b""));
    }
    Ok(out)
}
