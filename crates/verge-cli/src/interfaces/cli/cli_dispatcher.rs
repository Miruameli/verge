//! File: `cli_dispatcher.rs`
//!
//! Deskripsi: Dispatcher perintah CLI.
//! Layer: interfaces/cli
//! Tanggung jawab: Memetakan argv ke perintah dan mencetak bantuan/version.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `config/cli_usage.rs`
//!   - `interfaces/cli/commands/init_repository.rs`
//!   - `interfaces/cli/commands/table-versioning/**`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::config::cli_usage::{USAGE, VERSION};
use crate::interfaces::cli::commands::branches::manage_branches::run_branch;
use crate::interfaces::cli::commands::init_repository::run_init;
use crate::interfaces::cli::commands::merging::merge_tables::run_merge;
use crate::interfaces::cli::commands::table_versioning::queries::diff_tables::run_diff;
use crate::interfaces::cli::commands::table_versioning::queries::read_history::run_log;
use crate::interfaces::cli::commands::table_versioning::queries::read_snapshot::run_show;
use crate::interfaces::cli::commands::table_versioning::record_commit::run_commit;
use crate::interfaces::cli::commands::table_versioning::stage_table::run_import;
use crate::shared::kernel::result::Result;

/// Menjalankan satu perintah CLI.
///
/// Args:
/// - args — argumen tanpa nama program.
///
/// Returns:
/// - Ok(()) — perintah selesai tanpa error.
///
/// # Errors
///
/// Mengembalikan pesan yang siap ditampilkan ke pengguna untuk perintah tak
/// dikenal atau argumentasi yang salah.
pub fn dispatch(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("init") => run_init(&args[1..]),
        Some("import") => run_import(&args[1..]),
        Some("commit") => run_commit(&args[1..]),
        Some("log") => run_log(&args[1..]),
        Some("show") => run_show(&args[1..]),
        Some("diff") => run_diff(&args[1..]),
        Some("branch") => run_branch(&args[1..]),
        Some("merge") => run_merge(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            print!("{USAGE}");
            Ok(())
        }
        Some("--version" | "-V") => {
            println!("verge {VERSION}");
            Ok(())
        }
        Some(unknown) => anyhow::bail!("unknown command `{unknown}`\n\n{USAGE}"),
    }
}
