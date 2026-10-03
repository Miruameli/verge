//! File: `tag_deletion_tests.rs`
//!
//! Deskripsi: Test penghapusan tag.
//! Layer: application/version-control/tests/tagging
//! Tanggung jawab: Membuktikan penghapusan tag hanya menghapus pointer dan
//!   menolak tag yang memang tidak ada.
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

use super::super::fixtures::commit_timeline::users;
use super::super::fixtures::tag_fixtures::one_commit;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::application::version_control::use_cases::refs::tagging::tag_command::{
    create_tag, delete_tag,
};
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn menghapus_tag_tidak_menghapus_commit() {
    let (world, id) = one_commit();
    create_tag("q3", "HEAD", &world, &world, &world).expect("tag dibuat");

    delete_tag("q3", &world).expect("tag dihapus");

    assert_eq!(world.resolve("q3").expect("tag terbaca"), None);
    let still_there = resolve_revision(&id, &world, &world, &world, Some(&users()))
        .expect("commit masih dapat dibaca");
    assert_eq!(still_there.to_hex(), id, "data tidak ikut terhapus");
}

#[test]
fn menghapus_tag_yang_tidak_ada_ditolak() {
    let (world, _) = one_commit();

    let error = delete_tag("tidak-ada", &world).expect_err("tag tidak ada");

    assert!(matches!(error, VergeError::UnknownTag(name) if name == "tidak-ada"));
}
