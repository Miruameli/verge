//! File: `table_diff_cli.rs`
//!
//! Deskripsi: Test end-to-end `diff` lewat binary sungguhan.
//! Layer: tests (e2e)
//! Tanggung jawab: Membuktikan diff baris dan penolakan argumentasinya.
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
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

mod support;

use std::fs;
use std::path::PathBuf;

use support::{head_id, scratch, stage_and_commit, verge_error, verge_stdout};

/// Dua revisi tabel `users`; mengembalikan folder dan id revisi pertama.
fn two_revisions(name: &str) -> (PathBuf, String) {
    let dir = scratch(name);
    drop(verge_stdout(&dir, &["init"]));
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n", "seed users");
    let first = head_id(&dir);
    stage_and_commit(&dir, "id,name\n1,ana\n2,sari\n3,caca\n", "refresh users");
    (dir, first)
}

#[test]
fn diff_mencetak_baris_tambah_ubah_dan_hapus() {
    let (dir, first) = two_revisions("changes");
    let output = verge_stdout(
        &dir,
        &["diff", &format!("{first}..HEAD"), "--table", "users"],
    );
    assert_eq!(
        output.lines().collect::<Vec<_>>(),
        vec!["~ 2 ,budi -> ,sari", "+ 3 ,caca"],
        "baris yang tidak berubah tidak boleh dicetak: {output}"
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn diff_menerima_nama_branch_sebagai_revisi() {
    let (dir, _) = two_revisions("branches");
    let output = verge_stdout(&dir, &["diff", "main..main", "--table", "users"]);
    assert_eq!(output, "no changes\n");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn diff_untuk_tabel_yang_berbeda_menolak_tabel_lain() {
    let (dir, _) = two_revisions("wrong-table");
    let stderr = verge_error(&dir, &["diff", "main..main", "--table", "orders"]);
    assert!(stderr.contains("invalid reference"), "{stderr}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn diff_menolak_rentang_tanpa_pemisah() {
    let (dir, _) = two_revisions("no-range");
    let bare = verge_error(&dir, &["diff", "HEAD", "--table", "users"]);
    assert!(bare.contains("requires `..` between revisions"), "{bare}");
    let empty = verge_error(&dir, &["diff", "..HEAD", "--table", "users"]);
    assert!(empty.contains("both sides of `..`"), "{empty}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn diff_menolak_revisi_dan_tabel_yang_tidak_dikenal() {
    let (dir, _) = two_revisions("bad-input");
    let revision = verge_error(&dir, &["diff", "v1.0..HEAD", "--table", "users"]);
    assert!(revision.contains("invalid reference"), "{revision}");
    let table = verge_error(&dir, &["diff", "main..main", "--table", "Users"]);
    assert!(table.contains("invalid table name"), "{table}");
    let missing = verge_error(&dir, &["diff", "main..main"]);
    assert!(missing.contains("requires `--table <NAME>`"), "{missing}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn panduan_bantuan_mencantumkan_perintah_diff() {
    let dir = scratch("usage");
    let help = verge_stdout(&dir, &["--help"]);
    assert!(
        help.contains("verge diff <FROM>..<TO> --table <NAME>"),
        "{help}"
    );
    drop(fs::remove_dir_all(&dir));
}
