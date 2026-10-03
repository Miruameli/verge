//! File: `merge_tables.rs`
//!
//! Deskripsi: Perintah `verge merge`.
//! Layer: interfaces/cli/commands/merging
//! Tanggung jawab: Meminta merge branch lalu menyerahkan hasilnya ke pencetak laporan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/merging/merge_branch.rs`
//!   - `infrastructure/{commit,storage}/file-system/**`
//!   - `merge_report_printer.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use verge_core::application::version_control::use_cases::merging::merge_branch::{
    merge_branch, MergeBranchInput,
};
use verge_core::domain::merge::merge_strategy::MergeStrategy;
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;
use verge_core::infrastructure::system::system_clock::now_unix_ms;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::merging::merge_report_printer::print_report;
use crate::interfaces::cli::commands::table_versioning::{
    flag, object_store, split_args, workspace_layout,
};
use crate::shared::kernel::result::Result;

/// Environment variable yang menjadi cadangan untuk author.
const AUTHOR_ENV: &str = "VERGE_AUTHOR";

/// Flag yang diterima perintah ini.
const FLAGS: [&str; 4] = ["--table", "--strategy", "--message", "--author"];

/// Menjalankan `verge merge <BRANCH> --table <NAME> [--strategy <NAME>]`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — ringkasan merge dicetak; konflik membuat perintah keluar dengan
///   kode bukan nol agar pemanggil tahu merge belum terjadi.
///
/// # Errors
///
/// Mengembalikan pesan yang menyebut bentuk perintah bila branch, `--table`,
/// atau `--strategy` tidak diberikan atau tidak dikenal.
pub fn run_merge(args: &[String]) -> Result<()> {
    let (positional, options) = split_args("merge", &FLAGS, args)?;
    let raw_table = flag("merge", &options, "--table")?
        .ok_or_else(|| anyhow::anyhow!("`merge` requires `--table <NAME>`\n\n{USAGE}"))?;
    let table = TableName::parse(raw_table)?;
    let strategy = match flag("merge", &options, "--strategy")? {
        Some(name) => MergeStrategy::parse(name).ok_or_else(|| {
            anyhow::anyhow!(
                "unknown merge strategy `{name}`; expected one of {}\n\n{USAGE}",
                MergeStrategy::names()
            )
        })?,
        None => MergeStrategy::Manual,
    };
    let Some(source) = positional.first() else {
        anyhow::bail!("`merge` requires a source branch\n\n{USAGE}");
    };

    let layout = workspace_layout()?;
    let store: FileBlockStore = object_store(&layout)?;
    let commits = FileCommitRepository::new(store.clone());
    let refs = FileRefPointer::new(layout);
    let message = flag("merge", &options, "--message")?
        .map_or_else(|| format!("Merge branch `{source}`"), ToOwned::to_owned);
    let author = resolve_author(flag("merge", &options, "--author")?)?;
    let report = merge_branch(
        &MergeBranchInput {
            source: (*source).to_owned(),
            table,
            strategy,
            message,
            author,
            timestamp_unix_ms: now_unix_ms(),
        },
        &refs,
        &commits,
        &store,
    )?;

    print_report(&report);
    if report.has_conflicts() {
        std::process::exit(1);
    }
    Ok(())
}

/// Memilih author dari flag `--author`, lalu environment variable `VERGE_AUTHOR`.
///
/// # Errors
///
/// Mengembalikan pesan yang menyebut kedua sumber bila keduanya tidak diisi;
/// ALTERNATIF: memakai nama cadangan membuat commit terlihat berasal dari orang
/// yang tidak pernah menyetujui perubahan itu.
fn resolve_author(from_flag: Option<&str>) -> Result<String> {
    if let Some(author) = from_flag {
        return Ok(author.to_owned());
    }
    if let Ok(author) = std::env::var(AUTHOR_ENV) {
        if !author.is_empty() {
            return Ok(author);
        }
    }
    anyhow::bail!("`merge` requires `--author <NAME>` or $VERGE_AUTHOR")
}
