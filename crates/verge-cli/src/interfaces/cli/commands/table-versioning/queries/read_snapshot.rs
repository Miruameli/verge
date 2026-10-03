//! File: `read_snapshot.rs`
//!
//! Deskripsi: Perintah `verge show`.
//! Layer: interfaces/cli/commands/table-versioning/queries
//! Tanggung jawab: Meminta isi tabel pada sebuah revisi lalu menuliskannya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/read-snapshot/read_snapshot.rs`
//!   - `infrastructure/{commit,storage}/file-system/...`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::io::Write;

use verge_core::application::version_control::use_cases::read_snapshot::{
    read_snapshot, ReadSnapshotInput,
};
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;
use verge_core::infrastructure::commit::file_system::refs::file_tag_pointer::FileTagPointer;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::{
    flag, object_store, split_args, workspace_layout,
};
use crate::shared::kernel::result::Result;

/// Menjalankan `verge show <REVISION> --table <NAME>`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — isi tabel pada revisi itu ditulis apa adanya ke stdout.
///
/// # Errors
///
/// Mengembalikan pesan kesalahan bila revisi atau `--table` tidak diberikan,
/// nama tabel tidak valid, atau revisi tidak menunjuk tabel tersebut.
pub fn run_show(args: &[String]) -> Result<()> {
    let (positional, options) = split_args("show", &["--table"], args)?;
    let revision = positional
        .first()
        .ok_or_else(|| anyhow::anyhow!("`show` requires a revision\n\n{USAGE}"))?;
    let raw_table = flag("show", &options, "--table")?
        .ok_or_else(|| anyhow::anyhow!("`show` requires `--table <NAME>`\n\n{USAGE}"))?;
    let table = TableName::parse(raw_table)?;

    let layout = workspace_layout()?;
    let store: FileBlockStore = object_store(&layout)?;
    let commits = FileCommitRepository::new(store.clone());
    let refs = FileRefPointer::new(layout.clone());
    let tags = FileTagPointer::new(layout);
    let snapshot = read_snapshot(
        &ReadSnapshotInput {
            table,
            revision: (*revision).to_owned(),
        },
        &refs,
        &tags,
        &commits,
        &store,
    )?;

    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&snapshot.bytes)?;
    stdout.write_all(b"\n")?;
    Ok(())
}
