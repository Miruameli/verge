//! File: `branch_switch_tests.rs`
//!
//! Deskripsi: Test perpindahan branch aktif.
//! Layer: application/version-control/use-cases/refs/branching/tests
//! Tanggung jawab: Membuktikan efek `switch` terhadap `HEAD`.
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
use crate::application::version_control::use_cases::refs::branching::switch_branch::{
    switch_branch, SwitchBranchInput,
};
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn switch_branch_menandai_branch_baru_sebagai_aktif() {
    let (world, commit) = world_with_commit();
    create(&world, "eksperimen").expect("branch dibuat");

    let switched = switch_branch(
        &SwitchBranchInput {
            name: "eksperimen".to_owned(),
        },
        &world,
    )
    .expect("branch dialihkan");

    assert_eq!(switched.previous, "main");
    assert_eq!(switched.current, "eksperimen");
    assert_eq!(world.head_branch().unwrap(), "eksperimen");
    assert_eq!(world.resolve("eksperimen").unwrap().unwrap(), commit);
}

#[test]
fn switch_ke_branch_yang_tidak_ada_ditolak() {
    let (world, _) = world_with_commit();

    let error = switch_branch(
        &SwitchBranchInput {
            name: "hantu".to_owned(),
        },
        &world,
    )
    .expect_err("ditolak");

    assert!(matches!(error, VergeError::UnknownBranch(name) if name == "hantu"));
    assert_eq!(
        world.head_branch().unwrap(),
        "main",
        "HEAD tetap di branch lama"
    );
}

#[test]
fn switch_ke_branch_aktif_sendiri_tidak_mengubah_apa_pun() {
    let (world, _) = world_with_commit();

    let switched = switch_branch(
        &SwitchBranchInput {
            name: "main".to_owned(),
        },
        &world,
    )
    .expect("switch idempoten");

    assert_eq!(switched.previous, "main");
    assert_eq!(switched.current, "main");
}
