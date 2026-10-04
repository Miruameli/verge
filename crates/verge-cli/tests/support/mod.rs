//! File: `support/mod.rs`
//!
//! Deskripsi: Helper bersama untuk test end-to-end CLI.
//! Layer: tests (support)
//! Tanggung jawab: Menyediakan scratch dir dan runner binary `verge`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - binary verge
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

// Setiap test binary memakai subset helper yang berbeda.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

/// Direktori sementara yang unik untuk satu pengujian.
pub fn scratch(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let dir = std::env::temp_dir().join(format!("verge-e2e-{name}-{}-{nanos}", std::process::id()));
    drop(fs::remove_dir_all(&dir));
    fs::create_dir_all(&dir).expect("buat direktori sementara");
    dir
}

/// Menjalankan binary `verge`, memakai author bawaan bila `author` diisi.
fn run(dir: &Path, args: &[&str], author: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_verge"));
    command
        .args(args)
        .current_dir(dir)
        .env_remove("VERGE_AUTHOR");
    if let Some(author) = author {
        command.env("VERGE_AUTHOR", author);
    }
    command
        .output()
        .expect("binary verge harus dapat dijalankan")
}

/// Menjalankan perintah yang harus berhasil dan mengembalikan stdout-nya.
pub fn verge_stdout(dir: &Path, args: &[&str]) -> String {
    expect_ok(&run(dir, args, None))
}

/// Menjalankan perintah yang harus berhasil dengan `VERGE_AUTHOR` terisi.
pub fn verge_stdout_as(dir: &Path, args: &[&str], author: &str) -> String {
    expect_ok(&run(dir, args, Some(author)))
}

/// Menjalankan perintah yang harus gagal dan mengembalikan stderr-nya.
pub fn verge_error(dir: &Path, args: &[&str]) -> String {
    let output = run(dir, args, None);
    assert!(!output.status.success(), "`{args:?}` seharusnya gagal");
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Mengubah output perintah yang sukses menjadi stdout-nya.
fn expect_ok(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Argumen `verge commit` tabel `users` dengan pesan `message` dan author `ana`.
pub fn commit_args(message: &str) -> [&str; 7] {
    [
        "commit",
        "--table",
        "users",
        "--message",
        message,
        "--author",
        "ana",
    ]
}

/// Commit `message` sebagai `ana`; mengembalikan stdout bila berhasil.
pub fn commit_ok(dir: &Path, message: &str) -> String {
    verge_stdout(dir, &commit_args(message))
}

/// Mengimpor `seed` ke tabel `table` lewat berkas `data.csv`.
///
/// KENAPA nama berkasnya tetap: `import` memvalidasi nama berkas sebagai
/// segmen path, dan test tidak perlu membedakan nama sumber antar tabel.
pub fn import_table(dir: &Path, table: &str, seed: &str) {
    fs::write(dir.join("data.csv"), seed).expect("tulis data");
    drop(verge_stdout(dir, &["import", "data.csv", "--table", table]));
}

/// Commit `message` untuk tabel `table` sebagai `ana`.
pub fn commit_table_ok(dir: &Path, table: &str, message: &str) -> String {
    verge_stdout(
        dir,
        &[
            "commit",
            "--table",
            table,
            "--message",
            message,
            "--author",
            "ana",
        ],
    )
}

/// Mengimpor `seed` ke tabel `users`; dipakai fixture yang hanya butuh `users`.
pub fn import_users(dir: &Path, seed: &str) {
    import_table(dir, "users", seed);
}

/// Repository dengan `seed` yang sudah di-import ke tabel `users`.
pub fn staged_repository(name: &str, seed: &str) -> PathBuf {
    let dir = scratch(name);
    drop(verge_stdout(&dir, &["init"]));
    import_users(&dir, seed);
    dir
}

/// Mengimpor `seed` lalu me-commit-nya sebagai tabel `users`.
pub fn stage_and_commit(dir: &Path, seed: &str, message: &str) {
    import_users(dir, seed);
    drop(commit_ok(dir, message));
}

/// Mengimpor `seed` ke tabel `table` lalu me-commit-nya.
pub fn stage_and_commit_table(dir: &Path, table: &str, seed: &str, message: &str) {
    import_table(dir, table, seed);
    drop(commit_table_ok(dir, table, message));
}

/// Id commit yang ditunjuk branch `main` di repository `dir`.
pub fn head_id(dir: &Path) -> String {
    let pointer = dir.join(".verge/refs/heads/main");
    fs::read_to_string(pointer).expect("baca pointer branch")
}

/// Memberi jeda sehingga commit berikutnya mendapat milidetik berbeda.
///
/// KENAPA helper ini ada: presisi `timestamp_unix_ms` adalah milidetik, dan
/// beberapa commit dalam milidetik yang sama tidak dapat dibedakan `AS OF`.
/// Tanpa jeda, test time-travel akan lulus atau gagal tergantung kecepatan
/// mesin — persis jenis test flaky yang dilarang.
pub fn gap() {
    std::thread::sleep(std::time::Duration::from_millis(20));
}
