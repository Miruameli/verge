//! File: `diff_tables.rs`
//!
//! Deskripsi: Perintah `verge diff`.
//! Layer: interfaces/cli/commands/table-versioning/queries
//! Tanggung jawab: Meminta perubahan baris antara dua revisi lalu mencetaknya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/diff_tables.rs`
//!   - `infrastructure/commit/file-system/file_{commit_repository,ref_pointer}.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use verge_core::application::version_control::dtos::table_diff_report::TableDiffReport;
use verge_core::application::version_control::use_cases::diff_tables::{
    diff_tables, DiffTablesInput,
};
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::domain::tree::diff::row_change::RowChange;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::commit::file_system::file_ref_pointer::FileRefPointer;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::{
    flag, object_store, split_args, workspace_layout,
};
use crate::shared::kernel::result::Result;

/// Teks yang dicetak saat kedua revisi memuat tabel yang identik.
const NO_CHANGES: &str = "no changes";

/// Pemisah revisi dalam satu argumen posisional.
const RANGE_SEPARATOR: &str = "..";

/// Menjalankan `verge diff <FROM>..<TO> --table <NAME>`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — satu baris per perubahan baris, atau `no changes` bila identik.
///
/// # Errors
///
/// Mengembalikan pesan kesalahan bila rentang tidak memuat `..`, `--table`
/// tidak diberikan, nama tabel tidak valid, atau salah satu revisi tidak
/// menunjuk tabel tersebut.
pub fn run_diff(args: &[String]) -> Result<()> {
    let (positional, options) = split_args("diff", &["--table"], args)?;
    let raw_table = flag("diff", &options, "--table")?
        .ok_or_else(|| anyhow::anyhow!("`diff` requires `--table <NAME>`\n\n{USAGE}"))?;
    let table = TableName::parse(raw_table)?;
    let Some(range) = positional.first() else {
        anyhow::bail!("`diff` requires `<FROM>..<TO>`\n\n{USAGE}");
    };
    let (from, to) = split_range(range)?;

    let layout = workspace_layout()?;
    let store: FileBlockStore = object_store(&layout)?;
    let commits = FileCommitRepository::new(store.clone());
    let refs = FileRefPointer::new(layout);
    let report = diff_tables(
        &DiffTablesInput { table, from, to },
        &refs,
        &commits,
        &store,
    )?;

    print_report(&report);
    Ok(())
}

/// Memecah `<FROM>..<TO>` tepat pada pemisah pertama.
///
/// # Errors
///
/// Mengembalikan pesan yang menyebut bentuk rentang bila `..` tidak ada atau
/// salah satu sisinya kosong.
fn split_range(range: &str) -> Result<(String, String)> {
    let hint = format!("expected `<FROM>..<TO>`, got `{range}`");
    let Some((from, to)) = range.split_once(RANGE_SEPARATOR) else {
        anyhow::bail!("`diff` requires `{RANGE_SEPARATOR}` between revisions: {hint}\n\n{USAGE}");
    };
    if from.is_empty() || to.is_empty() {
        anyhow::bail!("`diff` requires a revision on both sides of `..`: {hint}\n\n{USAGE}");
    }
    Ok((from.to_owned(), to.to_owned()))
}

/// Mencetak seluruh perubahan; tabel identik hanya mencetak satu baris ringkas.
fn print_report(report: &TableDiffReport) {
    if report.is_empty() {
        println!("{NO_CHANGES}");
        return;
    }
    for change in &report.changes {
        println!("{}", render(change));
    }
}

/// Merender satu perubahan menjadi satu baris keluaran.
///
/// Nilai baris dicetak apa adanya, termasuk pemisah kolom pertamanya, sehingga
/// baris keluaran dapat dibaca kembali sebagai baris tabel.
fn render(change: &RowChange) -> String {
    match change {
        RowChange::Added { key, value } => format!("+ {key} {}", text(value)),
        RowChange::Removed { key, value } => format!("- {key} {}", text(value)),
        RowChange::Modified { key, before, after } => {
            format!("~ {key} {} -> {}", text(before), text(after))
        }
    }
}

/// Mengubah byte nilai baris menjadi teks yang aman dicetak.
fn text(value: &[u8]) -> String {
    String::from_utf8_lossy(value).into_owned()
}
