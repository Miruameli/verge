//! File: `init_repository.rs`
//!
//! Deskripsi: Perintah `verge init`.
//! Layer: interfaces/cli/commands
//! Tanggung jawab: Memanggil use case bootstrap dan mencetak hasilnya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `verge_core` (use case bootstrap)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use std::path::PathBuf;

use verge_core::application::repository_bootstrap::use_cases::initialize_repository::initialize_repository;
use verge_core::config::repository_layout::RepositoryLayout;
use verge_core::infrastructure::storage::file_system::file_store_factory::FileStoreFactory;
use verge_core::infrastructure::storage::file_system::local_file_system::LocalFileSystem;

use crate::config::cli_usage::USAGE;
use crate::shared::kernel::result::Result;

/// Menjalankan `verge init [PATH]`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — repository berhasil dibuat dan ringkasannya dicetak.
///
/// # Errors
///
/// Mengembalikan pesan kesalahan bila jumlah argumen salah atau bootstrap gagal.
pub fn run_init(args: &[String]) -> Result<()> {
    if args.len() > 1 {
        anyhow::bail!("`init` accepts at most one path\n\n{USAGE}");
    }
    let workspace = args
        .first()
        .map_or_else(|| PathBuf::from("."), PathBuf::from);
    let layout = RepositoryLayout::under(&workspace);
    let repo = initialize_repository(&layout, &FileStoreFactory, &LocalFileSystem)?;

    println!(
        "initialised empty Verge repository in {}",
        layout.root.display()
    );
    println!("objects: {}", repo.store().root().display());
    Ok(())
}
