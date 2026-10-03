//! File: `mod.rs`
//!
//! Deskripsi: Test use case version control.
//! Layer: application/version-control/use-cases/tests
//! Tanggung jawab: Mendeklarasikan test kelima use case dan helper bersama.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `diff_tables_tests.rs`
//!   - `read_history_tests.rs`
//!   - `read_snapshot_tests.rs`
//!   - `stage_table_tests.rs`
//!   - `recording/record_commit_tests.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

mod diff_tables_tests;
mod read_history_tests;
mod read_snapshot_tests;
mod stage_table_tests;

#[path = "recording/mod.rs"]
mod recording;

use crate::application::version_control::dtos::recorded_commit::RecordedCommit;
use crate::application::version_control::dtos::staged_table::StagedTable;
use crate::application::version_control::dtos::table_diff_report::TableDiffReport;
use crate::application::version_control::fakes::world::{any_path, FakeWorld};
use crate::application::version_control::use_cases::diff_tables::{diff_tables, DiffTablesInput};
use crate::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use crate::application::version_control::use_cases::stage_table::{stage_table, StageTableInput};
use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::nodes::tree_node_codec::decode;
use crate::shared::kernel::result::Result;

/// Jumlah baris tabel besar yang dipakai uji berbagi daun.
pub(crate) const BULK_ROWS: usize = 60;

/// Panjang nilai tiap baris tabel besar.
pub(crate) const VALUE_LEN: usize = 80;

/// Indeks baris yang diganti nilai pada tabel besar versi kedua.
pub(crate) const CHANGED_ROW: usize = 45;

/// Menyusun tabel besar berisi [`BULK_ROWS`] baris; baris [`CHANGED_ROW`]
/// memakai nilai lain dengan panjang sama sehingga batas partisi daun tetap.
pub(crate) fn bulk_table(changed: bool) -> Vec<u8> {
    let mut raw = b"id,nilai\n".to_vec();
    for index in 0..BULK_ROWS {
        let filler = if changed && index == CHANGED_ROW {
            "z"
        } else {
            "x"
        };
        raw.extend_from_slice(format!("k{index:03},{}\n", filler.repeat(VALUE_LEN)).as_bytes());
    }
    raw
}

/// Mengembalikan identifier blok daun yang tersimpan di `world`.
pub(crate) fn leaf_ids(world: &FakeWorld) -> Vec<BlockId> {
    world
        .blocks
        .borrow()
        .keys()
        .copied()
        .filter(|id| {
            decode(&world.get(id).expect("blok tes tersimpan"))
                .map(|node| node.is_leaf())
                .unwrap_or(false)
        })
        .collect()
}

/// Menjalankan `stage_table` atas isi mentah `data` lewat dunia in-memory.
pub(crate) fn stage(world: &FakeWorld, table: &TableName, data: &[u8]) -> StagedTable {
    world.set_source(data);
    let input = StageTableInput {
        table: table.clone(),
        source: any_path(),
    };
    stage_table(&input, &world.source(), world, world).expect("data kerja tersimpan")
}

/// Menyimpan tabel lalu membuat commit-nya; mengembalikan commit yang tercatat.
///
/// # Panics
///
/// Panik bila use case menolak; fixture test selalu memakai tabel yang valid.
pub(crate) fn commit(
    world: &FakeWorld,
    table: &str,
    data: &[u8],
    message: &str,
    time: i64,
) -> RecordedCommit {
    let name = TableName::parse(table).unwrap();
    stage(world, &name, data);
    record_commit(
        &RecordCommitInput {
            table: name,
            message: message.to_owned(),
            author: "ana".to_owned(),
        },
        world,
        world,
        world,
        time,
    )
    .expect("commit untuk test")
}

/// Menjalankan `diff_tables` atas dua revisi bebas bentuk.
pub(crate) fn diff(world: &FakeWorld, from: &str, to: &str) -> Result<TableDiffReport> {
    let input = DiffTablesInput {
        table: TableName::parse("users").unwrap(),
        from: from.to_owned(),
        to: to.to_owned(),
    };
    diff_tables(&input, world, world, world, world)
}

/// Membandingkan dua commit berurutan sambil memeriksa identitas laporannya.
pub(crate) fn report(
    world: &FakeWorld,
    lama: &RecordedCommit,
    baru: &RecordedCommit,
) -> TableDiffReport {
    let found = diff(world, &lama.id.to_string(), &baru.id.to_string()).expect("perbandingan");
    assert_eq!(found.from, lama.id);
    assert_eq!(found.to, baru.id);
    found
}
