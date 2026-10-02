//! File: `table_versioning_cli.rs`
//!
//! Deskripsi: Test end-to-end `import` dan `commit` lewat binary sungguhan.
//! Layer: tests (e2e)
//! Tanggung jawab: Membuktikan alur stage lalu commit beserta penolakannya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - binary verge, `tests/support/mod.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::fs;

mod support;

use support::{
    commit_args, commit_ok, import_users, scratch, staged_repository, verge_error, verge_stdout,
    verge_stdout_as,
};

#[test]
fn import_lalu_commit_mencetak_blok_dan_branch() {
    let dir = staged_repository("journey", "id,name\n1,ana\n");
    let first = commit_ok(&dir, "seed users");
    assert!(first.contains("created"), "{first}");
    assert!(first.contains("on main"), "{first}");

    import_users(&dir, "id,name\n1,ana\n2,budi\n");
    assert!(commit_ok(&dir, "add budi").contains("on main"));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn commit_menolak_tabel_yang_belum_di_import() {
    let dir = scratch("unstaged");
    drop(verge_stdout(&dir, &["init"]));
    let stderr = verge_error(&dir, &commit_args("m"));
    assert!(stderr.contains("no staged data"), "{stderr}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn commit_kedika_menolak_data_yang_tidak_berubah() {
    let dir = staged_repository("unchanged", "id,name\n1,ana\n");
    drop(commit_ok(&dir, "seed users"));
    let stderr = verge_error(&dir, &commit_args("seed users"));
    assert!(
        stderr.contains("unchanged since the last commit"),
        "{stderr}"
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn argumen_wajib_divalidasi_sebelum_dipakai() {
    let dir = scratch("no-table");
    drop(verge_stdout(&dir, &["init"]));
    let missing = verge_error(&dir, &["import", "users.csv"]);
    assert!(missing.contains("requires `--table <NAME>`"), "{missing}");
    let invalid = verge_error(&dir, &["commit", "--table", "Users", "--message", "m"]);
    assert!(invalid.contains("invalid table name"), "{invalid}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn commit_tanpa_author_menyebut_kedua_sumber() {
    let dir = staged_repository("no-author", "id,name\n1,ana\n");
    let bare = ["commit", "--table", "users", "--message", "m"];
    let stderr = verge_error(&dir, &bare);
    assert!(stderr.contains("--author"), "{stderr}");
    assert!(stderr.contains("VERGE_AUTHOR"), "{stderr}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn author_dari_environment_dipakai_saat_flag_tak_ada() {
    let dir = staged_repository("env-author", "id,name\n1,ana\n");
    let bare = ["commit", "--table", "users", "--message", "seed"];
    drop(verge_stdout_as(&dir, &bare, "budi"));
    let history = verge_stdout(&dir, &["log", "--table", "users"]);
    assert!(history.contains(" budi "), "{history}");
    drop(fs::remove_dir_all(&dir));
}
