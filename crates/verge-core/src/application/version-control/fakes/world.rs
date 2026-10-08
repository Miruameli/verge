//! File: `world.rs`
//!
//! Deskripsi: Dunia in-memory untuk test use case version control.
//! Layer: application/version-control/fakes
//! Tanggung jawab: Menyimpan state tabel, blok, commit, dan branch saat test.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `world_ports.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)
//!   - ADR-0006 (Prolly tree untuk tabel)

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::builder::build_plan;
use crate::domain::tree::codec::TableRows;
use crate::domain::tree::nodes::tree_node_codec::encode;

// Implementasi port dipisah ke modul anak agar berkas ini hanya mendeskripsikan state.
#[path = "world_ports.rs"]
mod ports;
#[path = "world_tag_ports.rs"]
mod tag_ports;

/// Seluruh port version control diimplementasikan sekali di memori.
///
/// Test memakai fake yang sama untuk kelima use case sehingga perilaku use case
/// dapat dibuktikan tanpa filesystem.
#[derive(Debug, Default)]
pub struct FakeWorld {
    /// Data kerja per tabel.
    pub working: RefCell<BTreeMap<String, BlockId>>,
    /// Isi blok per identifier.
    pub blocks: RefCell<BTreeMap<BlockId, Vec<u8>>>,
    /// Objek commit per identifier, disimpan sebagai byte kanonik.
    pub commits: RefCell<BTreeMap<CommitId, Vec<u8>>>,
    /// Branch aktif.
    pub head: RefCell<String>,
    /// Commit terakhir per branch.
    pub branches: RefCell<BTreeMap<String, CommitId>>,
    /// Commit yang ditunjuk tiap tag.
    pub tags: RefCell<BTreeMap<String, CommitId>>,
    /// Isi yang dikembalikan sumber data.
    pub source_bytes: RefCell<Vec<u8>>,
}

impl FakeWorld {
    /// Membuat dunia baru dengan branch aktif `main`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            head: RefCell::new("main".to_owned()),
            ..Self::default()
        }
    }

    /// Membuat sumber data yang mengembalikan `source_bytes`.
    #[must_use]
    pub fn source(&self) -> FakeSource<'_> {
        FakeSource(self)
    }

    /// Membuat sumber data yang mengembalikan `contents`.
    #[must_use]
    pub fn source_with<'a>(&'a self, contents: &'a str) -> FakeSource<'a> {
        self.set_source(contents.as_bytes());
        FakeSource(self)
    }

    /// Membangun tree tabel dari isi mentah lalu menunjuk akarnya sebagai data
    /// kerja, persis seperti `stage_table` menuliskan node ke store.
    ///
    /// # Panics
    ///
    /// Panik bila isi tabel tidak dapat diurai atau tidak punya baris; fake
    /// hanya dipakai test dengan fixture yang memang valid.
    pub fn stage_table(&self, table: &TableName, raw: &[u8]) -> BlockId {
        let rows = TableRows::parse(raw).expect("isi tabel fake harus valid");
        let plan = build_plan(&rows).expect("tabel fake harus punya baris");
        for node in &plan.nodes {
            self.put_block(&encode(node));
        }
        self.working
            .borrow_mut()
            .insert(table.as_str().to_owned(), plan.root);
        plan.root
    }

    /// Menyimpan byte sebagai blok dan mengembalikan identifier-nya.
    pub fn put_block(&self, data: &[u8]) -> BlockId {
        let id = BlockId::of(data);
        self.blocks
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| data.to_vec());
        id
    }

    /// Mengatur branch aktif.
    pub fn set_head(&self, branch: &str) {
        branch.clone_into(&mut self.head.borrow_mut());
    }

    /// Mengganti isi yang akan dikembalikan sumber data.
    pub fn set_source(&self, data: &[u8]) {
        *self.source_bytes.borrow_mut() = data.to_vec();
    }
}

/// Sumber data in-memory yang mengembalikan isi [`FakeWorld::source_bytes`].
#[derive(Debug)]
pub struct FakeSource<'a>(pub(crate) &'a FakeWorld);

impl FakeSource<'_> {
    /// Mengembalikan isi sumber yang sudah disiapkan; fake tidak pernah gagal.
    pub(crate) fn contents(&self) -> Vec<u8> {
        self.0.source_bytes.borrow().clone()
    }
}

/// Membuat path arbitrer untuk pemanggilan use case yang tidak membacanya.
#[must_use]
pub fn any_path() -> PathBuf {
    PathBuf::from("users.csv")
}
