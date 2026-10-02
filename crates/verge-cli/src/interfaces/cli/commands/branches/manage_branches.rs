//! File: `manage_branches.rs`
//!
//! Deskripsi: Perintah `verge branch`.
//! Layer: interfaces/cli/commands/branches
//! Tanggung jawab: Memetakan subperintah branch ke use case lalu mencetak hasil.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/use_cases/branching/**`
//!   - `infrastructure/commit/file-system/file_ref_pointer.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
//!   - ADR-0003 (Arsitektur 7-layer)

use verge_core::application::version_control::dtos::branch_list::BranchList;
use verge_core::application::version_control::use_cases::branching::create_branch::{
    create_branch, CreateBranchInput,
};
use verge_core::application::version_control::use_cases::branching::delete_branch::{
    delete_branch, DeleteBranchInput,
};
use verge_core::application::version_control::use_cases::branching::list_branches::list_branches;
use verge_core::application::version_control::use_cases::branching::switch_branch::{
    switch_branch, SwitchBranchInput,
};
use verge_core::domain::ident::value_objects::digest_text::HexText;
use verge_core::infrastructure::commit::file_system::file_ref_pointer::FileRefPointer;

use crate::config::cli_usage::USAGE;
use crate::interfaces::cli::commands::table_versioning::workspace_layout;
use crate::shared::kernel::result::Result;

/// Jumlah awalan hex yang dicetak pada daftar branch.
const HEAD_PREFIX_LEN: usize = 12;

/// Menjalankan `verge branch <list|create|switch|delete> [NAME]`.
///
/// Args:
/// - args — argumen setelah nama perintah.
///
/// Returns:
/// - Ok(()) — daftar branch dicetak atau perubahan branch dilaporkan.
///
/// # Errors
///
/// Mengembalikan pesan yang menyebut bentuk perintah bila subperintah atau nama
/// branch tidak diberikan, serta error dari use case bila nama tidak aman,
/// branch sudah ada, branch belum ada, atau branch aktif tidak boleh dihapus.
pub fn run_branch(args: &[String]) -> Result<()> {
    let Some(action) = args.first().map(String::as_str) else {
        anyhow::bail!("`branch` requires `list`, `create`, `switch`, or `delete`\n\n{USAGE}");
    };
    let layout = workspace_layout()?;
    let refs = FileRefPointer::new(layout);

    match action {
        "list" => {
            ensure_no_extra(args, "branch list")?;
            print_branches(&list_branches(&refs)?);
            Ok(())
        }
        "create" => {
            let name = require_name(args, "branch create")?;
            let created = create_branch(&CreateBranchInput { name: name.clone() }, &refs)?;
            println!("created {name} at {}", created.at.to_hex());
            Ok(())
        }
        "switch" => {
            let name = require_name(args, "branch switch")?;
            let switched = switch_branch(&SwitchBranchInput { name: name.clone() }, &refs)?;
            println!("switched {} -> {}", switched.previous, switched.current);
            Ok(())
        }
        "delete" => {
            let name = require_name(args, "branch delete")?;
            let deleted = delete_branch(&DeleteBranchInput { name: name.clone() }, &refs)?;
            match deleted.was_at {
                Some(at) => println!("deleted {name} (was at {})", at.to_hex()),
                None => println!("deleted {name}"),
            }
            Ok(())
        }
        unknown => anyhow::bail!("unknown branch action `{unknown}`\n\n{USAGE}"),
    }
}

/// Mengembalikan nama branch dari argumen kedua.
///
/// # Errors
///
/// Mengembalikan pesan yang menyebut bentuk perintah bila nama tidak diberikan.
fn require_name(args: &[String], command: &str) -> Result<String> {
    args.get(1)
        .cloned()
        .filter(|name| !name.is_empty())
        .ok_or_else(|| anyhow::anyhow!("`{command}` requires a branch name\n\n{USAGE}"))
}

/// Menolak argumen berlebih agar salah ketik tidak diam-diam diabaikan.
///
/// # Errors
///
/// Mengembalikan pesan yang menyebut argumen tak terduga bila ada argumen
/// ketiga; ALTERNATIF: menerima dan mengabaikannya membuat pengguna mengira
/// perintah berjalan padahal yang dikehendaki tidak dijalankan.
fn ensure_no_extra(args: &[String], command: &str) -> Result<()> {
    if args.len() > 1 {
        anyhow::bail!("`{command}` takes no branch name, got `{}`", args[1]);
    }
    Ok(())
}

/// Mencetak daftar branch dengan penanda branch aktif dan awalan commit.
fn print_branches(list: &BranchList) {
    if list.branches.is_empty() {
        println!("no branches yet");
        return;
    }
    for branch in &list.branches {
        let marker = if branch.current { "*" } else { " " };
        let head = branch.head.map_or_else(
            || "no commits".to_owned(),
            |id| id.to_hex().chars().take(HEAD_PREFIX_LEN).collect(),
        );
        println!("{marker} {:<20} {head}", branch.name);
    }
}
