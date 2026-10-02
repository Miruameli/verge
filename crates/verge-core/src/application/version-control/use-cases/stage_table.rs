//! File: `stage_table.rs`
//!
//! Deskripsi: Use case penyimpanan data kerja tabel.
//! Layer: application/version-control/use-cases/staging
//! Tanggung jawab: Membaca sumber data lalu menyimpannya sebagai prolly tree.
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
//!   - `domain/storage/ports/block_store.rs`
//!   - `domain/tree/table_codec.rs`, `tree_builder.rs`, `nodes/tree_node_codec.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use std::path::PathBuf;

use crate::application::version_control::dtos::staged_table::StagedTable;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::ports::table_source::{TableSource, MAX_TABLE_BYTES};
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::nodes::tree_node_codec::encode;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::tree_builder::build_plan;
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

/// Menyimpan isi tabel ke data kerja sebagai prolly tree.
///
/// KONTEKS: node ditulis ke `store` bukan ke `workspace` karena port data kerja
/// hanya memegang pointer; KENAPA: baris yang tidak berubah menghasilkan node
/// dengan digest sama sehingga `Store::put` mendedupinya tanpa kerja tambahan.
///
/// Args:
/// - input — tabel tujuan dan sumber datanya.
/// - source — port pembacaan data.
/// - workspace — port penyimpanan pointer data kerja.
/// - store — port object store blok.
///
/// Returns:
/// - Ok(StagedTable) — akar tree data kerja beserta ukuran datanya.
///
/// # Errors
///
/// Mengembalikan [`InvalidCommitField`](VergeError::InvalidCommitField) bila
/// data melebihi [`MAX_TABLE_BYTES`] sehingga proses kehabisan memori,
/// [`MalformedTable`](VergeError::MalformedTable) atau
/// [`EmptyTable`](VergeError::EmptyTable) bila isi tabel tidak dapat dijadikan
/// tree, serta error I/O dari port.
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
/// let workspace = FileTableWorkspace::new(layout);
/// let input = StageTableInput {
///     table: TableName::parse("users").expect("valid table"),
///     source: PathBuf::from("users.csv"),
/// };
/// let staged = stage_table(&input, &FileTableSource, &workspace, &store).expect("stage table");
/// assert!(staged.bytes > 0);
/// ```
pub fn stage_table(
    input: &StageTableInput,
    source: &dyn TableSource,
    workspace: &dyn TableWorkspace,
    store: &dyn Store,
) -> Result<StagedTable> {
    let data = source.read_all(&input.source)?;
    if data.len() > MAX_TABLE_BYTES {
        return Err(VergeError::InvalidCommitField {
            field: "table data",
            detail: "exceeds the maximum table size",
        });
    }
    let rows = TableRows::parse(&data)?;
    let plan = build_plan(&rows)?;
    let mut bytes = 0;
    for node in &plan.nodes {
        let encoded = encode(node);
        store.put(&encoded)?;
        bytes += encoded.len();
    }
    workspace.stage(&input.table, plan.root)?;
    Ok(StagedTable {
        table: input.table.clone(),
        root: plan.root,
        bytes,
        rows: rows.len(),
    })
}
