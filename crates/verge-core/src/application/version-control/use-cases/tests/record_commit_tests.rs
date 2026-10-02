//! File: `record_commit_tests.rs`
//!
//! Deskripsi: Test use case `record_commit`.
//! Layer: application/version-control/use-cases/record-commit
//! Tanggung jawab: Membuktikan rantai commit, validasi metadata, dan dedup.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `record_commit.rs`
//! Related issues: #8 (Milestone 2)
//! Related ADR: ADR-0005 (Tabel sebagai blok content-addressed)

use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;

/// Membangun masukan commit dengan pesan dan penulis bawaan.
fn input(table: &str, message: &str) -> RecordCommitInput {
    RecordCommitInput {
        table: TableName::parse(table).unwrap(),
        message: message.to_owned(),
        author: "ana".to_owned(),
    }
}

#[test]
fn commit_pertama_tidak_punya_parent_dan_menggerakkan_branch() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    world.stage(&table, b"id\n1\n");

    let recorded = record_commit(&input("users", "feat: seed"), &world, &world, &world, 1_000)
        .expect("commit pertama berhasil");

    assert!(recorded.created);
    assert_eq!(recorded.branch, "main");
    assert_eq!(world.resolve("main").unwrap(), Some(recorded.id));
    let commit = world.load(&recorded.id).unwrap();
    assert!(commit.parents().is_empty());
    assert_eq!(commit.tree(), recorded.tree);
    assert_eq!(commit.table(), &table);
    assert_eq!(commit.timestamp_unix_ms(), 1_000);
}

#[test]
fn commit_kedua_menaut_pada_commit_sebelumnya() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    world.stage(&table, b"id\n1\n");
    let first =
        record_commit(&input("users", "feat: seed"), &world, &world, &world, 1_000).unwrap();

    world.stage(&table, b"id\n1\n2,budi\n");
    let second = record_commit(
        &input("users", "feat: add budi"),
        &world,
        &world,
        &world,
        2_000,
    )
    .unwrap();

    let commit = world.load(&second.id).unwrap();
    assert_eq!(commit.parents(), [first.id]);
    assert_ne!(first.tree, second.tree);
    assert_eq!(commit.timestamp_unix_ms(), 2_000);
}

#[test]
fn isi_tabel_yang_tidak_berubah_ditolak_sebagai_commit_kosong() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    world.stage(&table, b"id\n1\n");
    record_commit(&input("users", "feat: seed"), &world, &world, &world, 1_000).unwrap();

    let error = record_commit(&input("users", "feat: seed"), &world, &world, &world, 2_000)
        .expect_err("commit kedua tanpa perubahan harus ditolak");

    assert!(matches!(error, VergeError::NothingToCommit(name) if name == "users"));
}

#[test]
fn tabel_yang_belum_di_stage_ditolak() {
    let world = FakeWorld::new();
    let error = record_commit(&input("users", "feat: seed"), &world, &world, &world, 1_000)
        .expect_err("tabel tanpa data kerja harus ditolak");
    assert!(matches!(error, VergeError::TableNotStaged(name) if name == "users"));
}

#[test]
fn branch_terpisah_mendapat_rantai_commit_sendiri() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    world.stage(&table, b"id\n1\n");
    let main = record_commit(&input("users", "feat: seed"), &world, &world, &world, 1_000).unwrap();

    world.set_head("eksperimen");
    world.stage(&table, b"id\n1\n2,budi\n");
    let side = record_commit(&input("users", "feat: side"), &world, &world, &world, 2_000).unwrap();

    assert_eq!(side.branch, "eksperimen");
    assert!(world.load(&side.id).unwrap().parents().is_empty());
    assert_eq!(world.resolve("main").unwrap(), Some(main.id));
}

#[test]
fn metadata_commit_tidak_valid_ditolak_sebelum_menyentuh_storage() {
    let world = FakeWorld::new();
    let table = TableName::parse("users").unwrap();
    world.stage(&table, b"id\n1\n");

    let mut kosong = input("users", "   ");
    kosong.author = "ana".to_owned();
    assert!(matches!(
        record_commit(&kosong, &world, &world, &world, 1_000),
        Err(VergeError::InvalidCommitField {
            field: "message",
            ..
        })
    ));

    let mut penulis = input("users", "feat: seed");
    penulis.author = "ana\ncurang".to_owned();
    assert!(matches!(
        record_commit(&penulis, &world, &world, &world, 1_000),
        Err(VergeError::InvalidCommitField {
            field: "author",
            ..
        })
    ));

    assert_eq!(
        world.resolve("main").unwrap(),
        None,
        "branch tidak boleh bergerak"
    );
}
