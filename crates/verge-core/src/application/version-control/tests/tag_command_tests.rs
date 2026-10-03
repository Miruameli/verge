//! File: `tag_command_tests.rs`
//!
//! Deskripsi: Test use case tag.
//! Layer: application/version-control/tests
//! Tanggung jawab: Membuktikan tag immutable, menolak nama tidak aman, dan
//!   daftar tag menampilkan commit yang ditunjuknya.
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

use super::fixtures::commit_timeline::{commit_at, step};
use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::application::version_control::use_cases::tagging::tag_command::{
    create_tag, delete_tag, list_tags,
};
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::ident::value_objects::digest_text::{parse_hex, HexText};
use crate::shared::exceptions::verge_error::VergeError;

/// Membangun dunia dengan satu commit tabel `users`.
fn one_commit() -> (FakeWorld, String) {
    let world = FakeWorld::new();
    let id = commit_at(&world, "id,name\n1,ana\n", step(0));
    (world, id.to_hex())
}

/// Mengubah hex fixture menjadi identifier commit.
fn commit_of(id: &str) -> Digest {
    parse_hex(id).expect("id fixture adalah hex")
}

#[test]
fn tag_baru_menunjuk_commit_yang_diminta() {
    let (world, id) = one_commit();

    create_tag("q3", "HEAD", &world, &world, &world).expect("tag dibuat");

    assert_eq!(
        world.resolve("q3").expect("tag terbaca"),
        Some(commit_of(&id))
    );
}

#[test]
fn tag_yang_sudah_ada_ditolak_dan_tidak_bergeser() {
    let (world, id) = one_commit();
    create_tag("q3", "HEAD", &world, &world, &world).expect("tag pertama");

    commit_at(&world, "id,name\n1,ana\n2,budi\n", step(1));
    let error = create_tag("q3", "HEAD", &world, &world, &world).expect_err("tag tidak ditimpa");

    assert!(matches!(error, VergeError::TagAlreadyExists(name) if name == "q3"));
    assert_eq!(
        world.resolve("q3").expect("tag terbaca"),
        Some(commit_of(&id)),
        "tag tetap menunjuk commit semula"
    );
}

#[test]
fn nama_tag_tidak_aman_ditolak_sebelum_menyentuh_disk() {
    let (world, _) = one_commit();

    for name in ["../keluar", "refs/heads/main", "trailing ", ".hidden", ""] {
        let error =
            create_tag(name, "HEAD", &world, &world, &world).expect_err("nama tidak aman ditolak");
        assert!(
            matches!(error, VergeError::InvalidName(_)),
            "`{name}` harus ditolak sebagai InvalidName"
        );
    }
    assert!(
        world.tags().expect("daftar tag").is_empty(),
        "tidak ada tag tertulis"
    );
}

#[test]
fn spasi_di_tengah_nama_diterima_karena_aman_di_semua_platform() {
    // Batas kebijakan adalah nama berkas lintas platform, bukan aversion ke
    // spasi: "laporan q2" adalah nama berkas sah di Linux, macOS, dan Windows.
    let (world, _) = one_commit();

    create_tag("laporan q2", "HEAD", &world, &world, &world).expect("spasi di tengah sah");

    assert_eq!(
        world.tags().expect("daftar tag"),
        vec!["laporan q2".to_owned()]
    );
}

#[test]
fn tag_ke_timestamp_ditolak_dengan_pesan_yang_menunjuk_jalan_keluar() {
    let (world, _) = one_commit();

    let error = create_tag("q3", "2026-10-01T10:00:00Z", &world, &world, &world)
        .expect_err("waktu bukan tujuan tag");

    assert!(matches!(error, VergeError::TagCannotPointAtInstant(_)));
    assert!(world.tags().expect("daftar tag").is_empty());
}

#[test]
fn tag_menunjuk_branch_dan_tag_lain_yang_sudah_ada() {
    let (world, id) = one_commit();
    create_tag("awal", "HEAD", &world, &world, &world).expect("tag awal");
    create_tag("salinan", "awal", &world, &world, &world).expect("tag kedua");

    assert_eq!(
        world.resolve("salinan").expect("tag terbaca"),
        Some(commit_of(&id))
    );
}

#[test]
fn menghapus_tag_tidak_menghapus_commit() {
    let (world, id) = one_commit();
    create_tag("q3", "HEAD", &world, &world, &world).expect("tag dibuat");

    delete_tag("q3", &world).expect("tag dihapus");

    assert_eq!(world.resolve("q3").expect("tag terbaca"), None);
    let still_there = resolve_revision(
        &id,
        &world,
        &world,
        &world,
        Some(&super::fixtures::commit_timeline::users()),
    )
    .expect("commit masih dapat dibaca");
    assert_eq!(still_there.to_hex(), id, "data tidak ikut terhapus");
}

#[test]
fn menghapus_tag_yang_tidak_ada_ditolak() {
    let (world, _) = one_commit();

    let error = delete_tag("tidak-ada", &world).expect_err("tag tidak ada");

    assert!(matches!(error, VergeError::UnknownTag(name) if name == "tidak-ada"));
}

#[test]
fn daftar_tag_menampilkan_nama_dan_commit_tertuju() {
    let (world, id) = one_commit();
    create_tag("q2", "HEAD", &world, &world, &world).expect("tag dibuat");

    let list =
        list_tags(&world.tags().expect("daftar tag"), &world, &world).expect("daftar terbaca");

    assert_eq!(list.entries.len(), 1);
    let (name, commit, summary) = &list.entries[0];
    assert_eq!(name, "q2");
    assert_eq!(commit.to_hex(), id);
    assert!(
        summary.contains("commit pada"),
        "ringkasan ikut tampil: {summary}"
    );
}
