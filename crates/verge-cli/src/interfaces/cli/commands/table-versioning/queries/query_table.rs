//! File: `query_table.rs`
//!
//! Deskripsi: Perintah `verge query`.
//! Layer: interfaces/cli/commands/table-versioning/queries
//! Tanggung jawab: Meminta isi tabel pada satu titik waktu lalu menuliskannya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/queries/query_table.rs`
//!   - `infrastructure/{commit,storage}/file-system/...`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use std::io::Write;

use verge_core::application::version_control::use_cases::queries::query_table::{
    query_table, QueryTableInput,
};
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;
use verge_core::infrastructure::commit::file_system::refs::file_tag_pointer::FileTagPointer;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::queries::sql::query_sql::run_sql_query;
use crate::interfaces::cli::commands::table_versioning::{
    flag, object_store, split_args, workspace_layout,
};
use crate::shared::kernel::result::Result;

/// Menjalankan `verge query --table <NAME> --as-of <WHEN>`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — isi tabel pada titik waktu itu ditulis apa adanya ke stdout.
///
/// # Errors
///
/// Mengembalikan pesan kesalahan bila `--table` atau `--as-of` tidak diberikan,
/// nama tabel tidak valid, atau tidak ada commit pada waktu tersebut.
pub fn run_query(args: &[String]) -> Result<()> {
    let (positional, options) = split_args("query", &["--table", "--as-of"], args)?;
    if positional.len() > 1 {
        anyhow::bail!("`query` takes at most one positional argument (SQL query)\n\n{USAGE}");
    }
    let raw_table = flag("query", &options, "--table")?
        .ok_or_else(|| anyhow::anyhow!("`query` requires `--table <NAME>`\n\n{USAGE}"))?;
    let as_of = flag("query", &options, "--as-of")?.ok_or_else(|| {
        anyhow::anyhow!("`query` requires `--as-of <TIMESTAMP|COMMIT|TAG>`\n\n{USAGE}")
    })?;
    let table = TableName::parse(raw_table)?;

    let layout = workspace_layout()?;
    let store: FileBlockStore = object_store(&layout)?;
    let commits = FileCommitRepository::new(store.clone());
    let refs = FileRefPointer::new(layout.clone());
    let tags = FileTagPointer::new(layout);
    if let Some(sql) = positional.first() {
        return run_sql_query(sql, table, as_of, &refs, &tags, &commits, &store);
    }

    let content = query_table(
        &QueryTableInput {
            table,
            as_of: (*as_of).to_owned(),
        },
        &refs,
        &tags,
        &commits,
        &store,
    )?;

    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&content.bytes)?;
    stdout.write_all(b"\n")?;
    Ok(())
}
