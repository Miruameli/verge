//! File: `table_workspace_tests.rs`
//!
//! Deskripsi: Test kontrak port data kerja tabel.
//! Layer: domain/table/ports
//! Tanggung jawab: Memastikan setiap implementasi port memenuhi kontrak yang sama.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `table_workspace.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::kernel::result::Result;

/// Fake in-memory: apa pun isi `stage`, identitasnya harus bisa dibaca kembali.
#[derive(Debug, Default)]
struct InMemoryWorkspace {
    staged: std::cell::RefCell<std::collections::BTreeMap<String, BlockId>>,
}

impl TableWorkspace for InMemoryWorkspace {
    fn stage(&self, name: &TableName, data: &[u8]) -> Result<BlockId> {
        let id = BlockId::of(data);
        self.staged
            .borrow_mut()
            .insert(name.as_str().to_owned(), id);
        Ok(id)
    }

    fn staged(&self, name: &TableName) -> Result<Option<BlockId>> {
        Ok(self.staged.borrow().get(name.as_str()).copied())
    }
}

#[test]
fn blok_yang_sama_dipakai_untuk_data_yang_sama() {
    let workspace = InMemoryWorkspace::default();
    let table = TableName::parse("users").unwrap();
    let first = workspace.stage(&table, b"a,1\n").unwrap();
    let second = workspace.stage(&table, b"a,1\n").unwrap();
    assert_eq!(first, second);
    assert_eq!(workspace.staged(&table).unwrap(), Some(first));
}

#[test]
fn tabel_yang_belum_di_stage_tidak_punya_data_kerja() {
    let workspace = InMemoryWorkspace::default();
    let table = TableName::parse("orders").unwrap();
    assert_eq!(workspace.staged(&table).unwrap(), None);
}
