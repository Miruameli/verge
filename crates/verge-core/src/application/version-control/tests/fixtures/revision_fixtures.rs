//! File: `revision_fixtures.rs`
//!
//! Deskripsi: Dunia ber-commit untuk test resolusi revisi.
//! Layer: application/version-control/tests/fixtures
//! Tanggung jawab: Menyiapkan repository dengan rantai commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::use_cases::record_commit::{
    record_commit, RecordCommitInput,
};
use crate::application::version_control::use_cases::stage_table::{stage_table, StageTableInput};
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::table::value_objects::table_name::TableName;

/// Tabel yang dipakai seluruh test resolusi revisi.
#[must_use]
pub fn users_table() -> TableName {
    TableName::parse("users").unwrap()
}

/// Membangun dunia dengan `count` commit tabel `users`.
///
/// Setiap iterasi mengubah satu baris sehingga commit tidak pernah ditolak
/// sebagai "tidak ada perubahan".
pub fn world_with_commits(count: usize) -> (FakeWorld, Vec<String>) {
    let world = FakeWorld::new();
    let mut ids = Vec::new();
    for index in 0..count {
        let contents = format!("id,name\n{index},user-{index}\n");
        stage_table(
            &StageTableInput {
                table: TableName::parse("users").unwrap(),
                source: std::path::PathBuf::from("users.csv"),
            },
            &world.source_with(&contents),
            &world,
            &world,
        )
        .expect("stage berhasil");
        let recorded = record_commit(
            &RecordCommitInput {
                table: TableName::parse("users").unwrap(),
                message: format!("feat: tabel dengan {index} baris"),
                author: "ana".to_owned(),
            },
            &world,
            &world,
            &world,
            1_000 + i64::try_from(index).unwrap_or_default(),
        )
        .expect("commit berhasil");
        ids.push(recorded.id.to_hex());
    }
    (world, ids)
}
