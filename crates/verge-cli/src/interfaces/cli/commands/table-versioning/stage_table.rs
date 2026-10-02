//! File: `stage_table.rs`
//!
//! Deskripsi: Perintah `verge import`.
//! Layer: interfaces/cli/commands/table-versioning
//! Tanggung jawab: Meminta use case stage lalu mencetak ringkasannya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/stage-table/stage_table.rs`
//!   - `infrastructure/table/file-system/file_{table_source,table_workspace}.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::path::PathBuf;

use verge_core::application::version_control::use_cases::stage_table::{
    stage_table, StageTableInput,
};
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::table::file_system::file_table_source::FileTableSource;
use verge_core::infrastructure::table::file_system::file_table_workspace::FileTableWorkspace;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::{
    flag, object_store, short_id, split_args, workspace_layout,
};
use crate::shared::kernel::result::Result;

/// Menjalankan `verge import <PATH> --table <NAME>`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — data tabel tersimpan sebagai blok dan ringkasannya dicetak.
///
/// # Errors
///
/// Mengembalikan pesan kesalahan bila argumentasi salah, nama tabel tidak
/// valid, folder kerja bukan repository, atau sumber data tidak terbaca.
pub fn run_import(args: &[String]) -> Result<()> {
    let (positional, options) = split_args("import", &["--table"], args)?;
    let raw_table = flag("import", &options, "--table")?
        .ok_or_else(|| anyhow::anyhow!("`import` requires `--table <NAME>`\n\n{USAGE}"))?;
    let table = TableName::parse(raw_table)?;
    let Some(source) = positional.first() else {
        anyhow::bail!("`import` requires a data file path\n\n{USAGE}");
    };

    let layout = workspace_layout()?;
    let store = object_store(&layout)?;
    let workspace = FileTableWorkspace::new(layout, store);
    let staged = stage_table(
        &StageTableInput {
            table,
            source: PathBuf::from(source),
        },
        &FileTableSource,
        &workspace,
    )?;

    println!(
        "staged table `{}` ({} bytes) as block {}",
        staged.table,
        staged.bytes,
        short_id(&staged.block)
    );
    Ok(())
}
