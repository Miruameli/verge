//! File: `init_repository_cli.rs`
//!
//! Deskripsi: Test end-to-end `verge init` lewat binary sungguhan.
//! Layer: tests (e2e)
//! Tanggung jawab: Membuktikan perilaku CLI yang dilihat pengguna.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - binary verge
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

/// Direktori sementara yang unik untuk satu pengujian.
fn scratch(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let dir = std::env::temp_dir().join(format!("verge-cli-{name}-{}-{nanos}", std::process::id()));
    drop(fs::remove_dir_all(&dir));
    fs::create_dir_all(&dir).expect("buat direktori sementara");
    dir
}

/// Menjalankan binary `verge` dengan argumen yang diberikan.
fn verge(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_verge"))
        .args(args)
        .output()
        .expect("binary verge harus dapat dijalankan")
}

#[test]
fn init_membuat_layout_repository() {
    let dir = scratch("layout");
    let output = verge(&["init", dir.to_str().expect("path utf-8")]);
    assert!(output.status.success(), "init harus sukses");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("initialised empty Verge repository"),
        "{stdout}"
    );

    let repo = dir.join(".verge");
    assert!(repo.join("objects").is_dir(), "object store harus ada");
    assert!(
        repo.join("refs/heads").is_dir(),
        "namespace branch harus ada"
    );
    assert!(repo.join("refs/tags").is_dir(), "namespace tag harus ada");
    assert_eq!(
        fs::read_to_string(repo.join("HEAD")).expect("baca HEAD"),
        "ref: refs/heads/main\n"
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn init_menolak_menimpa_repository_yang_ada() {
    let dir = scratch("existing");
    let path = dir.to_str().expect("path utf-8").to_owned();
    assert!(verge(&["init", &path]).status.success());
    let second = verge(&["init", &path]);
    assert!(!second.status.success(), "init kedua harus gagal");
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(stderr.contains("already exists"), "{stderr}");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn perintah_tak_dikenal_menampilkan_panduan() {
    let output = verge(&["frobnicate"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown command"), "{stderr}");
    assert!(stderr.contains("Usage:"), "{stderr}");
}

#[test]
fn version_dilaporkan() {
    let output = verge(&["--version"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("verge "));
}

#[test]
fn init_tanpa_argument_menolak_path_kedua() {
    let output = verge(&["init", "satu", "dua"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("at most one path"), "{stderr}");
}
