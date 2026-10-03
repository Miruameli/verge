//! File: `instant_table_filter_tests.rs`
//!
//! Deskripsi: Test `AS OF` pada riwayat yang mencampur beberapa tabel.
//! Layer: application/version-control/tests
//! Tanggung jawab: Membuktikan penelusuran waktu melewati commit tabel lain
//!   dan berhenti pada commit tabel yang ditanyakan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.3.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `fixtures/commit_timeline.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::fixtures::commit_timeline::{commit_table_at, orders, step, users};
use crate::application::version_control::fakes::world::FakeWorld;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::time::value_objects::timestamp::Timestamp;

/// Membangun riwayat: `users` pada langkah 0, `orders` pada langkah 1,
/// `users` lagi pada langkah 2, sehingga commit terakhir bukan `orders`.
fn mixed_history() -> (FakeWorld, String, String) {
    let world = FakeWorld::new();
    let users_first = commit_table_at(&world, &users(), "id,name\n1,ana\n", step(0));
    let orders_only = commit_table_at(&world, &orders(), "id,total\n1,10\n", step(1));
    commit_table_at(&world, &users(), "id,name\n1,ana\n2,budi\n", step(2));
    (world, users_first.to_hex(), orders_only.to_hex())
}

/// Menyelesaikan `--as-of` untuk tabel tertentu pada waktu `unix_ms`.
fn as_of(world: &FakeWorld, table: &TableName, unix_ms: i64) -> String {
    let when = Timestamp::from_unix_ms(unix_ms);
    resolve_revision(&when.to_rfc3339(), world, world, world, Some(table))
        .expect("waktu ada commit")
        .to_hex()
}

#[test]
fn tabel_yang_commit_terakhirnya_bukan_di_head_mengambil_commitnya_sendiri() {
    let (world, _, orders_commit) = mixed_history();

    let resolved = as_of(&world, &orders(), step(2));

    assert_eq!(
        resolved, orders_commit,
        "waktu setelah commit orders harus tetap menunjuk commit orders"
    );
}

#[test]
fn tabel_lain_tidak_menggantikan_commit_yang_lebih_lama_dengan_yang_baru() {
    let (world, users_first, _) = mixed_history();

    let resolved = as_of(&world, &users(), step(1));

    assert_eq!(
        resolved, users_first,
        "commit users baru berada setelah batas waktu sehingga tidak boleh terpilih"
    );
}
