//! File: `read_snapshot_tests.rs`
//!
//! Deskripsi: Test use case `read_snapshot`.
//! Layer: application/version-control/use-cases/read-snapshot
//! Tanggung jawab: Membuktikan pembacaan isi tabel pada revisi lama.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `read_snapshot.rs`
//! Related issues: #8 (Milestone 2)
//! Related ADR: ADR-0005 (Tabel sebagai blok content-addressed)

use crate::application::version_control::dtos::recorded_commit::RecordedCommit;
use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::use_cases::read_history::{
    read_history, ReadHistoryInput,
};
use crate::application::version_control::use_cases::read_snapshot::{
    read_snapshot, ReadSnapshotInput,
};
use crate::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;

/// Membuat commit tabel dan mengembalikan identitasnya.
fn commit(world: &FakeWorld, table: &str, data: &[u8], message: &str, time: i64) -> RecordedCommit {
    let name = TableName::parse(table).unwrap();
    world.stage(&name, data);
    let input = RecordCommitInput {
        table: name,
        message: message.to_owned(),
        author: "ana".to_owned(),
    };
    record_commit(&input, world, world, world, time).expect("commit untuk test")
}

/// Membaca isi tabel pada revisi tertentu.
fn snapshot(world: &FakeWorld, table: &str, revision: &str) -> Vec<u8> {
    let input = ReadSnapshotInput {
        table: TableName::parse(table).unwrap(),
        revision: revision.to_owned(),
    };
    read_snapshot(&input, world, world, world)
        .expect("snapshot terbaca")
        .bytes
}

#[test]
fn isi_pada_commit_lama_tetap_terbaca_setelah_commit_baru() {
    let world = FakeWorld::new();
    let lama = commit(&world, "users", b"id\n1,ana\n", "feat: satu", 1_000);
    commit(&world, "users", b"id\n1,ana\n2,budi\n", "feat: dua", 2_000);

    assert_eq!(snapshot(&world, "users", "HEAD"), b"id\n1,ana\n2,budi\n");
    assert_eq!(
        snapshot(&world, "users", &lama.id.to_string()),
        b"id\n1,ana\n"
    );
    assert_eq!(snapshot(&world, "users", "main"), b"id\n1,ana\n2,budi\n");
}

#[test]
fn revisi_commit_hex_membaca_data_yang_tepat_bukan_yang_terbaru() {
    let world = FakeWorld::new();
    let pertama = commit(&world, "users", b"id\n1\n", "feat: satu", 1_000);
    let kedua = commit(&world, "users", b"id\n1\n2\n", "feat: dua", 2_000);

    let input = ReadSnapshotInput {
        table: TableName::parse("users").unwrap(),
        revision: kedua.id.to_string(),
    };
    let content = read_snapshot(&input, &world, &world, &world).unwrap();
    let history = read_history(
        &ReadHistoryInput {
            table: TableName::parse("users").unwrap(),
            limit: 2,
        },
        &world,
        &world,
    )
    .unwrap();

    assert_eq!(content.commit, kedua.id);
    assert_ne!(content.commit, pertama.id);
    assert_eq!(history[0].id, kedua.id);
    assert_eq!(content.bytes, b"id\n1\n2\n");
}

#[test]
fn tabel_lain_pada_commit_yang_sama_ditolak() {
    let world = FakeWorld::new();
    let users = commit(&world, "users", b"id\n1\n", "feat: users", 1_000);

    let error = read_snapshot(
        &ReadSnapshotInput {
            table: TableName::parse("orders").unwrap(),
            revision: users.id.to_string(),
        },
        &world,
        &world,
        &world,
    )
    .expect_err("tabel yang berbeda harus ditolak");

    assert!(matches!(error, VergeError::InvalidRef(_)));
}

#[test]
fn revisi_yang_tidak_dikenal_ditolak_dengan_pesan_jelas() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id\n1\n", "feat: satu", 1_000);

    let error = read_snapshot(
        &ReadSnapshotInput {
            table: TableName::parse("users").unwrap(),
            revision: "release-2026".to_owned(),
        },
        &world,
        &world,
        &world,
    )
    .expect_err("revisi tak dikenal harus ditolak");

    assert!(matches!(error, VergeError::InvalidRef(text) if text == "release-2026"));
}

#[test]
fn branch_tanpa_commit_menolak_pembacaan_head() {
    let world = FakeWorld::new();
    let error = read_snapshot(
        &ReadSnapshotInput {
            table: TableName::parse("users").unwrap(),
            revision: "HEAD".to_owned(),
        },
        &world,
        &world,
        &world,
    )
    .expect_err("branch kosong harus ditolak");

    assert!(matches!(error, VergeError::HeadUnborn(branch) if branch == "main"));
}
