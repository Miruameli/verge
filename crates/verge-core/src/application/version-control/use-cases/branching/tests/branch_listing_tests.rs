//! File: `branch_listing_tests.rs`
//!
//! Deskripsi: Test pembacaan daftar branch.
//! Layer: application/version-control/use-cases/branching/tests
//! Tanggung jawab: Membuktikan isi dan urutan daftar branch.
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
use crate::application::version_control::use_cases::branching::delete_branch::{
    delete_branch, DeleteBranchInput,
};
use crate::application::version_control::use_cases::branching::list_branches::list_branches;

#[test]
fn daftar_branch_menandai_branch_aktif_dan_terurut() {
    let (world, _) = world_with_commit();
    create(&world, "zeta").expect("branch dibuat");
    create(&world, "alpha").expect("branch dibuat");

    let list = list_branches(&world).expect("branch terbaca");

    let names: Vec<&str> = list.branches.iter().map(|b| b.name.as_str()).collect();
    assert_eq!(names, vec!["alpha", "main", "zeta"]);
    assert_eq!(list.current, "main");
    assert!(list.branches.iter().any(|b| b.current && b.name == "main"));
    assert!(list.current_head().is_some());
}

#[test]
fn daftar_branch_tidak_mencantumkan_branch_yang_sudah_dihapus() {
    let (world, _) = world_with_commit();
    create(&world, "eksperimen").expect("branch dibuat");
    delete_branch(
        &DeleteBranchInput {
            name: "eksperimen".to_owned(),
        },
        &world,
    )
    .expect("branch dihapus");

    let list = list_branches(&world).expect("branch terbaca");

    let names: Vec<&str> = list.branches.iter().map(|b| b.name.as_str()).collect();
    assert_eq!(names, vec!["main"]);
}
