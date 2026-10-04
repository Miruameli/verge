//! File: `tag_listing_tests.rs`
//!
//! Deskripsi: Test daftar tag.
//! Layer: application/version-control/tests/tagging
//! Tanggung jawab: Membuktikan daftar tag memuat nama, commit yang ditunjuk,
//!   tabel commit tersebut, dan ringkasan commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `fixtures/tag_fixtures.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::super::fixtures::tag_fixtures::one_commit;
use crate::application::version_control::use_cases::refs::tagging::tag_command::{
    create_tag, list_tags,
};
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::ident::value_objects::digest_text::HexText;

#[test]
fn daftar_tag_menampilkan_nama_commit_dan_tabel_tertuju() {
    let (world, id) = one_commit();
    create_tag("q2", "HEAD", &world, &world, &world).expect("tag dibuat");

    let list =
        list_tags(&world.tags().expect("daftar tag"), &world, &world).expect("daftar terbaca");

    assert_eq!(list.entries.len(), 1);
    let entry = &list.entries[0];
    assert_eq!(entry.name, "q2");
    assert_eq!(entry.commit.to_hex(), id);
    assert_eq!(
        entry.table.to_string(),
        "users",
        "tabel ikut tampil agar tag tidak perlu ditebak: {}",
        entry.table
    );
    assert!(
        entry.summary.contains("commit pada"),
        "ringkasan ikut tampil: {}",
        entry.summary
    );
}
