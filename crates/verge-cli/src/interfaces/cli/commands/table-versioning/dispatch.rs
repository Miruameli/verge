//! File: `dispatch.rs`
//!
//! Deskripsi: Router perintah versioning tabel.
//! Layer: interfaces/cli/commands/table-versioning
//! Tanggung jawab: Memetakan nama perintah `import`, `commit`, `log`, `show`,
//!   `diff`, dan `query` ke modul yang mengerjakan. Dipisah dari
//!   `cli_dispatcher.rs` supaya penambahan perintah tabel tidak menambah import
//!   pada dispatcher utama.
//!
//! Author: Miruameli
//! Created: 2026-10-04
//! Modified: 2026-10-04
//! Version: 0.3.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `queries/`, `record_commit.rs`, `stage_table.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::queries::diff_tables::run_diff;
use crate::interfaces::cli::commands::table_versioning::queries::query_table::run_query;
use crate::interfaces::cli::commands::table_versioning::queries::read_history::run_log;
use crate::interfaces::cli::commands::table_versioning::queries::read_snapshot::run_show;
use crate::interfaces::cli::commands::table_versioning::record_commit::run_commit;
use crate::interfaces::cli::commands::table_versioning::stage_table::run_import;
use crate::shared::kernel::result::Result;

/// KONTEKS: daftar perintah tabel ditulis sekali agar pesan error menyebut nama
/// yang sama dengan pesan bantuan; KENAPA: dua daftar yang berbeda membuat
/// pengguna mencari perintah yang tidak pernah ada.
const TABLE_COMMANDS: [&str; 6] = ["import", "commit", "log", "show", "diff", "query"];

/// Menentukan apakah `name` adalah perintah versioning tabel.
///
/// Args:
/// - name — nama perintah dari argv.
///
/// Returns:
/// - true — perintah ditangani [`run_table_command`].
/// - false — perintah milik kelompok lain.
///
/// # Errors
///
/// Tidak ada; fungsi ini tidak menyentuh I/O.
#[must_use]
pub fn is_table_command(name: &str) -> bool {
    TABLE_COMMANDS.contains(&name)
}

/// Menjalankan satu perintah versioning tabel.
///
/// Args:
/// - args — argumen setelah nama perintah tabel.
///
/// Returns:
/// - Ok(()) — perintah tabel selesai tanpa error.
///
/// # Errors
///
/// Mengembalikan pesan yang menyebut daftar perintah tabel bila nama perintah
/// tidak dikenal, serta error dari modul perintah yang dipanggil.
pub fn run_table_command(args: &[String]) -> Result<()> {
    let Some(command) = args.first().map(String::as_str) else {
        anyhow::bail!("missing table command\n\n{USAGE}");
    };
    let rest = &args[1..];
    match command {
        "import" => run_import(rest),
        "commit" => run_commit(rest),
        "log" => run_log(rest),
        "show" => run_show(rest),
        "diff" => run_diff(rest),
        "query" => run_query(rest),
        unknown => anyhow::bail!(
            "unknown table command `{unknown}`; expected one of {}\n\n{USAGE}",
            TABLE_COMMANDS.join(", ")
        ),
    }
}
