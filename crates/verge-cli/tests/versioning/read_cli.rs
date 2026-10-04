//! File: `read_cli.rs`
//!
//! Deskripsi: Test end-to-end `log` dan `show` lewat binary sungguhan.
//! Layer: tests (e2e)
//! Tanggung jawab: Membuktikan pembacaan riwayat dan isi tabel per revisi.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - binary verge, `tests/support/mod.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #31 (Tabel tag pada pesan galat)
//!   - #36 (Test show lintas tabel dan dokumentasi field revision)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::fs;
use std::path::PathBuf;

use super::support::{
    head_id, scratch, stage_and_commit, stage_and_commit_table, verge_error, verge_stdout,
};

/// Repository dengan dua commit; mengembalikan folder dan id commit pertama.
fn versioned_repository(name: &str) -> (PathBuf, String) {
    let dir = scratch(name);
    drop(verge_stdout(&dir, &["init"]));
    stage_and_commit(&dir, "id,name\n1,ana\n", "seed users");
    let first_id = head_id(&dir);
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n", "add budi");
    (dir, first_id)
}

/// Repository dengan satu commit `users` lalu satu commit `orders`.
///
/// KENAPA dua tabel: `show` pada revisi harus menolak commit milik tabel lain,
/// dan itu hanya mungkin bila branch benar-benar memuat commit dua tabel.
fn two_table_repository(name: &str) -> PathBuf {
    let dir = scratch(name);
    drop(verge_stdout(&dir, &["init"]));
    stage_and_commit(&dir, "id,name\n1,ana\n", "seed users");
    stage_and_commit_table(&dir, "orders", "id,total\n1,900\n", "seed orders");
    dir
}

#[test]
fn log_mencetak_satu_baris_per_commit_terbaru_dulu() {
    let (dir, _) = versioned_repository("history");
    let history = verge_stdout(&dir, &["log", "--table", "users"]);
    let lines: Vec<&str> = history.lines().collect();
    assert_eq!(lines.len(), 2, "{history}");
    assert!(lines[0].contains("ana add budi"), "{history}");
    assert!(lines[1].contains("ana seed users"), "{history}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn log_membatasi_jumlah_entri_lewat_limit() {
    let (dir, _) = versioned_repository("limited");
    let history = verge_stdout(&dir, &["log", "--table", "users", "--limit", "1"]);
    assert_eq!(history.lines().count(), 1, "{history}");
    assert!(history.contains("add budi"), "{history}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn show_menulis_isi_tabel_pada_setiap_revisi() {
    let (dir, first_id) = versioned_repository("snapshot");
    let head = verge_stdout(&dir, &["show", "HEAD", "--table", "users"]);
    assert_eq!(head, "id,name\n1,ana\n2,budi\n\n");
    let initial = verge_stdout(&dir, &["show", &first_id, "--table", "users"]);
    assert_eq!(initial, "id,name\n1,ana\n\n");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn show_menolak_revisi_yang_tidak_dikenal() {
    let (dir, _) = versioned_repository("bad-revision");
    let stderr = verge_error(&dir, &["show", "v1.0", "--table", "users"]);
    assert!(stderr.contains("invalid reference"), "{stderr}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn show_menolak_revisi_yang_menunjuk_commit_tabel_lain() {
    let dir = two_table_repository("cross-table");
    let stderr = verge_error(&dir, &["show", "HEAD", "--table", "users"]);
    assert!(
        stderr.contains("points to table `orders`, not `users`"),
        "{stderr}"
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn log_menolak_limit_yang_bukan_angka_positif() {
    let (dir, _) = versioned_repository("bad-limit");
    let words = verge_error(&dir, &["log", "--table", "users", "--limit", "sepuluh"]);
    assert!(words.contains("positive number"), "{words}");
    let zero = verge_error(&dir, &["log", "--table", "users", "--limit", "0"]);
    assert!(zero.contains("positive number"), "{zero}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn log_menolak_folder_yang_bukan_repository() {
    let dir = scratch("bare");
    let stderr = verge_error(&dir, &["log", "--table", "users"]);
    assert!(stderr.contains("no Verge repository"), "{stderr}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn argumen_tak_dikenal_menampilkan_panduan_lengkap() {
    let dir = scratch("unknown");
    let misspelled = verge_error(&dir, &["log", "--tabel", "users"]);
    assert!(
        misspelled.contains("unknown option `--tabel`"),
        "{misspelled}"
    );
    let stderr = verge_error(&dir, &["checkout", "main"]);
    assert!(stderr.contains("unknown command"), "{stderr}");
    for command in ["init", "import", "commit", "log", "show", "diff"] {
        assert!(stderr.contains(&format!("verge {command}")), "{stderr}");
    }
    drop(fs::remove_dir_all(&dir));
}
