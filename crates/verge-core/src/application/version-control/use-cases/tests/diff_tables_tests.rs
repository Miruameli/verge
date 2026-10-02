//! File: `diff_tables_tests.rs`
//!
//! Deskripsi: Test use case `diff_tables`.
//! Layer: application/version-control/use-cases/tests
//! Tanggung jawab: Membuktikan tambah, ubah, hapus, dan revisi tak dikenal.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `diff_tables.rs`
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::application::version_control::fakes::world::FakeWorld;
use crate::domain::tree::diff::row_change::RowChange;
use crate::shared::exceptions::verge_error::VergeError;

use super::{commit, diff, report};

#[test]
fn baris_baru_ditambahkan_dilaporkan_sebagai_tambahan() {
    let world = FakeWorld::new();
    let lama = commit(&world, "users", b"id\n1,ana\n", "feat: satu", 1_000);
    let baru = commit(&world, "users", b"id\n1,ana\n2,budi\n", "feat: dua", 2_000);

    let found = report(&world, &lama, &baru);

    assert_eq!(found.len(), 1);
    assert!(!found.is_empty());
    assert_eq!(
        found.changes[0],
        RowChange::Added {
            key: "2".to_owned(),
            value: b",budi".to_vec(),
        }
    );
}

#[test]
fn nilai_baris_yang_berubah_dilaporan_sebagai_perubahan() {
    let world = FakeWorld::new();
    let lama = commit(&world, "users", b"id,nama\n1,ana\n", "feat: satu", 1_000);
    let baru = commit(&world, "users", b"id,nama\n1,sari\n", "feat: dua", 2_000);

    let found = report(&world, &lama, &baru);

    assert_eq!(found.changes[0].key(), "1");
    assert_eq!(
        found.changes[0],
        RowChange::Modified {
            key: "1".to_owned(),
            before: b",ana".to_vec(),
            after: b",sari".to_vec(),
        }
    );
}

#[test]
fn baris_yang_dihapus_dilaporkan_sebagai_penghapusan() {
    let world = FakeWorld::new();
    let lama = commit(&world, "users", b"id\n1,ana\n2,budi\n", "feat: satu", 1_000);
    let baru = commit(&world, "users", b"id\n1,ana\n", "feat: dua", 2_000);

    let found = report(&world, &lama, &baru);

    assert_eq!(
        found.changes,
        [RowChange::Removed {
            key: "2".to_owned(),
            value: b",budi".to_vec(),
        }]
    );
}

#[test]
fn tabel_identik_menghasilkan_laporan_kosong() {
    let world = FakeWorld::new();
    let satu = commit(&world, "users", b"id,nama\n1,ana\n", "feat: satu", 1_000);
    let dua = commit(
        &world,
        "users",
        b"id,nama\n1,ana\n2,budi\n",
        "feat: dua",
        2_000,
    );

    assert_eq!(report(&world, &satu, &dua).len(), 1);

    let found = diff(&world, &satu.id.to_string(), &satu.id.to_string()).unwrap();

    assert!(found.is_empty());
    assert_eq!(found.len(), 0);
}

#[test]
fn perubahan_campuran_terurut_menurut_kunci_baris() {
    let world = FakeWorld::new();
    let lama = commit(
        &world,
        "users",
        b"id\n1,ana\n2,budi\n3,sari\n",
        "satu",
        1_000,
    );
    let baru = commit(
        &world,
        "users",
        b"id\n1,raha\n3,sari\n4,tegar\n",
        "dua",
        2_000,
    );

    let found = report(&world, &lama, &baru);
    let kunci: Vec<&str> = found.changes.iter().map(RowChange::key).collect();

    assert_eq!(found.len(), 3);
    assert_eq!(kunci, ["1", "2", "4"]);
}

#[test]
fn revisi_tak_dikenal_ditolak_dengan_pesan_jelas() {
    let world = FakeWorld::new();
    commit(&world, "users", b"id\n1,ana\n", "feat: satu", 1_000);

    let error = diff(&world, "HEAD", "release-2026").expect_err("revisi harus ditolak");

    assert!(matches!(error, VergeError::InvalidRef(text) if text == "release-2026"));
}

#[test]
fn branch_tanpa_commit_menolak_perbandingan_head() {
    let world = FakeWorld::new();

    let error = diff(&world, "HEAD", "HEAD").expect_err("branch kosong harus ditolak");

    assert!(matches!(error, VergeError::HeadUnborn(branch) if branch == "main"));
}
