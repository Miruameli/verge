//! File: `query_sql.rs`
//!
//! Deskripsi: Backend eksekusi query SQL pada CLI.
//! Layer: interfaces/cli/commands/table-versioning/queries/sql
//! Tanggung jawab: Menerima string SQL + port instances, memanggil use case
//! `sql_query`, mencetak CSV hasil ke stdout.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/queries/sql_query.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use std::io::Write;

use verge_core::application::version_control::use_cases::queries::sql_query::{
    sql_query, SqlQueryInput,
};
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::{CommitRepository, RefPointer, Store, TagPointer};

use crate::shared::kernel::result::Result;

/// Menjalankan `verge query --table <NAME> --as-of <WHEN> "<SQL>"`.
///
/// Args:
/// - sql — string SQL yang akan diparse dan dieksekusi.
/// - table — nama tabel yang diqueri.
/// - `as_of` — titik waktu resolusi (timestamp, unix ms, tag, atau commit id).
/// - refs — port pointer branch.
/// - tags — port pointer tag.
/// - commits — port penyimpanan objek commit.
/// - store — port object store.
///
/// Returns:
/// - Ok(()) — CSV hasil query ditulis ke stdout.
///
/// # Errors
///
/// Melemparkan error dari resolver revision, parser SQL, atau executor.
pub fn run_sql_query(
    sql: &str,
    table: TableName,
    as_of: &str,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<()> {
    let csv = sql_query(
        &SqlQueryInput {
            table,
            as_of: as_of.to_owned(),
            sql: sql.to_owned(),
        },
        refs,
        tags,
        commits,
        store,
    )?;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&csv)?;
    stdout.write_all(b"\n")?;
    Ok(())
}
