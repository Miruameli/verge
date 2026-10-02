//! File: `read_history_tests.rs`
//!
//! Deskripsi: Test use case `read_history`.
//! Layer: application/version-control/use-cases/tests
//! Tanggung jawab: Membuktikan penyaringan per tabel dan urutan riwayat.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `read_history.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::use_cases::read_history::{
    read_history, ReadHistoryInput,
};
use crate::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;

use super::commit;

/// Membaca riwayat dengan batas tertentu.
fn history(world: &FakeWorld, table: &str, limit: usize) -> Vec<String> {
    let input = ReadHistoryInput {
        table: TableName::parse(table).unwrap(),
        limit,
    };
    read_history(&input, world, world)
        .expect("riwayat terbaca")
        .into_iter()
        .map(|entry| entry.summary)
        .collect()
}

#[test]
fn riwayat_terbaru_lebih_dulu_dan_dibatasi_limit() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id\n1,ana\n", "feat: satu", 1_000);
    commit(&world, "users", b"id\n1,ana\n2,budi\n", "feat: dua", 2_000);
    commit(
        &world,
        "users",
        b"id\n1,ana\n2,budi\n3,sari\n",
        "feat: tiga",
        3_000,
    );

    assert_eq!(
        history(&world, "users", 2),
        ["feat: tiga".to_owned(), "feat: dua".to_owned()]
    );
}

#[test]
fn riwayat_disaring_per_tabel_pada_rantai_yang_sama() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id\n1,ana\n", "feat: users satu", 1_000);
    commit(&world, "orders", b"id\n1,sepatu\n", "feat: orders", 2_000);
    commit(
        &world,
        "users",
        b"id\n1,ana\n2,budi\n",
        "feat: users dua",
        3_000,
    );

    assert_eq!(
        history(&world, "users", 10),
        ["feat: users dua".to_owned(), "feat: users satu".to_owned()]
    );
    assert_eq!(history(&world, "orders", 10), ["feat: orders".to_owned()]);
}

#[test]
fn branch_tanpa_commit_menolak_pembacaan_riwayat() {
    let world = FakeWorld::new();
    let input = ReadHistoryInput {
        table: TableName::parse("users").unwrap(),
        limit: 5,
    };

    let error = read_history(&input, &world, &world).expect_err("branch kosong harus ditolak");

    assert!(matches!(error, VergeError::HeadUnborn(branch) if branch == "main"));
}

#[test]
fn entri_memuat_penulis_dan_waktu_commit() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    world.stage_table(&table, b"id\n1,ana\n");
    let recorded = record_commit(
        &RecordCommitInput {
            table: table.clone(),
            message: "feat: satu\n\nparagraf kedua".to_owned(),
            author: "budi".to_owned(),
        },
        &world,
        &world,
        &world,
        4_242,
    )
    .unwrap();

    let input = ReadHistoryInput { table, limit: 1 };
    let entries = read_history(&input, &world, &world).unwrap();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id, recorded.id);
    assert_eq!(entries[0].author, "budi");
    assert_eq!(entries[0].summary, "feat: satu");
    assert_eq!(entries[0].timestamp_unix_ms, 4_242);
}
