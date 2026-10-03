//! File: `query_fixtures.rs`
//!
//! Deskripsi: Fixture repository dengan tiga commit untuk test time-travel.
//! Layer: interfaces/cli/tests/time-travel
//! Tanggung jawab: Menyiapkan riwayat yang waktunya dapat dibaca dari `verge log`.
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

use std::path::{Path, PathBuf};

use super::support::{gap, stage_and_commit, staged_repository, verge_stdout};

/// Repository dengan tiga commit; mengembalikan foldernya.
///
/// JEDA antar commit wajib: presisi waktu commit adalah milidetik, sehingga
/// tiga commit dalam milidetik yang sama tidak dapat dibedakan lewat `AS OF`.
pub(super) fn timeline(name: &str) -> PathBuf {
    let dir = staged_repository(name, "id,name\n1,ana\n");
    stage_and_commit(&dir, "id,name\n1,ana\n", "feat: seed");
    gap();
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n", "feat: budi");
    gap();
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n3,citra\n", "feat: citra");
    dir
}

/// Waktu commit ke-`index` yang dicetak `verge log`, dihitung dari atas.
pub(super) fn commit_time(dir: &Path, index: usize) -> String {
    let log = verge_stdout(dir, &["log", "--table", "users", "--limit", "10"]);
    log.lines()
        .nth(index)
        .unwrap_or_else(|| panic!("baris ke-{index} harus ada pada log:\n{log}"))
        .split_whitespace()
        .nth(1)
        .unwrap_or_else(|| panic!("waktu commit tidak tercetak:\n{log}"))
        .to_owned()
}
