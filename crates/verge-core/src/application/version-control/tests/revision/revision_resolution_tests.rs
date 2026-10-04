//! File: `revision_resolution_tests.rs`
//!
//! Deskripsi: Test resolusi `HEAD`, `HEAD~N`, dan nama branch.
//! Layer: application/version-control/tests/revision
//! Tanggung jawab: Membuktikan navigasi commit sebagai nama revisi.
//! Bentuk teks yang menyerupai waktu diuji di `revision_shape_tests.rs`.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-04
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `revision_resolver.rs`, `revision_target.rs`
//! Related issues: #18 (Milestone 3), #25 (Milestone 4)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel), ADR-0008 (AS OF dan tag)

use super::super::fixtures::revision_fixtures::{users_table, world_with_commits};
use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn head_menunjuk_commit_terakhir_branch_aktif() {
    let (world, ids) = world_with_commits(2);
    let resolved = resolve_revision("HEAD", &world, &world, &world, Some(&users_table()))
        .expect("HEAD selesai");

    assert_eq!(resolved.to_hex(), ids[1]);
}

#[test]
fn head_dengan_langkah_kembali_menelusuri_rantai_first_parent() {
    let (world, ids) = world_with_commits(3);

    assert_eq!(
        resolve_revision("HEAD~1", &world, &world, &world, Some(&users_table()))
            .expect("satu langkah")
            .to_hex(),
        ids[1]
    );
    assert_eq!(
        resolve_revision("HEAD~2", &world, &world, &world, Some(&users_table()))
            .expect("dua langkah")
            .to_hex(),
        ids[0]
    );
}

#[test]
fn awalan_hex_yang_dicetak_log_dapat_dipakai_ulang() {
    let (world, ids) = world_with_commits(2);
    let printed = &ids[0][..12];

    let resolved = resolve_revision(printed, &world, &world, &world, Some(&users_table()))
        .expect("awalan resolved");

    assert_eq!(resolved.to_hex(), ids[0]);
}

#[test]
fn identifier_penuh_dipakai_apa_adanya() {
    let (world, ids) = world_with_commits(1);

    assert_eq!(
        resolve_revision(&ids[0], &world, &world, &world, Some(&users_table()))
            .expect("id penuh")
            .to_hex(),
        ids[0]
    );
}

#[test]
fn nama_branch_dipakai_sebagai_revisi() {
    let (world, ids) = world_with_commits(1);

    assert_eq!(
        resolve_revision("main", &world, &world, &world, Some(&users_table()))
            .expect("branch main")
            .to_hex(),
        ids[0]
    );
}

#[test]
fn branch_kosong_menolak_head_dan_head_langkah_kembali() {
    let world = FakeWorld::new();

    assert!(matches!(
        resolve_revision("HEAD", &world, &world, &world, Some(&users_table())),
        Err(VergeError::HeadUnborn(_))
    ));
    assert!(matches!(
        resolve_revision("HEAD~1", &world, &world, &world, Some(&users_table())),
        Err(VergeError::HeadUnborn(_))
    ));
}

#[test]
fn langkah_kembali_terlalu_dalam_ditolak() {
    let (world, _) = world_with_commits(1);

    assert!(matches!(
        resolve_revision("HEAD~5", &world, &world, &world, Some(&users_table())),
        Err(VergeError::InvalidRef(_))
    ));
    assert!(
        resolve_revision("HEAD~x", &world, &world, &world, Some(&users_table())).is_err(),
        "jumlah langkah bukan angka harus ditolak"
    );
}
