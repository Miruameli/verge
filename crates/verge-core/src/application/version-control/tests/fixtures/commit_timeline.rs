//! File: `commit_timeline.rs`
//!
//! Deskripsi: Fixture commit dengan waktu yang dapat dikendalikan.
//! Layer: application/version-control/tests/fixtures
//! Tanggung jawab: Menyusun riwayat dengan jarak waktu yang pasti supaya
//!   pengujian `AS OF` tidak bergantung pada jam sistem.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/fakes/world.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use crate::application::version_control::use_cases::stage_table::{stage_table, StageTableInput};
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::table::value_objects::table_name::TableName;

/// Jarak waktu antar commit; jauh lebih besar daripada presisi milidetik
/// sehingga batas `AS OF` tidak pernah jatuh pada dua commit sekaligus.
pub const STEP_MS: i64 = 5_000;

/// Tabel yang dipakai fixture ini.
#[must_use]
pub fn users() -> TableName {
    TableName::parse("users").unwrap()
}

/// Melakukan satu commit pada waktu `at_ms` lalu mengembalikan identifier-nya.
///
/// Isi tabel berubah setiap pemanggilan sehingga commit tidak ditolak sebagai
/// "tidak ada perubahan".
pub fn commit_at(world: &FakeWorld, contents: &str, at_ms: i64) -> CommitId {
    stage_table(
        &StageTableInput {
            table: users(),
            source: std::path::PathBuf::from("users.csv"),
        },
        &world.source_with(contents),
        &world,
        &world,
    )
    .expect("stage berhasil");
    record_commit(
        &RecordCommitInput {
            table: users(),
            message: format!("feat: commit pada {at_ms}"),
            author: "ana".to_owned(),
        },
        &world,
        &world,
        &world,
        at_ms,
    )
    .expect("commit berhasil")
    .id
}

/// Waktu commit pada langkah ke-`index`, dimulai dari [`STEP_MS`].
///
/// Returns:
/// - i64 — milidetik Unix yang dipakai fixture.
#[must_use]
pub fn step(index: usize) -> i64 {
    STEP_MS * (i64::try_from(index).unwrap_or(i64::MAX) + 1)
}
