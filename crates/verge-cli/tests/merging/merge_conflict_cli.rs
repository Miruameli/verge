//! File: `merge_conflict_cli.rs`
//!
//! Deskripsi: Test end-to-end merge berkonflik.
//! Layer: interfaces/cli/tests
//! Tanggung jawab: Membuktikan penolakan konflik dan strategi resolusi.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `interfaces/cli/tests/support/mod.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use std::path::PathBuf;

use super::support::{head_id, stage_and_commit, staged_repository, verge_error, verge_stdout};

/// Membangun repository dengan dua branch yang masing-masing mengubah satu baris.
pub(super) fn diverged(name: &str) -> PathBuf {
    let dir = staged_repository(name, "id,name\n1,ana\n2,budi\n");
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n", "feat: seed");
    verge_stdout(&dir, &["branch", "create", "eksperimen"]);
    verge_stdout(&dir, &["branch", "switch", "eksperimen"]);
    stage_and_commit(
        &dir,
        "id,name\n1,ana\n2,budi\n3,citra\n",
        "feat: eksperimen",
    );
    verge_stdout(&dir, &["branch", "switch", "main"]);
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n4,dimas\n", "feat: main");
    dir
}

/// Membangun repository dengan dua branch yang mengubah baris yang sama.
fn conflicting(name: &str) -> PathBuf {
    let dir = staged_repository(name, "id,name\n1,ana\n");
    stage_and_commit(&dir, "id,name\n1,ana\n", "feat: seed");
    verge_stdout(&dir, &["branch", "create", "kiri"]);
    verge_stdout(&dir, &["branch", "create", "kanan"]);
    verge_stdout(&dir, &["branch", "switch", "kiri"]);
    stage_and_commit(&dir, "id,name\n1,ANA\n", "feat: kiri");
    verge_stdout(&dir, &["branch", "switch", "kanan"]);
    stage_and_commit(&dir, "id,name\n1,BUDI\n", "feat: kanan");
    verge_stdout(&dir, &["branch", "switch", "kiri"]);
    dir
}
#[test]
fn konflik_menahan_merge_dan_mencetak_ketiga_sisi() {
    let dir = conflicting("merge-conflict");
    let before = head_id(&dir);

    let error = verge_error(
        &dir,
        &["merge", "kanan", "--table", "users", "--author", "ana"],
    );

    assert!(error.contains("merge cancelled"), "{error}");
    assert!(error.contains(",ANA"), "sisi ours: {error}");
    assert!(error.contains(",BUDI"), "sisi theirs: {error}");
    assert_eq!(head_id(&dir), before, "merge batal tidak boleh menulis");
    let table = verge_stdout(&dir, &["show", "HEAD", "--table", "users"]);
    assert!(table.contains("1,ANA"), "isi branch aktif utuh: {table}");
}

#[test]
fn strategi_ours_mempertahankan_nilai_branch_aktif() {
    let dir = conflicting("merge-ours");

    verge_stdout(
        &dir,
        &[
            "merge",
            "kanan",
            "--table",
            "users",
            "--strategy",
            "ours",
            "--author",
            "ana",
        ],
    );
    let table = verge_stdout(&dir, &["show", "HEAD", "--table", "users"]);

    assert!(table.contains("1,ANA"), "{table}");
}

#[test]
fn strategi_theirs_mengambil_nilai_branch_yang_digabung() {
    let dir = conflicting("merge-theirs");

    verge_stdout(
        &dir,
        &[
            "merge",
            "kanan",
            "--table",
            "users",
            "--strategy",
            "theirs",
            "--author",
            "ana",
        ],
    );
    let table = verge_stdout(&dir, &["show", "HEAD", "--table", "users"]);

    assert!(table.contains("1,BUDI"), "{table}");
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
