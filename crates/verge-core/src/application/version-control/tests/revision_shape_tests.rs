//! File: `revision_shape_tests.rs`
//!
//! Deskripsi: Test pemilah bentuk teks revisi.
//! Layer: application/version-control/tests
//! Tanggung jawab: Membuktikan bentuk waktu hanya dialihkan ke parser waktu
//!   dan nama pointer berbentuk tanggal tidak salah ditolak.
//!
//! Author: Miruameli
//! Created: 2026-10-04
//! Modified: 2026-10-04
//! Version: 0.3.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `revision_target.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::fixtures::revision_fixtures::{users_table, world_with_commits};
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::application::version_control::revision_target::{self, RevisionTarget};
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::ident::value_objects::digest_text::{parse_hex, HexText};

#[test]
fn bentuk_waktu_dipilah_sebagai_instants() {
    for text in [
        "2026-10-01T10:00:00Z",
        "2026-10-01T10:00:00.123Z",
        "@1767225600000",
    ] {
        assert!(
            matches!(
                revision_target::classify(text),
                Ok(RevisionTarget::Instant(_))
            ),
            "`{text}` harus dipilah sebagai waktu"
        );
    }
}

#[test]
fn nama_tidak_berbentuk_tanggal_tetap_dipilah_sebagai_branch_atau_tag() {
    for text in ["2026-q1-report", "main", "rilis"] {
        assert!(
            matches!(
                revision_target::classify(text),
                Ok(RevisionTarget::Branch(_))
            ),
            "`{text}` harus dipilah sebagai nama pointer"
        );
    }
    assert!(matches!(
        revision_target::classify("refs/tags/q3"),
        Ok(RevisionTarget::Tag(_))
    ));
    assert!(matches!(
        revision_target::classify("refs/heads/main"),
        Ok(RevisionTarget::Branch(_))
    ));
}

#[test]
fn tanggal_tanpa_jam_ditolak_dengan_pesan_yang_menyebut_bentuk_benar() {
    let error = revision_target::classify("2026-10-01")
        .expect_err("tanggal tanpa jam bukan waktu yang sah");

    assert!(
        matches!(error, crate::VergeError::InvalidTimestamp(_)),
        "harus ditolak sebagai timestamp, bukan diteruskan ke storage"
    );
}

#[test]
fn branch_berbentuk_tanggal_tetap_resolve_sebagai_branch() {
    let (world, ids) = world_with_commits(1);
    world
        .advance("2026-q1-report", parse_hex(&ids[0]).expect("hex fixture"))
        .expect("branch bertanggal dibuat");

    let resolved = resolve_revision(
        "2026-q1-report",
        &world,
        &world,
        &world,
        Some(&users_table()),
    )
    .expect("branch bertanggal harus resolve");

    assert_eq!(resolved.to_hex(), ids[0]);
}
