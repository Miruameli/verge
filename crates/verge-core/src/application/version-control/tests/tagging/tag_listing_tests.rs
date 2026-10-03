//! File: `tag_listing_tests.rs`
//!
//! Deskripsi: Test daftar tag.
//! Layer: application/version-control/tests/tagging
//! Tanggung jawab: Membuktikan daftar tag memuat nama, commit yang ditunjuk,
//!   dan ringkasan commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `fixtures/tag_fixtures.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
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
