//! File: `manage_tags.rs`
//!
//! Deskripsi: Perintah `verge tag`.
//! Layer: interfaces/cli/commands/tagging
//! Tanggung jawab: Menerjemahkan subperintah tag menjadi use case.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use-cases/refs/tagging/tag_command.rs`
//!   - `infrastructure/{commit,storage}/file-system/...`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use verge_core::application::version_control::use_cases::refs::tagging::tag_command::{
    create_tag, delete_tag, list_tags,
};
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;
use verge_core::infrastructure::commit::file_system::refs::file_tag_pointer::FileTagPointer;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;
use verge_core::TagPointer;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::{
    flag, object_store, short_id, split_args, workspace_layout,
};
use crate::shared::kernel::result::Result;

/// Menjalankan `verge tag <SUBCOMMAND>`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — subperintah selesai dan hasilnya dicetak ke stdout.
///
/// # Errors
///
/// Mengembalikan pesan kesalahan untuk subperintah atau flag yang tidak dikenal.
pub fn run_tag(args: &[String]) -> Result<()> {
    let Some((subcommand, rest)) = args.split_first() else {
        anyhow::bail!("`tag` requires a subcommand\n\n{USAGE}");
    };
    match subcommand.as_str() {
        "create" => run_create(rest),
        "list" => run_list(),
        "delete" => run_delete(rest),
        unknown => anyhow::bail!("unknown tag subcommand `{unknown}`\n\n{USAGE}"),
    }
}

/// Menjalankan `verge tag create <NAME> [--revision <REV>]`.
fn run_create(args: &[String]) -> Result<()> {
    let (positional, options) = split_args("tag create", &["--revision"], args)?;
    let name = positional
        .first()
        .ok_or_else(|| anyhow::anyhow!("`tag create` requires a name\n\n{USAGE}"))?;
    let revision = flag("tag create", &options, "--revision")?.unwrap_or("HEAD");

    let layout = workspace_layout()?;
    let store: FileBlockStore = object_store(&layout)?;
    let commits = FileCommitRepository::new(store.clone());
    let refs = FileRefPointer::new(layout.clone());
    let tags = FileTagPointer::new(layout);
    create_tag(name, revision, &refs, &tags, &commits)?;

    println!("tagged {name} at {revision}");
    Ok(())
}

/// Menjalankan `verge tag list`.
///
/// Kolom tabel dicetak karena tag tidak pernah menyimpan nama tabelnya sendiri:
/// tanpa kolom itu pengguna harus menebak tabel mana yang berlaku untuk tag.
fn run_list() -> Result<()> {
    let layout = workspace_layout()?;
    let store: FileBlockStore = object_store(&layout)?;
    let commits = FileCommitRepository::new(store.clone());
    let tags = FileTagPointer::new(layout);
    let names = tags.tags()?;
    let list = list_tags(&names, &tags, &commits)?;

    for entry in &list.entries {
        println!(
            "{} {} {} {}",
            entry.name,
            short_id(&entry.commit),
            entry.table,
            entry.summary
        );
    }
    Ok(())
}

/// Menjalankan `verge tag delete <NAME>`.
fn run_delete(args: &[String]) -> Result<()> {
    let (positional, _) = split_args("tag delete", &[], args)?;
    let name = positional
        .first()
        .ok_or_else(|| anyhow::anyhow!("`tag delete` requires a name\n\n{USAGE}"))?;

    let layout = workspace_layout()?;
    let tags = FileTagPointer::new(layout);
    delete_tag(name, &tags)?;

    println!("deleted tag {name}");
    Ok(())
}
