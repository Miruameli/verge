//! File: `branch_deletion_tests.rs`
//!
//! Deskripsi: Test penghapusan branch.
//! Layer: application/version-control/use-cases/refs/branching/tests
//! Tanggung jawab: Membuktikan penghapusan pointer tanpa kehilangan data.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi eksternal)
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use super::fixtures::{create, world_with_commit};
use crate::application::version_control::use_cases::refs::branching::delete_branch::{
    delete_branch, DeleteBranchInput,
};
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn hapus_branch_tidak_menghapus_commit_yang_menunjuknya() {
    let (world, commit) = world_with_commit();
    create(&world, "eksperimen").expect("branch dibuat");

    let deleted = delete_branch(
        &DeleteBranchInput {
            name: "eksperimen".to_owned(),
        },
        &world,
    )
    .expect("branch dihapus");

    assert_eq!(deleted.was_at.map(|id| id.to_hex()), Some(commit.to_hex()));
    assert!(!world.branches().unwrap().contains(&"eksperimen".to_owned()));
    assert_eq!(world.resolve("main").unwrap().unwrap(), commit);
}

#[test]
fn hapus_branch_aktif_ditolak() {
    let (world, _) = world_with_commit();

    let error = delete_branch(
        &DeleteBranchInput {
            name: "main".to_owned(),
        },
        &world,
    )
    .expect_err("branch aktif tidak boleh dihapus");

    assert!(matches!(error, VergeError::BranchInUse(name) if name == "main"));
}

#[test]
fn hapus_branch_yang_tidak_ada_ditolak() {
    let (world, _) = world_with_commit();

    let error = delete_branch(
        &DeleteBranchInput {
            name: "hantu".to_owned(),
        },
        &world,
    )
    .expect_err("branch tidak ada");

    assert!(matches!(error, VergeError::UnknownBranch(name) if name == "hantu"));
}
