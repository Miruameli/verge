//! File: `merge_writer.rs`
//!
//! Deskripsi: Penulisan hasil merge menjadi commit.
//! Layer: application/version-control/use-cases/merging
//! Tanggung jawab: Membangun tree gabungan lalu menyimpan commit merge.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/tree/**`, `domain/commit/**`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::application::version_control::use_cases::merging::merge_branch::MergeBranchInput;
use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::tree::nodes::tree_node_codec::encode as encode_node;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::tree_builder::build_plan;
use crate::shared::kernel::result::Result;

/// Menulis tabel gabungan sebagai commit merge lalu memindahkan branch aktif.
///
/// KONTEKS: node yang tidak berubah menghasilkan digest sama sehingga `put`
/// mendeduplikasi tanpa kerja tambahan; KENAPA: menulis ulang node yang sama akan
/// membatalkan seluruh keuntungan prolly tree saat merge.
///
/// Args:
/// - input — metadata commit merge.
/// - `current` — branch aktif yang akan digeser.
/// - `ours`, `theirs` — ujung kedua sisi merge.
/// - header — header tabel dari branch aktif.
/// - rows — baris hasil gabungan.
/// - refs — port pointer branch.
/// - commits — port penyimpanan objek commit.
/// - store — port object store blok.
///
/// Returns:
/// - Ok(CommitId) — commit merge yang baru ditulis.
///
/// # Errors
///
/// Mengembalikan error dari port bila blok atau commit gagal ditulis.
#[allow(clippy::too_many_arguments)]
pub fn write_merge_commit(
    input: &MergeBranchInput,
    current: &str,
    ours: CommitId,
    theirs: CommitId,
    header: &[u8],
    rows: Vec<crate::domain::tree::value_objects::table_row::TableRow>,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<CommitId> {
    let plan = build_plan(&TableRows::from_parts(header.to_vec(), rows))?;
    for node in &plan.nodes {
        store.put(&encode_node(node))?;
    }
    let commit = Commit::new(
        vec![ours, theirs],
        plan.root,
        &input.table,
        input.author.clone(),
        input.message.clone(),
        input.timestamp_unix_ms,
    );
    commits.save(&commit)?;
    refs.advance(current, commit.id())?;
    Ok(commit.id())
}
