//! File: `stage_table.rs`
//!
//! Deskripsi: Use case penyimpanan data kerja tabel.
//! Layer: application/version-control/use-cases/stage-table
//! Tanggung jawab: Membaca sumber data lalu menyimpannya sebagai blok.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/dtos/staged_table.rs`
//!   - `domain/table/ports/table_source.rs`, `table_workspace.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::path::PathBuf;

use crate::application::version_control::dtos::staged_table::StagedTable;
use crate::domain::table::ports::table_source::{TableSource, MAX_TABLE_BYTES};
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan penyimpanan data kerja tabel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageTableInput {
    /// Tabel tujuan.
    pub table: TableName,
    /// Sumber data yang dibaca.
    pub source: PathBuf,
}

/// Menyimpan isi tabel ke data kerja sebagai blok content-addressed.
///
/// Args:
/// - input — tabel tujuan dan sumber datanya.
/// - source — port pembacaan data.
/// - workspace — port penyimpanan data kerja.
///
/// Returns:
/// - Ok(StagedTable) — blok data kerja beserta ukuran datanya.
///
/// # Errors
///
/// Mengembalikan error I/O bila sumber tidak dapat dibaca atau blok tidak dapat
/// ditulis, serta [`InvalidCommitField`](VergeError::InvalidCommitField) bila
/// data melebihi [`MAX_TABLE_BYTES`] sehingga proses kehabisan memori.
///
/// Example:
/// ```no_run
/// use std::path::PathBuf;
///
/// use verge_core::application::version_control::use_cases::stage_table::{
///     stage_table, StageTableInput,
/// };
/// use verge_core::config::repository_layout::RepositoryLayout;
/// use verge_core::domain::table::value_objects::table_name::TableName;
/// use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;
/// use verge_core::infrastructure::table::file_system::file_table_source::FileTableSource;
/// use verge_core::infrastructure::table::file_system::file_table_workspace::FileTableWorkspace;
///
/// let layout = RepositoryLayout::under("/tmp/verge-doc");
/// let store = FileBlockStore::open(layout.objects()).expect("open store");
/// let workspace = FileTableWorkspace::new(layout, store);
/// let input = StageTableInput {
///     table: TableName::parse("users").expect("valid table"),
///     source: PathBuf::from("users.csv"),
/// };
/// let staged = stage_table(&input, &FileTableSource, &workspace).expect("stage table");
/// assert!(staged.bytes > 0);
/// ```
pub fn stage_table(
    input: &StageTableInput,
    source: &dyn TableSource,
    workspace: &dyn TableWorkspace,
) -> Result<StagedTable> {
    let data = source.read_all(&input.source)?;
    if data.len() > MAX_TABLE_BYTES {
        return Err(VergeError::InvalidCommitField {
            field: "table data",
            detail: "exceeds the maximum table size",
        });
    }
    let block = workspace.stage(&input.table, &data)?;
    Ok(StagedTable {
        table: input.table.clone(),
        block,
        bytes: data.len(),
    })
}
