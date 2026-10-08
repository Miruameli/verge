//! File: `stage_table_tests.rs`
//!
//! Deskripsi: Test use case `stage_table`.
//! Layer: application/version-control/use-cases/tests
//! Tanggung jawab: Membuktikan tree tersimpan dan blok berulang di-dedup.
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
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::application::version_control::fakes::world::{any_path, FakeWorld};
use crate::application::version_control::use_cases::stage_table::{stage_table, StageTableInput};
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::codec::table_reader::read_table;
use crate::shared::exceptions::verge_error::VergeError;

use super::{bulk_table, leaf_ids, stage};

#[test]
fn data_sumber_tersimpan_sebagai_tree_yang_bisa_dibaca_utuh() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    let data = b"id,name\n1,ana\n2,budi\n";

    let staged = stage(&world, &table, data);

    assert_eq!(staged.rows, 2);
    assert_eq!(world.staged(&table).unwrap(), Some(staged.root));
    assert_eq!(read_table(staged.root, &world).unwrap().to_bytes(), data);
    assert_eq!(
        staged.bytes,
        world.blocks.borrow().values().map(Vec::len).sum()
    );
}

#[test]
fn import_berulang_dengan_isi_identik_tidak_menambah_blok_baru() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    let data = b"id,nilai\n1,ana\n2,budi\n";

    let pertama = stage(&world, &table, data);
    let blok_pertama = world.blocks.borrow().len();
    let kedua = stage(&world, &table, data);
    let ketiga = stage(&world, &table, data);

    assert_eq!(blok_pertama, 3, "header, satu daun, dan satu akar");
    assert_eq!(pertama.root, kedua.root);
    assert_eq!(kedua.root, ketiga.root);
    assert_eq!(world.blocks.borrow().len(), blok_pertama);
}

#[test]
fn baris_yang_tidak_berubah_tetap_memakai_blok_daun_yang_sama() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    stage(&world, &table, &bulk_table(false));
    let daun_sebelum = leaf_ids(&world);
    let blok_sebelum = world.blocks.borrow().len();

    stage(&world, &table, &bulk_table(true));

    let daun_sesudah = leaf_ids(&world);
    let dibagi = daun_sesudah
        .iter()
        .filter(|id| daun_sebelum.contains(id))
        .count();
    assert!(
        daun_sebelum.len() > 1,
        "tabel uji harus dipecah beberapa daun"
    );
    assert!(
        daun_sesudah.len() < blok_sebelum,
        "blok yang sama tidak cukup untuk dua versi tabel"
    );
    assert_eq!(
        dibagi,
        daun_sebelum.len(),
        "setiap daun lama harus masih dipakai ulang"
    );
    assert_eq!(
        daun_sesudah.len(),
        daun_sebelum.len() + 1,
        "hanya daun pemuat baris berubah yang baru"
    );
}

#[test]
fn isi_tanpa_baris_ditolak_tanpa_menulis_blok_apapun() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    world.set_source(b"id,name\n");

    let error = stage_table(
        &StageTableInput {
            table: table.clone(),
            source: any_path(),
        },
        &world.source(),
        &world,
        &world,
    )
    .expect_err("tabel tanpa baris tidak punya akar tree");

    assert!(matches!(error, VergeError::EmptyTable));
    assert_eq!(world.staged(&table).unwrap(), None);
    assert!(world.blocks.borrow().is_empty());
}
