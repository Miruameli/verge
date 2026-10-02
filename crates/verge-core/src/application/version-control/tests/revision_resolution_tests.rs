//! File: `revision_resolution_tests.rs`
//!
//! Deskripsi: Test resolusi `HEAD`, `HEAD~N`, dan nama branch.
//! Layer: application/version-control
//! Tanggung jawab: Membentukkan navigasi commit sebagai nama revisi.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `revision_resolver.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use super::fixtures::revision_fixtures::world_with_commits;
use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::shared::exceptions::verge_error::VergeError;

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
