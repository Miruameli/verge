//! File: `tree_builder.rs`
//!
//! Deskripsi: Pembangun prolly tree dari baris tabel terurut.
//! Layer: domain/tree
//! Tanggung jawab: Mempartisi baris menjadi blok yang dapat dipakai ulang.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `nodes/tree_node.rs`, `nodes/tree_node_codec.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::tree::nodes::tree_node::{TreeNode, LEAF_SIZE_LIMIT};
use crate::domain::tree::nodes::tree_node_codec::encode;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Partisi baris menjadi node tree sebelum ditulis ke store.
///
/// Invariants:
/// - Partisi hanya bergantung pada isi baris dan batas ukuran, bukan pada urutan
///   penulisan; dua kali build dengan baris sama menghasilkan partisi sama.
/// - Setiap daun memuat minimal satu baris dan semua baris tercakup tepat sekali.
/// - `root` adalah identifier node teratas; node internal selalu memuat anak
///   header sebagai anak pertama sehingga tabel dapat disusun ulang utuh.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreePlan {
    /// Node yang perlu ditulis, daun lebih dulu lalu ke arah akar.
    pub nodes: Vec<TreeNode>,
    /// Identifier setiap daun sesuai urutan baris.
    pub leaf_ids: Vec<BlockId>,
    /// Identifier node akar.
    pub root: BlockId,
}

/// Mempartisi `rows` menjadi node tree dan menghitung identifier tiap node.
///
/// Args:
/// - table — tabel terurai; barisnya harus sudah terurut dan unik.
///
/// Returns:
/// - Ok(TreePlan) — partisi node beserta identifier akarnya.
///
/// # Errors
///
/// Mengembalikan [`EmptyTable`](VergeError::EmptyTable) bila tidak ada baris,
/// karena tabel kosong tidak punya akar tree.
///
/// Example:
/// ```
/// use verge_core::domain::tree::table_codec::TableRows;
/// use verge_core::domain::tree::tree_builder::build_plan;
///
/// let rows = TableRows::parse(b"id,name\n1,ana\n2,budi\n").unwrap();
/// let plan = build_plan(&rows).unwrap();
/// // Dua baris kecil muat dalam satu daun: header, daun, lalu akar internal.
/// assert_eq!(plan.leaf_ids.len(), 1);
/// assert_eq!(plan.nodes.len(), 3);
/// ```
pub fn build_plan(table: &TableRows) -> Result<TreePlan> {
    let rows = table.rows();
    if rows.is_empty() {
        return Err(VergeError::EmptyTable);
    }
    let header = TreeNode::Header {
        content: table.header().to_vec(),
    };
    let leaves = partition_leaves(rows);
    let mut nodes: Vec<TreeNode> = Vec::with_capacity(leaves.len() + 1);
    let mut leaf_ids = Vec::with_capacity(leaves.len());
    for leaf in leaves {
        leaf_ids.push(Digest::of(&encode(&leaf)));
        nodes.push(leaf);
    }
    let mut level = vec![Digest::of(&encode(&header))];
    level.extend(leaf_ids.clone());
    nodes.insert(0, header);
    while level.len() > 1 {
        let branch = TreeNode::Branch { children: level };
        level = vec![Digest::of(&encode(&branch))];
        nodes.push(branch);
    }
    let root = level[0];
    Ok(TreePlan {
        nodes,
        leaf_ids,
        root,
    })
}

/// Mempartisi baris menjadi daun sesuai batas ukuran node.
fn partition_leaves(rows: &[TableRow]) -> Vec<TreeNode> {
    let mut leaves = Vec::new();
    let mut current: Vec<TableRow> = Vec::new();
    let mut current_size = 0_usize;
    for row in rows {
        let row_size = row_size(row);
        let exceeds = !current.is_empty() && current_size + row_size > LEAF_SIZE_LIMIT;
        if exceeds {
            leaves.push(TreeNode::Leaf {
                rows: std::mem::take(&mut current),
            });
            current_size = 0;
        }
        current_size += row_size;
        current.push(row.clone());
    }
    if !current.is_empty() {
        leaves.push(TreeNode::Leaf { rows: current });
    }
    leaves
}

/// Menghitung ukuran baris setelah encoding, termasuk overhead field.
fn row_size(row: &TableRow) -> usize {
    row.key().as_bytes().len() + row.value().len() + 16
}
