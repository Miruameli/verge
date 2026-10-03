//! File: `read_history.rs`
//!
//! Deskripsi: Perintah `verge log`.
//! Layer: interfaces/cli/commands/table-versioning/queries
//! Tanggung jawab: Meminta riwayat commit lalu mencetaknya satu baris per entri.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/read-history/read_history.rs`
//!   - `infrastructure/commit/file-system/file_{commit_repository,ref_pointer}.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use verge_core::application::version_control::use_cases::read_history::{
    read_history, ReadHistoryInput,
};
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::domain::time::value_objects::timestamp::Timestamp;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::{
    flag, object_store, short_id, split_args, workspace_layout,
};
use crate::shared::kernel::result::Result;

/// Jumlah entri yang dicetak bila `--limit` tidak diberikan.
const DEFAULT_LIMIT: usize = 20;

/// Tabel yang dipakai bila `--table` tidak diberikan.
const DEFAULT_TABLE: &str = "main";

/// Menjalankan `verge log [--table <NAME>] [--limit <N>]`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — riwayat tercetak, satu baris per entri.
///
/// # Errors
///
/// Mengembalikan pesan kesalahan bila `--limit` bukan angka positif, nama
/// tabel tidak valid, atau riwayat tidak dapat dibaca.
pub fn run_log(args: &[String]) -> Result<()> {
    let (_positional, options) = split_args("log", &["--table", "--limit"], args)?;
    let table = TableName::parse(flag("log", &options, "--table")?.unwrap_or(DEFAULT_TABLE))?;
    let limit = match flag("log", &options, "--limit")? {
        Some(raw) => parse_limit(raw)?,
        None => DEFAULT_LIMIT,
    };

    let layout = workspace_layout()?;
    let commits = FileCommitRepository::new(object_store(&layout)?);
    let refs = FileRefPointer::new(layout);
    let entries = read_history(&ReadHistoryInput { table, limit }, &refs, &commits)?;

    for entry in entries {
        println!(
            "{} {} {} {}",
            short_id(&entry.id),
            Timestamp::from_unix_ms(entry.timestamp_unix_ms).to_rfc3339(),
            entry.author,
            entry.summary
        );
    }
    Ok(())
}

/// Mengubah teks `--limit` menjadi jumlah entri.
///
/// # Errors
///
/// Mengembalikan pesan ke pengguna bila `raw` bukan angka atau bernilai 0.
fn parse_limit(raw: &str) -> Result<usize> {
    match raw.parse::<usize>() {
        Ok(limit) if limit >= 1 => Ok(limit),
        _ => anyhow::bail!("`--limit` must be a positive number, got `{raw}`\n\n{USAGE}"),
    }
}
