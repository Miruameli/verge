//! File: `table_diff.rs`
//!
//! Deskripsi: Test integrasi diff baris di atas prolly tree.
//! Layer: tests
//! Tanggung jawab: Membuktikan perubahan baris antar commit di disk nyata.
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
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

mod support;

use std::path::PathBuf;

use support::flow::{diff, import_and_commit, snapshot, try_diff, users};
use support::{cleanup, harness, workspace, write_source, Harness};

use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::domain::tree::diff::row_change::RowChange;
use verge_core::VergeError;

/// Dua commit tabel `users`; mengembalikan harness, folder kerja, dan id awal.
fn two_commits(label: &str) -> (Harness, PathBuf, String) {
    let dir = workspace(label);
    let harness = harness(&dir);
    let seed = write_source(&dir, "satu.csv", b"id,name\n1,ana\n2,budi\n");
    let first = import_and_commit(&harness, &seed, "feat: seed users", "ana", 1_000);
    let next = write_source(&dir, "dua.csv", b"id,name\n1,ana\n2,sari\n3,caca\n");
    let second = import_and_commit(&harness, &next, "feat: refresh users", "ana", 2_000);
    assert_ne!(first, second, "dua commit harus berbeda");
    (harness, dir, first.to_string())
}

#[test]
fn diff_antar_commit_menampilkan_baris_yang_diubah_dan_ditambah() {
    let (harness, dir, first) = two_commits("diff");

    let report = diff(&harness, &first, "HEAD");

    assert_eq!(report.from.to_string(), first);
    assert_eq!(report.len(), 2);
    assert_eq!(
        report.changes,
        vec![
            RowChange::Modified {
                key: "2".to_owned(),
                before: b",budi".to_vec(),
                after: b",sari".to_vec(),
            },
            RowChange::Added {
                key: "3".to_owned(),
                value: b",caca".to_vec(),
            },
        ]
    );
    assert_eq!(
        snapshot(&harness, &first),
        b"id,name\n1,ana\n2,budi\n",
        "isi commit lama tetap terbaca setelah commit baru"
    );
    cleanup(&dir);
}

#[test]
fn diff_terhadap_revisi_yang_sama_tidak_perubahan() {
    let (harness, dir, first) = two_commits("diff-sama");

    assert!(diff(&harness, &first, &first).is_empty());
    cleanup(&dir);
}

#[test]
fn diff_menolak_revisi_dan_tabel_yang_tidak_dikenal() {
    let (harness, dir, _) = two_commits("diff-revisi");
    let unknown = try_diff(&harness, &users(), "v1.0", "HEAD").expect_err("revisi tak dikenal");
    assert!(
        matches!(unknown, VergeError::InvalidRef(_)),
        "error yang diharapkan: invalid reference, didapat: {unknown:?}"
    );
    let orders = TableName::parse("orders").expect("nama tabel valid");
    let salah = try_diff(&harness, &orders, "HEAD", "HEAD").expect_err("tabel tak ada di commit");
    assert!(
        matches!(
            salah,
            VergeError::CommitBelongsToOtherTable { ref requested, ref commit_table, .. }
                if requested.to_string() == "orders" && commit_table.to_string() == "users"
        ),
        "error yang diharapkan: commit milik tabel lain, didapat: {salah:?}"
    );
    cleanup(&dir);
}
