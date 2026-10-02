//! File: `merge_clean_cli.rs`
//!
//! Deskripsi: Test end-to-end merge tanpa konflik.
//! Layer: interfaces/cli/tests
//! Tanggung jawab: Membuktikan gabungan baris dari dua branch yang berbeda.
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
fn diverged(name: &str) -> PathBuf {
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

#[test]
fn merge_tanpa_konflik_menggabungkan_baris_kedua_sisi() {
    let dir = diverged("merge-clean");

    let output = verge_stdout(
        &dir,
        &["merge", "eksperimen", "--table", "users", "--author", "ana"],
    );
    let table = verge_stdout(&dir, &["show", "HEAD", "--table", "users"]);

    assert!(output.contains("merged eksperimen"), "{output}");
    assert!(table.contains("3,citra"), "baris eksperimen masuk: {table}");
    assert!(table.contains("4,dimas"), "baris main masuk: {table}");
}

#[test]
fn merge_menulis_commit_baru_yang_berbeda_dari_kedua_sisi() {
    let dir = diverged("merge-commit");
    let before = head_id(&dir);

    verge_stdout(
        &dir,
        &["merge", "eksperimen", "--table", "users", "--author", "ana"],
    );

    assert_ne!(head_id(&dir), before, "branch aktif harus bergerak");
    let log = verge_stdout(&dir, &["log", "--table", "users", "--limit", "1"]);
    assert!(log.contains("Merge branch `eksperimen`"), "{log}");
}

#[test]
fn merge_yang_sudah_terlanjur_ditolak_tanpa_menulis_commit() {
    let dir = diverged("merge-twice");
    verge_stdout(
        &dir,
        &["merge", "eksperimen", "--table", "users", "--author", "ana"],
    );
    let merged = head_id(&dir);

    let error = verge_error(
        &dir,
        &["merge", "eksperimen", "--table", "users", "--author", "ana"],
    );

    assert!(error.contains("already merged"), "{error}");
    assert_eq!(head_id(&dir), merged, "commit tidak boleh bertambah");
}
