//! File: `tree_node_decoding.rs`
//!
//! Deskripsi: Pembacaan byte kanonik node tree.
//! Layer: domain/tree/nodes
//! Tanggung jawab: Menolak byte node yang terpotong atau bukan encoding node.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `tree_node_codec.rs`, `tree_node.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::tree::nodes::tree_node::TreeNode;
use crate::domain::tree::nodes::tree_node_codec::{
    BRANCH_MARKER, HEADER_MARKER, LEAF_MARKER, NODE_TAG,
};
use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Membaca node dari byte kanonik.
///
/// Returns:
/// - Ok(TreeNode) — node yang terbaca.
///
/// # Errors
///
/// Mengembalikan [`MalformedTreeNode`](VergeError::MalformedTreeNode) bila byte
/// bukan encoding kanonik node atau terpotong di tengah.
///
/// Example:
/// ```
/// use verge_core::domain::tree::nodes::tree_node::TreeNode;
/// use verge_core::domain::tree::nodes::tree_node_codec::{decode, encode};
///
/// let node = TreeNode::Branch { children: vec![] };
/// assert!(decode(&encode(&node)).is_ok());
/// assert!(decode(b"bukan node").is_err());
/// ```
pub fn decode(input: &[u8]) -> Result<TreeNode> {
    let mut cursor = 0;
    let tag_end = NODE_TAG.len();
    if input.get(cursor..tag_end) != Some(NODE_TAG) {
        return Err(malformed("unknown tree node tag"));
    }
    cursor = tag_end;
    let marker = *input
        .get(cursor)
        .ok_or_else(|| malformed("truncated node"))?;
    cursor += 1;
    let count = read_count(input, &mut cursor)?;
    match marker {
        HEADER_MARKER => {
            let content = read_field(input, &mut cursor).ok_or_else(truncated)?;
            Ok(TreeNode::Header {
                content: content.to_vec(),
            })
        }
        LEAF_MARKER => {
            let mut rows = Vec::with_capacity(count);
            for _ in 0..count {
                let key = read_field(input, &mut cursor).ok_or_else(truncated)?;
                let value = read_field(input, &mut cursor).ok_or_else(truncated)?;
                rows.push(
                    TableRow::new(RowKey::new(key.to_vec()), value.to_vec())
                        .ok_or_else(|| malformed("leaf node has an empty row key"))?,
                );
            }
            Ok(TreeNode::Leaf { rows })
        }
        BRANCH_MARKER => {
            let mut children = Vec::with_capacity(count);
            for _ in 0..count {
                let raw = input.get(cursor..cursor + 32).ok_or_else(truncated)?;
                let mut bytes = [0_u8; 32];
                bytes.copy_from_slice(raw);
                children.push(BlockId::from_bytes(bytes));
                cursor += 32;
            }
            Ok(TreeNode::Branch { children })
        }
        _ => Err(malformed("unknown node kind")),
    }
}

/// Membaca satu field berpindah prefiks panjang dari `input` pada `cursor`.
fn read_field<'a>(input: &'a [u8], cursor: &mut usize) -> Option<&'a [u8]> {
    let end = cursor.checked_add(8)?;
    let raw: [u8; 8] = input.get(*cursor..end)?.try_into().ok()?;
    let length = usize::try_from(u64::from_le_bytes(raw)).ok()?;
    let end = end.checked_add(length)?;
    let slice = input.get(end - length..end)?;
    *cursor = end;
    Some(slice)
}

/// Membaca jumlah elemen node dengan batas agar byte rusak tidak alokasi besar.
fn read_count(input: &[u8], cursor: &mut usize) -> Result<usize> {
    let end = cursor.checked_add(8).ok_or_else(truncated)?;
    let raw: [u8; 8] = input
        .get(*cursor..end)
        .ok_or_else(truncated)?
        .try_into()
        .map_err(|_| truncated())?;
    *cursor = end;
    usize::try_from(u64::from_le_bytes(raw)).map_err(|_| malformed("element count out of range"))
}

/// Membangun error node dengan alasan yang aman ditampilkan.
fn malformed(reason: &'static str) -> VergeError {
    VergeError::MalformedTreeNode { reason }
}

/// Membangun error node untuk byte node yang lebih pendek dari yang dibaca.
fn truncated() -> VergeError {
    malformed("truncated tree node")
}
