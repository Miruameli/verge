//! File: `tree_node.rs`
//!
//! Deskripsi: Node prolly tree sebagai node daun atau node internal.
//! Layer: domain/tree/nodes
//! Tanggung jawab: Menyimpan baris secara terpartisi dengan identifier stabil.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `../value-objects/table_row.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Batas ukuran node daun dalam byte setelah encoding.
///
/// Batas ini membuat partisi determined: baris ditambahkan ke daun berjalan
/// selama ukurannya belum melebihi batas, tanpa bergantung pada urutan
/// penulisan lain.
pub const LEAF_SIZE_LIMIT: usize = 4096;

/// Node prolly tree.
///
/// Invariants:
/// - Node daun menyimpan baris terurut dan tidak kosong; node internal menyimpan
///   identifier anak yang terurut dan tidak kosong.
/// - Identifier node = digest encoding kanonik, sehingga node dengan isi sama
///   selalu berbagi blok.
///
/// Immutability: penuh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeNode {
    /// Node header yang memuat baris kolom tabel.
    Header {
        /// Isi baris kolom apa adanya, tanpa baris baru.
        content: Vec<u8>,
    },
    /// Node daun yang memuat baris tabel.
    Leaf {
        /// Baris terurut menurut kunci.
        rows: Vec<TableRow>,
    },
    /// Node internal yang menunjuk node anak.
    Branch {
        /// Identifier node anak terurut.
        children: Vec<BlockId>,
    },
}

impl TreeNode {
    /// Menghitung ukuran node dalam byte setelah encoding.
    ///
    /// Returns:
    /// - usize — ukuran encoding, dipakai untuk menentukan partisi daun.
    #[must_use]
    pub fn encoded_size(&self) -> usize {
        match self {
            Self::Header { content } => content.len() + 16,
            Self::Leaf { rows } => rows
                .iter()
                .map(|row| row.key().as_bytes().len() + row.value().len() + 16)
                .sum(),
            Self::Branch { children } => children.len() * 32,
        }
    }

    /// Mengembalikan isi node header.
    #[must_use]
    pub fn header(&self) -> &[u8] {
        match self {
            Self::Header { content } => content,
            Self::Leaf { .. } | Self::Branch { .. } => &[],
        }
    }

    /// Mengembalikan baris pada node daun.
    #[must_use]
    pub fn rows(&self) -> &[TableRow] {
        match self {
            Self::Leaf { rows } => rows,
            Self::Header { .. } | Self::Branch { .. } => &[],
        }
    }

    /// Mengembalikan identifier anak pada node internal.
    #[must_use]
    pub fn children(&self) -> &[BlockId] {
        match self {
            Self::Branch { children } => children,
            Self::Header { .. } | Self::Leaf { .. } => &[],
        }
    }

    /// Melaporkan apakah node adalah daun.
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf { .. })
    }

    /// Menghitung jumlah baris yang dipuat node.
    #[must_use]
    pub fn row_count(&self) -> usize {
        match self {
            Self::Leaf { rows } => rows.len(),
            Self::Branch { children } => children.len(),
            Self::Header { .. } => 0,
        }
    }
}
