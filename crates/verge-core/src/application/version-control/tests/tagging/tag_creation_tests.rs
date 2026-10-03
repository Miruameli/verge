//! File: `tag_creation_tests.rs`
//!
//! Deskripsi: Test pembuatan tag.
//! Layer: application/version-control/tests/tagging
//! Tanggung jawab: Membuktikan tag baru menunjuk commit yang diminta, tag yang
//!   sudah ada tidak ditimpa, nama tidak aman ditolak sebelum menyentuh disk,
//!   dan tujuan tag selain commit ditolak dengan pesan yang jelas.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `fixtures/commit_timeline.rs`
//!   - `fixtures/tag_fixtures.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::super::fixtures::commit_timeline::{commit_at, step};
use super::super::fixtures::tag_fixtures::{commit_of, one_commit};
use crate::application::version_control::use_cases::refs::tagging::tag_command::create_tag;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::shared::exceptions::verge_error::VergeError;

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
