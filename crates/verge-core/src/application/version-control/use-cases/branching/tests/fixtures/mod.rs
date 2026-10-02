//! File: `branch_fixtures.rs`
//!
//! Deskripsi: worlds dan helper bersama untuk test manipulasi branch.
//! Layer: application/version-control/use-cases/branching/tests
//! Tanggung jawab: Menyediakan dunia ber-commit tanpa mengulang penyiapan.
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

use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::use_cases::branching::create_branch::{
    create_branch, CreateBranchInput,
};
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::shared::kernel::result::Result;

/// Bentuk hasil `create_branch` agar assertion test tetap pendek.
#[derive(Debug)]
pub struct Created {
    /// Nama branch yang dibuat.
    pub name: String,
    /// Commit awal branch dalam hex.
    pub at: String,
}

/// Membangun dunia dengan satu commit pada branch `main`.
#[must_use]
pub fn world_with_commit() -> (FakeWorld, Digest) {
    let world = FakeWorld::new();
    world
        .advance("main", Digest::of(b"commit-pertama"))
        .expect("pointer awal");
    (world, Digest::of(b"commit-pertama"))
}

/// Memanggil `create_branch` dengan nama `name`.
pub fn create(world: &FakeWorld, name: &str) -> Result<Created> {
    create_branch(
        &CreateBranchInput {
            name: name.to_owned(),
        },
        world,
    )
    .map(|made| Created {
        name: made.name,
        at: made.at.to_hex(),
    })
}
