//! File: `sql_query.rs`
//!
//! Deskripsi: Use case query SQL pada satu titik waktu.
//! Layer: application/version-control/use-cases/queries
//! Tanggung jawab: Menyelesaikan `--as-of`/AS OF inline, mem-parse SQL,
//!   melakukan planning, membaca tabel (atau commit history untuk
//!   table-valued function), menjalankan query, dan mengembalikan CSV.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/revision_resolver.rs`
//!   - `domain/sql/{parser,executor,planner}/mod.rs`
//!   - `domain/tree/table_reader.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!   - #92 (M5 Part 2: planner dan Verge extensions)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)
//!   - ADR-0011 (Batas 10.000 commit time-travel)
//!   - ADR-0012 (SQL parser semantics and limits)

use crate::application::version_control::revision_resolver::resolve_revision;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::sql::executor::{execute_plan, header_columns};
use crate::domain::sql::parser::parse_sql;
use crate::domain::sql::planner::plan_select;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::table_reader::read_table;
use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::kernel::result::Result;

/// Batas maksimal commit yang ditelusuri oleh `commits()`.
const MAX_COMMIT_SCAN: usize = 10_000;

/// Header virtual untuk table-valued function `commits()`.
const COMMIT_HEADERS: [&str; 6] = [
    "commit_id",
    "parent_ids",
    "table",
    "author",
    "timestamp",
    "message",
];

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
/// Jika SQL mengandung `AS OF <when>` secara inline, nilai itu dipakai
/// sebagai titik waktu dan mengalahkan nilai `--as-of` CLI.
///
/// Args:
/// - input — tabel, titik waktu (CLI), dan statement SQL.
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
/// ([`SqlParse`](VergeError::SqlParse)), planner,
/// atau executor.
pub fn sql_query(
    input: &SqlQueryInput,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<Vec<u8>> {
    let stmt = parse_sql(&input.sql)?;
    let as_of = stmt.as_of.as_deref().unwrap_or(&input.as_of);
    let id = resolve_revision(as_of, refs, tags, commits, Some(&input.table))?;

    let (rows, plan) = if stmt.is_table_function && stmt.table == "commits" {
        let rows = build_commit_rows(&id, commits)?;
        let headers: Vec<String> = COMMIT_HEADERS.iter().map(ToString::to_string).collect();
        let plan = plan_select(&stmt, &headers)?;
        (rows, plan)
    } else {
        let commit = commits.load(&id)?;
        if commit.table() != &input.table {
            return Err(
                crate::shared::exceptions::verge_error::VergeError::CommitBelongsToOtherTable {
                    revision: as_of.to_owned(),
                    commit_table: commit.table().clone(),
                    requested: input.table.clone(),
                },
            );
        }
        let rows = read_table(commit.tree(), store)?;
        let headers = header_columns(rows.header())?;
        let plan = plan_select(&stmt, &headers)?;
        (rows, plan)
    };

    Ok(execute_plan(&rows, &plan)?)
}

/// Menelusuri first-parent dari `start` dan membangun `TableRows` virtual
/// untuk table-valued function `commits()`.
///
/// Kolom: `commit_id`, `parent_ids`, `table`, `author`, `timestamp`, `message`.
/// Traversal terbatas oleh [`MAX_COMMIT_SCAN`] (ADR-0011).
///
/// # Errors
///
/// Mengembalikan error bila commit tidak dapat dimuat.
fn build_commit_rows(start: &CommitId, commits: &dyn CommitRepository) -> Result<TableRows> {
    let mut rows = Vec::new();
    let mut cursor: Option<CommitId> = Some(*start);
    let mut seen = 0;
    while let Some(id) = cursor {
        if seen >= MAX_COMMIT_SCAN {
            break;
        }
        let commit = commits.load(&id)?;
        let parents = commit
            .parents()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(";");
        let value = format!(
            ",{parents},{table},{author},{ts},{msg}",
            parents = parents,
            table = commit.table().as_str(),
            author = commit.author(),
            ts = commit.timestamp_unix_ms(),
            msg = commit.summary(),
        );
        let Some(row) = TableRow::new(
            RowKey::new(commit.id().to_string().into_bytes()),
            value.into_bytes(),
        ) else {
            return Err(
                crate::shared::exceptions::verge_error::VergeError::NothingToCommit(
                    "commit_id kosong".to_string(),
                ),
            );
        };
        rows.push(row);
        seen += 1;
        cursor = commit.parents().first().copied();
    }
    rows.sort_by(|a, b| b.key().cmp(a.key()));
    Ok(TableRows::from_parts(
        COMMIT_HEADERS.join(",").into_bytes(),
        rows,
    ))
}
