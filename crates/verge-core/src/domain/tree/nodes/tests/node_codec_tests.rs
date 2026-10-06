//! File: `node_codec_tests.rs`
//!
//! Deskripsi: Test encoding kanonik node tree.
//! Layer: domain/tree/nodes
//! Tanggung jawab: Membuktikan byte node deterministik dan menolak byte rusak.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `tree_node.rs`, `tree_node_codec.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::tree::nodes::tree_node::TreeNode;
use crate::domain::tree::nodes::tree_node_codec::{decode, encode};
use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Membuat baris dengan kunci dan nilai tertentu.
fn row(key: &str, value: &str) -> TableRow {
    TableRow::new(
        RowKey::new(key.as_bytes().to_vec()),
        value.as_bytes().to_vec(),
    )
    .expect("kunci tidak kosong")
}

#[test]
fn node_daun_dapat_dibaca_kembali_dari_encodingnya() {
    let node = TreeNode::Leaf {
        rows: vec![row("1", ",ana"), row("2", ",budi")],
    };

    let decoded = decode(&encode(&node)).expect("node terbaca");

    assert_eq!(decoded, node);
}

#[test]
fn node_internal_dapat_dibaca_kembali_dari_encodingnya() {
    let node = TreeNode::Branch {
        children: vec![Digest::of(b"a"), Digest::of(b"b")],
    };

    assert_eq!(decode(&encode(&node)).expect("node terbaca"), node);
}

#[test]
fn node_dengan_isi_sama_selalu_memiliki_identifier_sama() {
    let left = TreeNode::Leaf {
        rows: vec![row("1", ",ana")],
    };
    let right = TreeNode::Leaf {
        rows: vec![row("1", ",ana")],
    };

    assert_eq!(Digest::of(&encode(&left)), Digest::of(&encode(&right)));
}

#[test]
fn node_daun_dan_internal_dengan_panjang_sama_tidak_bertabrakan() {
    // Domain separation tag menjaga digest node daun dan internal tetap berbeda
    // walau jumlah elemennya sama.
    let leaf = TreeNode::Leaf { rows: vec![] };
    let branch = TreeNode::Branch { children: vec![] };

    assert_ne!(Digest::of(&encode(&leaf)), Digest::of(&encode(&branch)));
}

#[test]
fn encoding_node_bukan_kanonik_ditolak_saat_dibaca() {
    let mut bytes = encode(&TreeNode::Branch {
        children: vec![Digest::of(b"a")],
    });
    bytes.truncate(bytes.len() - 1);

    assert!(decode(&bytes).is_err(), "byte terpotong harus ditolak");
    assert!(decode(b"bukan node sama sekali").is_err());
    assert!(decode(&[]).is_err(), "byte kosong harus ditolak");
}

#[test]
fn node_daun_dan_internal_melaporkan_kemampuan_dengan_benar() {
    let leaf = TreeNode::Leaf {
        rows: vec![row("1", ",ana")],
    };
    let branch = TreeNode::Branch {
        children: vec![Digest::of(b"a")],
    };

    assert!(leaf.is_leaf());
    assert!(!branch.is_leaf());
    assert_eq!(leaf.rows().len(), 1);
    assert_eq!(branch.children().len(), 1);
    assert!(branch.rows().is_empty());
    assert!(leaf.children().is_empty());
    assert_eq!(leaf.row_count(), 1);
    assert_eq!(branch.row_count(), 1);
}

#[test]
fn baris_memakai_nilai_yang_tidak_bisa_ditafsirkan_sebagai_utf8() {
    let node = TreeNode::Leaf {
        rows: vec![
            TableRow::new(RowKey::new(b"1".to_vec()), vec![0xff, 0xfe, b','])
                .expect("kunci tidak kosong"),
        ],
    };

    assert_eq!(decode(&encode(&node)).expect("node terbaca"), node);
}
