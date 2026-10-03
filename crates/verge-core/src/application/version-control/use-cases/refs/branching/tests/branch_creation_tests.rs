//! File: `branch_creation_tests.rs`
//!
//! Deskripsi: Test pembuatan branch.
//! Layer: application/version-control/use-cases/refs/branching/tests
//! Tanggung jawab: Membuktikan aturan nama dan titik awal branch baru.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use super::fixtures::{create, world_with_commit};
use crate::application::version_control::fakes::world::FakeWorld;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn branch_baru_menunjuk_commit_terakhir_branch_aktif() {
    let (world, commit) = world_with_commit();

    let created = create(&world, "eksperimen").expect("branch dibuat");

    assert_eq!(created.name, "eksperimen");
    assert_eq!(created.at, commit.to_hex());
    assert_eq!(
        world.head_branch().expect("baca HEAD"),
        "main",
        "HEAD tidak bergeser"
    );
}

#[test]
fn branch_baru_tidak_menyalin_blok_data() {
    let (world, _) = world_with_commit();

    create(&world, "eksperimen").expect("branch dibuat");

    assert_eq!(
        world.resolve("eksperimen").expect("resolve"),
        world.resolve("main").expect("resolve"),
        "kedua branch menunjuk commit yang sama"
    );
}

#[test]
fn branch_yang_sudah_ada_ditolak() {
    let (world, _) = world_with_commit();
    create(&world, "eksperimen").expect("branch pertama");

    let error = create(&world, "eksperimen").expect_err("branch kedua harus ditolak");

    assert!(matches!(error, VergeError::BranchAlreadyExists(name) if name == "eksperimen"));
}

#[test]
fn nama_branch_tidak_aman_ditolak_sebelum_menulis_pointer() {
    let (world, _) = world_with_commit();

    let error = create(&world, "../melolos").expect_err("nama harus ditolak");

    assert!(matches!(error, VergeError::InvalidName(_)));
    assert!(world
        .branches()
        .expect("daftar")
        .contains(&"main".to_owned()));
}

#[test]
fn nama_device_tercadang_ditolak_agar_portable_antar_platform() {
    let (world, _) = world_with_commit();

    let error = create(&world, "NUL").expect_err("nama device harus ditolak");

    assert!(matches!(error, VergeError::InvalidName(_)));
}

#[test]
fn branch_dibuat_di_atas_branch_aktif_yang_bukan_default() {
    let (world, _) = world_with_commit();
    world
        .advance("eksperimen", Digest::of(b"commit-eksperimen"))
        .expect("pointer eksperimen");
    world.switch("eksperimen").expect("pindah");

    let created = create(&world, "kedua").expect("branch dibuat");

    assert_eq!(created.at, Digest::of(b"commit-eksperimen").to_hex());
}

#[test]
fn branch_tidak_bisa_dibuat_saat_branch_aktif_belum_punya_commit() {
    let world = FakeWorld::new();

    let error = create(&world, "eksperimen").expect_err("tidak ada commit awal");

    assert!(matches!(error, VergeError::HeadUnborn(name) if name == "main"));
}
