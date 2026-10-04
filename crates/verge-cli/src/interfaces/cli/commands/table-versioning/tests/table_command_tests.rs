//! File: `table_command_tests.rs`
//!
//! Deskripsi: Test kontrak router perintah versioning tabel.
//! Layer: interfaces/cli/commands/table-versioning
//! Tanggung jawab: Membuktikan daftar perintah tabel dan cabang `match` router
//!   selalu sama, sehingga perintah yang terdaftar tidak pernah berakhir
//!   sebagai `unknown table command`.
//!
//! Author: Miruameli
//! Created: 2026-10-04
//! Modified: 2026-10-04
//! Version: 0.3.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `dispatch.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use super::super::dispatch::{is_table_command, run_table_command};

/// Nama perintah tabel yang harus ditangani router.
const TABLE_COMMANDS: [&str; 6] = ["import", "commit", "log", "show", "diff", "query"];

#[test]
fn setiap_perintah_tabel_dikenali_sebagai_perintah_tabel() {
    for name in TABLE_COMMANDS {
        assert!(
            is_table_command(name),
            "`{name}` harus ditangani router tabel"
        );
    }
}

#[test]
fn perintah_kelompok_lain_tidak_dikenali_sebagai_perintah_tabel() {
    for name in [
        "init",
        "branch",
        "merge",
        "tag",
        "--help",
        "--version",
        "bogus",
    ] {
        assert!(!is_table_command(name), "`{name}` bukan perintah tabel");
    }
}

#[test]
fn perintah_tabel_tak_dikenal_ditolak_dengan_daftar_yang_jelas() {
    let args = vec!["frobnicate".to_owned()];
    let error = run_table_command(&args).expect_err("perintah tak dikenal harus ditolak");
    let message = error.to_string();
    assert!(message.contains("unknown table command"), "{message}");
    assert!(
        message.contains("query"),
        "{message} harus menyebut perintah yang ada"
    );
}

#[test]
fn perintah_tabel_tanpa_argumen_ditolak() {
    let error = run_table_command(&[]).expect_err("argv kosong harus ditolak");
    assert!(
        error.to_string().contains("missing table command"),
        "{error}"
    );
}
