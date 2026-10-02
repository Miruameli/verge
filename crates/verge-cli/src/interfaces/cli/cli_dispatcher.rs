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
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use crate::config::cli_usage::{USAGE, VERSION};
use crate::interfaces::cli::commands::init_repository::run_init;
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
