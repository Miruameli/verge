//! File: `tag_fixtures.rs`
//!
//! Deskripsi: Fixture repository ber-tag untuk test time-travel.
//! Layer: interfaces/cli/tests/time-travel
//! Tanggung jawab: Menyiapkan dua commit agar tag dapat diarahkan ke lampau.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `support/mod.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use std::path::PathBuf;

use super::support::{gap, stage_and_commit, staged_repository};

/// Repository dengan dua commit; mengembalikan foldernya.
///
/// Dua commit diperlukan agar `--revision HEAD~1` punya target sehingga tag
/// dapat diarahkan ke keadaan lampau.
pub(super) fn two_commits(name: &str) -> PathBuf {
    let dir = staged_repository(name, "id,name\n1,ana\n");
    stage_and_commit(&dir, "id,name\n1,ana\n", "feat: seed");
    gap();
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n", "feat: budi");
    dir
}
