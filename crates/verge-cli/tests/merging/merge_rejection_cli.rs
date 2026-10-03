//! File: `merge_rejection_cli.rs`
//!
//! Deskripsi: Test penolakan `verge merge`.
//! Layer: interfaces/cli/tests/merging
//! Tanggung jawab: Membuktikan merge yang tidak boleh terjadi berhenti tanpa
//!   menulis commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `../support/mod.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use super::merge_conflict_cli::diverged;
use super::support::{head_id, import_users, verge_error, verge_stdout};

#[test]
fn merge_dengan_tabel_lain_ditolak_bukan_menggabungkan_salah_tabel() {
    let dir = diverged("merge-wrong-table");

    let error = verge_error(
        &dir,
        &[
            "merge",
            "eksperimen",
            "--table",
            "orders",
            "--author",
            "ana",
        ],
    );

    assert!(error.contains("not for table `orders`"), "{error}");
}

#[test]
fn import_ulang_tidak_menimpa_merge_yang_sudah_tersimpan() {
    let dir = diverged("merge-stable");
    verge_stdout(
        &dir,
        &["merge", "eksperimen", "--table", "users", "--author", "ana"],
    );
    let merged = head_id(&dir);

    import_users(&dir, "id,name\n1,ana\n");

    assert_eq!(
        head_id(&dir),
        merged,
        "data kerja tidak boleh mengubah commit merge"
    );
}

#[test]
fn strategi_tidak_dikenal_ditolak_sebelum_membaca_repository() {
    let dir = diverged("merge-unknown-strategy");

    let error = verge_error(
        &dir,
        &[
            "merge",
            "eksperimen",
            "--table",
            "users",
            "--strategy",
            "acak",
            "--author",
            "ana",
        ],
    );

    assert!(error.contains("unknown merge strategy `acak`"), "{error}");
    assert!(
        error.contains("last-write-wins"),
        "daftar harus disebutkan: {error}"
    );
}

#[test]
fn merge_branch_tanpa_commit_ditolak() {
    let dir = diverged("merge-empty");
    verge_stdout(&dir, &["branch", "create", "kosong"]);

    let error = verge_error(
        &dir,
        &["merge", "kosong", "--table", "users", "--author", "ana"],
    );

    assert!(
        error.contains("branch `kosong` has no commits to merge"),
        "{error}"
    );
}

#[test]
fn merge_tanpa_tabel_ditolak_sebelum_membaca_repository() {
    let dir = diverged("merge-no-table");

    let error = verge_error(&dir, &["merge", "eksperimen", "--author", "ana"]);

    assert!(error.contains("requires `--table <NAME>`"), "{error}");
}
