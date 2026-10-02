//! File: `mod.rs`
//!
//! Deskripsi: Perintah CLI versioning tabel.
//! Layer: interfaces/cli/commands/table-versioning
//! Tanggung jawab: Deklarasikan `import`/`commit`/`log`/`show` dan helper argv.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `stage_table.rs`, `record_commit.rs`, `read_history.rs`, `read_snapshot.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::path::PathBuf;

use verge_core::config::repository_layout::RepositoryLayout;
use verge_core::domain::ident::value_objects::digest::Digest;
use verge_core::domain::ident::value_objects::digest_text::HexText;
use verge_core::domain::storage::ports::block_store_factory::BlockStoreFactory;
use verge_core::domain::storage::ports::metadata_writer::MetadataWriter;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;
use verge_core::infrastructure::storage::file_system::file_store_factory::FileStoreFactory;
use verge_core::infrastructure::storage::file_system::local_file_system::LocalFileSystem;
use verge_core::shared::exceptions::verge_error::VergeError;

use crate::config::cli_usage::USAGE;
use crate::shared::kernel::result::Result;

pub mod read_history;
pub mod read_snapshot;
pub mod record_commit;
pub mod stage_table;

/// Panjang prefiks digest yang dicetak sebagai ringkasan.
const SHORT_ID_LEN: usize = 12;

/// Folder kerja tempat repository dicari.
const WORKSPACE: &str = ".";

/// Pasangan flag dan nilainya setelah argv dipecah.
pub(super) type Options<'a> = Vec<(&'a str, &'a str)>;

/// Hasil pemecahan argv: argumen posisional dan pasangan flag.
pub(super) type ParsedArgs<'a> = (Vec<&'a str>, Options<'a>);

/// Layout repository pada folder kerja, bila `.verge` benar-benar ada.
///
/// Existence check memakai adapter `MetadataWriter` dan bukan `std::fs` agar
/// lapisan ini tetap bebas I/O langsung.
///
/// # Errors
///
/// Mengembalikan [`NotARepository`](VergeError::NotARepository) bila folder
/// kerja bukan repository Verge.
pub(super) fn workspace_layout() -> Result<RepositoryLayout> {
    let layout = RepositoryLayout::under(WORKSPACE);
    if !LocalFileSystem.exists(&layout.root) {
        return Err(VergeError::NotARepository(PathBuf::from(WORKSPACE)).into());
    }
    Ok(layout)
}

/// Membuka object store milik `layout` lewat factory filesystem lokal.
///
/// # Errors
///
/// Mengembalikan error I/O bila direktori objek tidak dapat disiapkan.
pub(super) fn object_store(layout: &RepositoryLayout) -> Result<FileBlockStore> {
    Ok(BlockStoreFactory::open(
        &FileStoreFactory,
        &layout.objects(),
    )?)
}

/// Merender `id` sebagai 12 karakter hex pertama.
pub(super) fn short_id(id: &Digest) -> String {
    id.to_hex().chars().take(SHORT_ID_LEN).collect()
}

/// Memecah argv menjadi argumen posisional dan pasangan flag.
///
/// Args:
/// - command — nama perintah, dipakai pada pesan error.
/// - allowed — flag yang boleh muncul.
/// - args — argumen setelah nama perintah.
///
/// # Errors
///
/// Mengembalikan pesan ke pengguna bila flag dikenal tanpa nilai, flag tak
/// dikenal, atau lebih dari satu argumen posisional.
pub(super) fn split_args<'a>(
    command: &str,
    allowed: &[&str],
    args: &'a [String],
) -> Result<ParsedArgs<'a>> {
    let mut positional: Vec<&str> = Vec::new();
    let mut options: Options<'_> = Vec::new();
    let mut remaining = args.iter().map(String::as_str);
    while let Some(arg) = remaining.next() {
        if allowed.contains(&arg) {
            let value = remaining
                .next()
                .ok_or_else(|| anyhow::anyhow!("`{arg}` requires a value\n\n{USAGE}"))?;
            options.push((arg, value));
        } else if arg.starts_with('-') {
            anyhow::bail!("unknown option `{arg}` for `{command}`\n\n{USAGE}");
        } else {
            positional.push(arg);
        }
    }
    if positional.len() > 1 {
        anyhow::bail!("`{command}` takes at most one argument\n\n{USAGE}");
    }
    Ok((positional, options))
}

/// Mengambil nilai flag `name`, menolak flag yang muncul dua kali.
///
/// # Errors
///
/// Mengembalikan pesan ke pengguna bila `name` diulang.
pub(super) fn flag<'a>(
    command: &str,
    options: &Options<'a>,
    name: &str,
) -> Result<Option<&'a str>> {
    let mut matches = options
        .iter()
        .filter(|(key, _)| *key == name)
        .map(|(_, value)| *value);
    let first = matches.next();
    if matches.next().is_some() {
        anyhow::bail!("`{name}` was given more than once for `{command}`\n\n{USAGE}");
    }
    Ok(first)
}
