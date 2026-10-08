//! File: `mod.rs`
//!
//! Deskripsi: Planner SQL — mengubah AST `SelectStatement` menjadi `ExecutionPlan`.
//! Layer: domain/sql/planner
//! Tanggung jawab: Meresolusi nama kolom ke indeks, memvalidasi klausa WHERE,
//!   menerapkan `AS OF` inline, dan mendeteksi table-valued function.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `super::ast::{ColumnList, Expr, SelectStatement, SqlError}`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!   - #92 (M5 Part 2: planner dan Verge extensions)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use super::ast::{ColumnList, Expr, SelectStatement, SqlError};

/// Error dari planner SQL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanError {
    /// Penjelasan singkat yang aman ditampilkan ke pengguna.
    pub message: String,
}

impl PlanError {
    /// Membuat error planner baru.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SQL plan error: {}", self.message)
    }
}

impl std::error::Error for PlanError {}

impl From<PlanError> for SqlError {
    fn from(e: PlanError) -> Self {
        SqlError::at(0, e.message)
    }
}

/// Rencana eksekusi hasil planning AST.
///
/// Dibuat oleh [`plan_select`] setelah menerima header kolom tabel.
/// Planner menyelesaikan nama kolom ke indeks sekaligus, sehingga
/// eksekutor tidak perlu mencari lagi untuk setiap baris.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionPlan {
    /// Indeks kolom yang dipilih (0-based), terurut sesuai daftar SELECT.
    pub select_indices: Vec<usize>,
    /// Nama kolom hasil untuk header CSV output.
    pub select_names: Vec<String>,
    /// Ekspresi WHERE yang sudah divalidasi, bila ada.
    pub filter: Option<Expr>,
    /// Nilai `AS OF` dari SQL inline (jika ada).
    pub as_of: Option<String>,
    /// Nama tabel.
    pub table: String,
    /// Apakah `table` adalah table-valued function (mis. `commits()`).
    pub is_table_function: bool,
}

/// Membuat `ExecutionPlan` dari `SelectStatement` dan header kolom tabel.
///
/// - Memetakan nama kolom `SELECT` ke indeks pada header.
/// - `SELECT *` memilih semua kolom secara berurutan.
/// - AS OF inline dan flag table function dipertahankan.
///
/// # Errors
///
/// Mengembalikan `PlanError` bila:
/// - kolom yang dipilih tidak ada di header,
/// - daftar kolom SELECT kosong (hanya bisa terjadi pada `Named(vec![]`).
pub fn plan_select(stmt: &SelectStatement, headers: &[String]) -> Result<ExecutionPlan, PlanError> {
    let (indices, names) = match &stmt.columns {
        ColumnList::All => {
            let indices: Vec<usize> = (0..headers.len()).collect();
            let names: Vec<String> = headers.to_vec();
            (indices, names)
        }
        ColumnList::Named(cols) => {
            if cols.is_empty() {
                return Err(PlanError::new("daftar kolom SELECT tidak boleh kosong"));
            }
            let mut indices = Vec::with_capacity(cols.len());
            let mut names = Vec::with_capacity(cols.len());
            for col in cols {
                let idx = headers.iter().position(|h| h == col).ok_or_else(|| {
                    PlanError::new(format!(
                        "kolom `{col}` tidak ditemukan di tabel `{}`",
                        stmt.table
                    ))
                })?;
                indices.push(idx);
                names.push(col.clone());
            }
            (indices, names)
        }
    };

    Ok(ExecutionPlan {
        select_indices: indices,
        select_names: names,
        filter: stmt.where_clause.clone(),
        as_of: stmt.as_of.clone(),
        table: stmt.table.clone(),
        is_table_function: stmt.is_table_function,
    })
}

#[cfg(test)]
mod tests;
