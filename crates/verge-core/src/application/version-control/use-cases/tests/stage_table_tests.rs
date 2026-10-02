//! File: `stage_table_tests.rs`
//!
//! Deskripsi: Test use case `stage_table`.
//! Layer: application/version-control/use-cases/stage-table
//! Tanggung jawab: Membuktikan data sumber tersimpan sebagai blok dan deduplikasi.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `stage_table.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::application::version_control::fakes::world::{any_path, FakeWorld};
use crate::application::version_control::use_cases::stage_table::{stage_table, StageTableInput};
use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;

#[test]
fn data_sumber_tersimpan_sebagai_blok_yang_bisa_dibaca_utuh() {
    let world = FakeWorld::new();
    world
        .source_bytes
        .borrow_mut()
        .extend_from_slice(b"id,name\n1,ana\n");
    let input = StageTableInput {
        table: TableName::parse("users").unwrap(),
        source: any_path(),
    };

    let staged = stage_table(&input, &world.source(), &world).unwrap();

    assert_eq!(staged.bytes, b"id,name\n1,ana\n".len());
    assert_eq!(world.get(&staged.block).unwrap(), b"id,name\n1,ana\n");
    assert_eq!(world.staged(&input.table).unwrap(), Some(staged.block));
}

#[test]
fn data_identik_tidak_menggandakan_blok() {
    let world = FakeWorld::new();
    world
        .source_bytes
        .borrow_mut()
        .extend_from_slice(b"id\n1\n");
    let input = StageTableInput {
        table: TableName::parse("users").unwrap(),
        source: any_path(),
    };

    let first = stage_table(&input, &world.source(), &world).unwrap();
    let second = stage_table(&input, &world.source(), &world).unwrap();

    assert_eq!(first.block, second.block);
    assert_eq!(
        world.blocks.borrow().len(),
        1,
        "hanya satu blok untuk data identik"
    );
}

#[test]
fn tabel_berbeda_dengan_data_sama_tetap_berbagi_blok() {
    let world = FakeWorld::new();
    world
        .source_bytes
        .borrow_mut()
        .extend_from_slice(b"id\n1\n");
    let source = world.source();
    let users = StageTableInput {
        table: TableName::parse("users").unwrap(),
        source: any_path(),
    };
    let orders = StageTableInput {
        table: TableName::parse("orders").unwrap(),
        source: any_path(),
    };

    let first = stage_table(&users, &source, &world).unwrap();
    let second = stage_table(&orders, &source, &world).unwrap();

    assert_eq!(first.block, second.block);
    assert_eq!(world.blocks.borrow().len(), 1);
    assert_ne!(first.table, second.table);
    assert_eq!(BlockId::of(b"id\n1\n"), first.block);
}
