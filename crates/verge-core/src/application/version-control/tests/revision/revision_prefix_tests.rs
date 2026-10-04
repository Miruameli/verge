//! File: `revision_prefix_tests.rs`
//!
//! Deskripsi: Test resolusi awalan hex dan penolakan revisi lintas tabel.
//! Layer: application/version-control/tests/revision
//! Tanggung jawab: Membuktikan aturan awalan 12 hex seperti yang dicetak log.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
use super::super::fixtures::revision_fixtures::users_table;
use super::super::fixtures::revision_fixtures::world_with_commits;
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

#[test]
fn awalan_hex_yang_tidak_ada_ditolak() {
    let (world, _) = world_with_commits(2);

    let error = resolve_revision("0123456789ab", &world, &world, &world, Some(&users_table()))
        .expect_err("awalan asing harus ditolak");

    assert!(matches!(error, VergeError::InvalidRef(_)));
}

#[test]
fn awalan_terlalu_pendek_dianggap_nama_branch_ditolak_bukan_diterka() {
    let (world, _) = world_with_commits(1);

    assert!(
        resolve_revision("abc", &world, &world, &world, Some(&users_table())).is_err(),
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
        &world,
    )
    .expect("snapshot terbaca");

    assert_eq!(snapshot.bytes, b"id,name\n0,user-0\n");
    assert_eq!(snapshot.commit.to_hex(), ids[0]);
}
