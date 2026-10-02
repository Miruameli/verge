//! File: `tree_node_codec.rs`
//!
//! Deskripsi: Encoding kanonik node prolly tree.
//! Layer: domain/tree/nodes
//! Tanggung jawab: Menghasilkan byte deterministik yang di-hash jadi identifier node.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `tree_node.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::tree::nodes::tree_node::TreeNode;

// Pembacaan node dipisah ke modul anak agar berkas ini hanya berisi satu arah
// penulisan: encoding.
#[path = "tree_node_decoding.rs"]
mod decoding;

pub use decoding::decode;

/// Prefix domain-separation agar digest node tidak pernah sama dengan digest
/// blok lain yang kebetulan berisi byte serupa.
pub(super) const NODE_TAG: &[u8] = b"verge-tree-node-v1";

/// Penanda node daun.
pub(super) const LEAF_MARKER: u8 = 0;

/// Penanda node internal.
pub(super) const BRANCH_MARKER: u8 = 1;

/// Penanda node header.
pub(super) const HEADER_MARKER: u8 = 2;

/// Mengencode node tree menjadi byte kanonik.
///
/// Returns:
/// - Vec<u8> — encoding deterministik dan stabil lintas platform.
///
/// Example:
/// ```
/// use verge_core::domain::tree::nodes::tree_node::TreeNode;
/// use verge_core::domain::tree::nodes::tree_node_codec::encode;
///
/// let node = TreeNode::Branch { children: vec![] };
/// assert_eq!(encode(&node).is_empty(), false);
/// ```
#[must_use]
pub fn encode(node: &TreeNode) -> Vec<u8> {
    let mut out = Vec::with_capacity(node.encoded_size() + NODE_TAG.len() + 16);
    out.extend_from_slice(NODE_TAG);
    match node {
        TreeNode::Header { content } => {
            out.push(HEADER_MARKER);
            out.extend_from_slice(&1_u64.to_le_bytes());
            push_field(&mut out, content);
        }
        TreeNode::Leaf { rows } => {
            out.push(LEAF_MARKER);
            out.extend_from_slice(&(rows.len() as u64).to_le_bytes());
            for row in rows {
                push_field(&mut out, row.key().as_bytes());
                push_field(&mut out, row.value());
            }
        }
        TreeNode::Branch { children } => {
            out.push(BRANCH_MARKER);
            out.extend_from_slice(&(children.len() as u64).to_le_bytes());
            for child in children {
                out.extend_from_slice(child.as_bytes());
            }
        }
    }
    out
}

/// Menambahkan field berpindah prefiks panjang ke `out`.
fn push_field(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    out.extend_from_slice(bytes);
}
