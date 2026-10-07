//! File: `sql_query.rs`
//!
//! Deskripsi: Use case query SQL pada satu titik waktu.
//! Layer: application/version-control/use-cases/queries
//! Tanggung jawab: Menyelesaikan `--as-of`, membaca tabel, mem-parse SQL,
//! menjalankan query, dan mengembalikan CSV hasil proyeksi.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/revision_resolver.rs`
//!   - `domain/sql/parser/mod.rs`, `domain/sql/executor.rs`
//!   - `domain/tree/table_reader.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)
//!   - ADR-0012 (SQL parser semantics and limits)

use crate::application::version_control::revision_resolver::resolve_revision;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::sql::executor::execute_query;
use crate::domain::sql::parser::parse_sql;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::table_reader::read_table;
use crate::shared::kernel::result::Result;

/// Masukan query SQL pada satu titik waktu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlQueryInput {
    /// Tabel yang diqueri.
    pub table: TableName,
    /// Titik waktu: RFC 3339 UTC, `@<unix_ms>`, nama tag, atau commit id.
    pub as_of: String,
    /// Statement SQL (mis. `SELECT id, name FROM ... WHERE age > 18`).
    pub sql: String,
}

/// Menjalankan query SQL pada tabel pada commit yang ditunjuk `as_of`.
///
/// Args:
/// - input — tabel, titik waktu, dan statement SQL.
/// - refs — port pointer branch.
/// - tags — port pointer tag.
/// - commits — port penyimpanan objek commit.
/// - store — port object store.
///
/// Returns:
/// - Ok(Vec<u8>) — CSV hasil query (header + baris yang lolos filter).
///
/// # Errors
///
/// Mengembalikan error dari [`resolve_revision`], parser SQL
/// ([`SqlParse`](VergeError::SqlParse)), atau executor
/// ([`SqlParse`](VergeError::SqlParse)).
pub fn sql_query(
    input: &SqlQueryInput,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<Vec<u8>> {
    let id = resolve_revision(&input.as_of, refs, tags, commits, Some(&input.table))?;
    let commit = commits.load(&id)?;
    if commit.table() != &input.table {
        return Err(
            crate::shared::exceptions::verge_error::VergeError::CommitBelongsToOtherTable {
                revision: input.as_of.clone(),
                commit_table: commit.table().clone(),
                requested: input.table.clone(),
            },
        );
    }
    let rows = read_table(commit.tree(), store)?;
    let stmt = parse_sql(&input.sql)?;
    Ok(execute_query(&rows, &stmt)?)
}
