//! File: `block_store_tests.rs`
//!
//! Deskripsi: Kontrak `Store` diuji lewat implementasi in-memory.
//! Layer: domain/storage/ports
//! Tanggung jawab: Membuktikan deduplikasi dan penanganan blok hilang.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `block_store.rs`, `block_id.rs`, `put_outcome.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::cell::RefCell;
use std::collections::HashMap;

use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::storage::value_objects::put_outcome::PutOutcome;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Store in-memory yang dipakai sebagai fake pada test kontrak port.
#[derive(Default)]
struct InMemoryStore {
    blocks: RefCell<HashMap<Digest, Vec<u8>>>,
}

impl Store for InMemoryStore {
    fn put(&self, data: &[u8]) -> Result<PutOutcome> {
        let id = Digest::of(data);
        let inserted = self.blocks.borrow_mut().insert(id, data.to_vec()).is_none();
        Ok(PutOutcome { id, inserted })
    }

    fn get(&self, id: &BlockId) -> Result<Vec<u8>> {
        let found = self.blocks.borrow().get(id).cloned();
        found.ok_or_else(|| VergeError::BlockNotFound {
            id: *id,
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "missing block"),
        })
    }

    fn contains(&self, id: &BlockId) -> bool {
        self.blocks.borrow().contains_key(id)
    }
}

#[test]
fn byte_identik_hanya_disimpan_sekali() {
    let store = InMemoryStore::default();
    let first = store.put(b"same").expect("tulis pertama");
    let second = store.put(b"same").expect("tulis kedua");
    assert!(first.inserted, "blok pertama harus baru");
    assert!(!second.inserted, "blok kedua harus deduplikasi");
    assert_eq!(first.id, second.id, "identifier harus sama");
}

#[test]
fn blok_tidak_ada_dilaporkan_sebagai_error() {
    let store = InMemoryStore::default();
    let absent = Digest::of(b"absent");
    assert!(!store.contains(&absent));
    assert!(matches!(
        store.get(&absent),
        Err(VergeError::BlockNotFound { .. })
    ));
}
