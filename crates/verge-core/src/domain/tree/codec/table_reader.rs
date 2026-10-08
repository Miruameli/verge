//! File: `table_reader.rs`
//!
//! Deskripsi: Pembacaan prolly tree menjadi baris tabel.
//! Layer: domain/tree/codec
//! Tanggung jawab: Menyusun ulang tabel dari node yang disimpan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `nodes/tree_node.rs`, `table_codec.rs`, `storage/ports/block_store.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!   - #97 (domain/tree split)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use super::TableRows;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::tree::nodes::tree_node::TreeNode;
use crate::domain::tree::nodes::tree_node_codec::decode;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Batas kedalaman tree saat pembacaan.
///
/// Tree dibangun dari atas ke bawah sehingga kedalamannya kecil; batas ini
/// mencegah infinite loop jika ada cycle di data (harus tidak terjadi).
const MAX_DEPTH: usize = 100;

/// Membaca tabel dari block store melalui root node.
///
/// # Errors
/// Mengembalikan error jika root node tidak ditemukan, decode gagal,
/// atau melebihi batas kedalaman.
pub fn read_table(root_id: BlockId, store: &dyn Store) -> Result<TableRows> {
    let mut nodes = Vec::new();
    let mut current_id = root_id.clone();
    let mut depth = 0;
    loop {
        let block = store.get(&current_id)?;
        let node: TreeNode = decode(&block)?;

        match &node {
            TreeNode::Branch { children, .. } => {
                if depth >= MAX_DEPTH {
                    return Err(VergeError::InvalidRef(
                        "Tree depth exceeds MAX_DEPTH".into(),
                    ));
                }
                current_id = children[0].clone();
                depth += 1;
            }
            TreeNode::Leaf { .. } => {
                nodes.push(node);
                break;
            }
            TreeNode::Header { .. } => {
                nodes.push(node);
            }
        }
    }

    // Build table from leaf nodes
    let mut all_rows = Vec::new();
    for node in nodes {
        match node {
            TreeNode::Leaf { rows, .. } => all_rows.extend(rows),
            TreeNode::Branch { .. } => {}
            TreeNode::Header { .. } => {}
        }
    }

    // The first node that was read might have the header in its first row
    // For simplicity, we assume the first leaf has the header
    if let Some(first_leaf) = all_rows.first() {
        // Create a TableRows with the header from the first row
        let header = first_leaf.value().to_vec();
        let mut table = TableRows::new(header);
        for row in all_rows {
            table.push(row);
        }
        Ok(table)
    } else {
        Ok(TableRows::default())
    }
}
