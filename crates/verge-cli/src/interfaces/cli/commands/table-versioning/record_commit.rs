//! File: `record_commit.rs`
//!
//! Deskripsi: Perintah `verge commit`.
//! Layer: interfaces/cli/commands/table-versioning
//! Tanggung jawab: Meminta use case commit lalu mencetak id dan branch.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/record-commit/record_commit.rs`
//!   - `infrastructure/{commit,table,system}/...`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use verge_core::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::commit::file_system::file_ref_pointer::FileRefPointer;
use verge_core::infrastructure::system::system_clock::now_unix_ms;
use verge_core::infrastructure::table::file_system::file_table_workspace::FileTableWorkspace;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::{
    flag, object_store, short_id, split_args, workspace_layout,
};
use crate::shared::kernel::result::Result;

/// Nama environment variable yang menyimpan default author.
const AUTHOR_ENV: &str = "VERGE_AUTHOR";

/// Menjalankan `verge commit --table <NAME> --message <MSG> [--author <NAME>]`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — commit tercatat dan id singkat serta branch dicetak.
///
/// # Errors
///
/// Mengembalikan pesan kesalahan bila argumentasi salah, author tidak
/// ditemukan, atau use case menolak commit tersebut.
pub fn run_commit(args: &[String]) -> Result<()> {
    let (_positional, options) = split_args("commit", &["--table", "--message", "--author"], args)?;
    let raw_table = flag("commit", &options, "--table")?
        .ok_or_else(|| anyhow::anyhow!("`commit` requires `--table <NAME>`\n\n{USAGE}"))?;
    let message = flag("commit", &options, "--message")?
        .ok_or_else(|| anyhow::anyhow!("`commit` requires `--message <MSG>`\n\n{USAGE}"))?;
    let table = TableName::parse(raw_table)?;
    let author = resolve_author(flag("commit", &options, "--author")?)?;

    let layout = workspace_layout()?;
    let store = object_store(&layout)?;
    let workspace = FileTableWorkspace::new(layout.clone());
    let commits = FileCommitRepository::new(store);
    let refs = FileRefPointer::new(layout);
    let recorded = record_commit(
        &RecordCommitInput {
            table,
            message: message.to_owned(),
            author,
        },
        &workspace,
        &commits,
        &refs,
        now_unix_ms(),
    )?;

    println!(
        "[{}] {} on {}",
        short_id(&recorded.id),
        if recorded.created {
            "created"
        } else {
            "reused"
        },
        recorded.branch
    );
    Ok(())
}

/// Memilih author dari flag `--author`, lalu environment variable `VERGE_AUTHOR`.
///
/// # Errors
///
/// Mengembalikan pesan yang menyebut kedua sumber bila keduanya tidak diisi.
fn resolve_author(from_flag: Option<&str>) -> Result<String> {
    if let Some(author) = from_flag {
        return Ok(author.to_owned());
    }
    if let Ok(author) = std::env::var(AUTHOR_ENV) {
        if !author.is_empty() {
            return Ok(author);
        }
    }
    anyhow::bail!(
        "commit author is required: pass `--author <NAME>` or set {AUTHOR_ENV}\n\n{USAGE}"
    )
}
