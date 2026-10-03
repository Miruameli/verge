//! File: `instant_lookup_tests.rs`
//!
//! Deskripsi: Test pemilihan commit pada waktu tertentu.
//! Layer: application/version-control/tests
//! Tanggung jawab: Membuktikan `AS OF <TIMESTAMP>` memilih commit yang benar
//!   pada rantai first-parent, termasuk setelah merge.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `fixtures/commit_timeline.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::fixtures::commit_timeline::{commit_at, step, users};
use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::application::version_control::use_cases::merging::merge_branch::{
    merge_branch, MergeBranchInput,
};
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::merge::merge_strategy::MergeStrategy;
use crate::domain::time::value_objects::timestamp::Timestamp;
use crate::shared::exceptions::verge_error::VergeError;

/// Menyelesaikan `--as-of` pada tabel `users` dengan port dunia in-memory.
fn as_of(world: &FakeWorld, unix_ms: i64) -> Result<String, VergeError> {
    let when = Timestamp::from_unix_ms(unix_ms);
    resolve_revision(&when.to_rfc3339(), world, world, world, Some(&users())).map(|id| id.to_hex())
}

#[test]
fn waktu_di_antara_dua_commit_memilih_yang_sebelumnya() {
    let world = FakeWorld::new();
    let first = commit_at(&world, "id,name\n1,ana\n", step(0));
    let second = commit_at(&world, "id,name\n1,ana\n2,budi\n", step(1));

    let resolved = as_of(&world, step(0) + 1).expect("waktu ada");

    assert_eq!(resolved, first.to_hex(), "commit pertama masih terpilih");
    assert_ne!(resolved, second.to_hex());
}

#[test]
fn waktu_yang_sama_dengan_sebuah_commit_memakai_commit_itu() {
    let world = FakeWorld::new();
    commit_at(&world, "id,name\n1,ana\n", step(0));
    let second = commit_at(&world, "id,name\n1,ana\n2,budi\n", step(1));

    assert_eq!(as_of(&world, step(1)).expect("waktu ada"), second.to_hex());
}

#[test]
fn waktu_setelah_commit_terakhir_memakai_commit_terakhir() {
    let world = FakeWorld::new();
    commit_at(&world, "id,name\n1,ana\n", step(0));
    let second = commit_at(&world, "id,name\n1,ana\n2,budi\n", step(1));

    assert_eq!(
        as_of(&world, step(1) * 100).expect("waktu ada"),
        second.to_hex()
    );
}

#[test]
fn waktu_sebelum_commit_pertama_ditolak_dan_menyebut_kom_tertua() {
    let world = FakeWorld::new();
    let first = commit_at(&world, "id,name\n1,ana\n", step(0));

    let error = as_of(&world, 1).expect_err("belum ada commit");

    let VergeError::NoCommitAtInstant { requested, oldest } = error else {
        panic!("galat harus NoCommitAtInstant");
    };
    assert_eq!(requested, "1970-01-01T00:00:00.001Z", "1 ms setelah epoch");
    assert_eq!(
        oldest,
        Timestamp::from_unix_ms(step(0)).to_rfc3339(),
        "pesan menyebut commit tertua yang tersedia"
    );
    let _ = first;
}

#[test]
fn commit_dari_branch_yang_sudah_digabung_tidak_dipilih_sebagai_waktu_lampau() {
    // ADR-0008: `AS OF` mengikuti rantai first-parent. Commit branch lain baru
    // menjadi bagian branch ini pada saat merge, jadi waktunya yang lebih lama
    // tidak boleh muncul sebagai "keadaan branch ini" pada waktu itu.
    let world = FakeWorld::new();
    let root = commit_at(&world, "id,name\n1,ana\n", step(0));

    world
        .advance("eksperimen", root)
        .expect("branch eksperimen dibuat");
    world.switch("eksperimen").expect("pindah ke eksperimen");
    commit_at(&world, "id,name\n1,ana\n2,budi\n", step(1));

    world.switch("main").expect("kembali ke main");
    commit_at(&world, "id,name\n1,ana\n3,citra\n", step(2));

    let merged = merge_branch(
        &MergeBranchInput {
            source: "eksperimen".to_owned(),
            table: users(),
            strategy: MergeStrategy::Manual,
            message: "Merge branch `eksperimen`".to_owned(),
            author: "ana".to_owned(),
            timestamp_unix_ms: step(3),
        },
        &world,
        &world,
        &world,
    )
    .expect("merge berhasil");

    assert_eq!(
        as_of(&world, step(1) + 1).expect("waktu ada"),
        root.to_hex(),
        "waktu lama harus menunjuk commit yang benar-benar aktif di branch ini"
    );
    assert_eq!(
        as_of(&world, step(3)).expect("waktu ada"),
        merged.commit.expect("merge menulis commit").to_hex()
    );
}
