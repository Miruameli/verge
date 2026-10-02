//! File: `revision_resolver_tests.rs`
//!
//! Deskripsi: Test penyelesaian nama revisi.
//! Layer: application/version-control
//! Tanggung jawab: Membuktikan `HEAD`, `HEAD~N`, awalan hex, dan branch.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `revision_resolver.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::application::version_control::use_cases::read_snapshot::{
    read_snapshot, ReadSnapshotInput,
};
use crate::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use crate::application::version_control::use_cases::stage_table::{stage_table, StageTableInput};
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;

/// Membangun dunia dengan `count` commit tabel `users`.
///
/// Setiap iterasi mengubah satu baris sehingga commit tidak pernah ditolak
/// sebagai "tidak ada perubahan".
fn world_with_commits(count: usize) -> (FakeWorld, Vec<String>) {
    let world = FakeWorld::new();
    let mut ids = Vec::new();
    for index in 0..count {
        let contents = format!("id,name\n{index},user-{index}\n");
        stage_table(
            &StageTableInput {
                table: TableName::parse("users").unwrap(),
                source: std::path::PathBuf::from("users.csv"),
            },
            &world.source_with(&contents),
            &world,
            &world,
        )
        .expect("stage berhasil");
        let recorded = record_commit(
            &RecordCommitInput {
                table: TableName::parse("users").unwrap(),
                message: format!("feat: tabel dengan {index} baris"),
                author: "ana".to_owned(),
            },
            &world,
            &world,
            &world,
            1_000 + i64::try_from(index).unwrap_or_default(),
        )
        .expect("commit berhasil");
        ids.push(recorded.id.to_hex());
    }
    (world, ids)
}

#[test]
fn head_menunjuk_commit_terakhir_branch_aktif() {
    let (world, ids) = world_with_commits(2);
    let resolved = resolve_revision("HEAD", &world, &world).expect("HEAD selesai");

    assert_eq!(resolved.to_hex(), ids[1]);
}

#[test]
fn head_dengan_langkah_kembali_menelusuri_rantai_first_parent() {
    let (world, ids) = world_with_commits(3);

    assert_eq!(
        resolve_revision("HEAD~1", &world, &world)
            .expect("satu langkah")
            .to_hex(),
        ids[1]
    );
    assert_eq!(
        resolve_revision("HEAD~2", &world, &world)
            .expect("dua langkah")
            .to_hex(),
        ids[0]
    );
}

#[test]
fn awalan_hex_yang_dicetak_log_dapat_dipakai_ulang() {
    let (world, ids) = world_with_commits(2);
    let printed = &ids[0][..12];

    let resolved = resolve_revision(printed, &world, &world).expect("awalan resolved");

    assert_eq!(resolved.to_hex(), ids[0]);
}

#[test]
fn identifier_penuh_dipakai_apa_adanya() {
    let (world, ids) = world_with_commits(1);

    assert_eq!(
        resolve_revision(&ids[0], &world, &world)
            .expect("id penuh")
            .to_hex(),
        ids[0]
    );
}

#[test]
fn nama_branch_dipakai_sebagai_revisi() {
    let (world, ids) = world_with_commits(1);

    assert_eq!(
        resolve_revision("main", &world, &world)
            .expect("branch main")
            .to_hex(),
        ids[0]
    );
}

#[test]
fn branch_kosong_menolak_head_dan_head_langkah_kembali() {
    let world = FakeWorld::new();

    assert!(matches!(
        resolve_revision("HEAD", &world, &world),
        Err(VergeError::HeadUnborn(_))
    ));
    assert!(matches!(
        resolve_revision("HEAD~1", &world, &world),
        Err(VergeError::HeadUnborn(_))
    ));
}

#[test]
fn langkah_kembali_terlalu_dalam_ditolak() {
    let (world, _) = world_with_commits(1);

    assert!(matches!(
        resolve_revision("HEAD~5", &world, &world),
        Err(VergeError::InvalidRef(_))
    ));
    assert!(
        resolve_revision("HEAD~x", &world, &world).is_err(),
        "jumlah langkah bukan angka harus ditolak"
    );
}

#[test]
fn awalan_hex_yang_tidak_ada_ditolak() {
    let (world, _) = world_with_commits(2);

    let error =
        resolve_revision("0123456789ab", &world, &world).expect_err("awalan asing harus ditolak");

    assert!(matches!(error, VergeError::InvalidRef(_)));
}

#[test]
fn awalan_terlalu_pendek_dianggap_nama_branch_ditolak_bukan_diterka() {
    let (world, _) = world_with_commits(1);

    assert!(
        resolve_revision("abc", &world, &world).is_err(),
        "awalan di bawah 12 hex tidak boleh diterka menjadi commit"
    );
}

#[test]
fn commit_tabel_lain_ditolak_saat_membaca_snapshot() {
    let (world, users_ids) = world_with_commits(1);
    let orders = TableName::parse("orders").unwrap();
    stage_table(
        &StageTableInput {
            table: orders.clone(),
            source: std::path::PathBuf::from("orders.csv"),
        },
        &world.source_with("id,name\n1,x\n"),
        &world,
        &world,
    )
    .expect("stage tabel lain");
    record_commit(
        &RecordCommitInput {
            table: orders.clone(),
            message: "feat: tabel lain".to_owned(),
            author: "ana".to_owned(),
        },
        &world,
        &world,
        &world,
        9_000,
    )
    .expect("commit tabel lain");

    let error = read_snapshot(
        &ReadSnapshotInput {
            table: orders,
            revision: users_ids[0].clone(),
        },
        &world,
        &world,
        &world,
    )
    .expect_err("commit tabel users tidak boleh dipakai untuk tabel orders");

    assert!(matches!(error, VergeError::InvalidRef(_)));
}

#[test]
fn snapshot_dapat_dibaca_lewat_awalan_hex() {
    let (world, ids) = world_with_commits(2);

    let snapshot = read_snapshot(
        &ReadSnapshotInput {
            table: TableName::parse("users").unwrap(),
            revision: ids[0][..12].to_owned(),
        },
        &world,
        &world,
        &world,
    )
    .expect("snapshot terbaca");

    assert_eq!(snapshot.bytes, b"id,name\n0,user-0\n");
    assert_eq!(snapshot.commit.to_hex(), ids[0]);
}
