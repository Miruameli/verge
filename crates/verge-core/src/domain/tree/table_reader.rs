//! File: `table_reader.rs`
//!
//! Deskripsi: Pembacaan prolly tree menjadi baris tabel.
//! Layer: domain/tree
//! Tanggung jawab: Menyusun ulang tabel dari node yang disimpan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `nodes/tree_node.rs`, `table_codec.rs`, `storage/ports/block_store.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::tree::nodes::tree_node::TreeNode;
use crate::domain::tree::nodes::tree_node_codec::decode;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Batas kedalaman tree saat pembacaan.
///
/// Tree dibangun dari atas ke bawah sehingga kedalamannya kecil; batas ini
/// mencegah node rusak membuat pembacaan berulang tanpa henti.
///
/// KENAPA: nilai 64 jauh di atas kedalaman tree yang realistis (9), sehingga
///         tidak memotong tree yang sahih tetapi tetap menghentikan siklus.
const MAX_DEPTH: usize = 64;

/// Membaca seluruh baris tabel yang ditunjuk `root`.
///
/// Args:
/// - root — identifier node akar tree.
/// - store — port object store.
///
/// Returns:
/// - Ok(TableRows) — baris tabel tersusun ulang.
///
/// # Errors
///
/// Mengembalikan [`MalformedTreeNode`](VergeError::MalformedTreeNode) bila node
/// bukan encoding kanonik atau tree melebihi batas kedalaman, serta
/// [`BlockNotFound`](VergeError::BlockNotFound) bila ada node yang hilang.
///
/// Example:
/// ```ignore
/// use verge_core::domain::tree::table_reader::read_table;
/// // Fungsi ini memerlukan object store; contoh lengkap ada di integration test.
/// ```
pub fn read_table(root: BlockId, store: &dyn Store) -> Result<TableRows> {
    let mut header: Option<Vec<u8>> = None;
    let rows = collect(root, store, 0, &mut header)?;
    let header = header.ok_or(VergeError::MalformedTreeNode {
        reason: "tree has no header node",
    })?;
    Ok(TableRows::from_parts(header, rows))
}

/// Mengumpulkan baris dari satu node dan seluruh keturunannya.
fn collect(
    root: BlockId,
    store: &dyn Store,
    depth: usize,
    header: &mut Option<Vec<u8>>,
) -> Result<Vec<TableRow>> {
    if depth > MAX_DEPTH {
        return Err(VergeError::MalformedTreeNode {
            reason: "tree is deeper than the allowed maximum",
        });
    }
    let node = decode(&store.get(&root)?)?;
    match node {
        TreeNode::Header { content } => {
            *header = Some(content);
            Ok(Vec::new())
        }
        TreeNode::Leaf { rows } => Ok(rows),
        TreeNode::Branch { children } => {
            let mut collected = Vec::new();
            for child in children {
                collected.extend(collect(child, store, depth + 1, header)?);
            }
            Ok(collected)
        }
    }
}
