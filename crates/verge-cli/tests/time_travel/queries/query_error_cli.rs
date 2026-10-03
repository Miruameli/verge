//! File: `query_error_cli.rs`
//!
//! Deskripsi: Test penolakan `verge query`.
//! Layer: interfaces/cli/tests/time-travel
//! Tanggung jawab: Membuktikan setiap `--as-of` yang tidak dapat dijawab ditolak
//!   dengan pesan yang menyebut jalan keluarnya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `query_fixtures.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::query_fixtures::timeline;
use super::support::verge_error;

#[test]
fn waktu_sebelum_commit_pertama_ditolak_dan_menyebut_batas() {
    let dir = timeline("as-of-too-early");

    let error = verge_error(
        &dir,
        &[
            "query",
            "--table",
            "users",
            "--as-of",
            "2020-01-01T00:00:00Z",
        ],
    );

    assert!(error.contains("no commit at or before"), "{error}");
    assert!(error.contains("oldest commit is"), "{error}");
}

#[test]
fn offset_bukan_nol_ditolak_dengan_pesan_yang_menunjuk_format() {
    let dir = timeline("as-of-offset");

    let error = verge_error(
        &dir,
        &[
            "query",
            "--table",
            "users",
            "--as-of",
            "2026-10-01T10:00:00+07:00",
        ],
    );

    assert!(error.contains("RFC 3339 UTC"), "{error}");
}

#[test]
fn query_menolak_tanpa_as_of() {
    let dir = timeline("as-of-missing");

    let error = verge_error(&dir, &["query", "--table", "users"]);

    assert!(error.contains("--as-of"), "{error}");
}

#[test]
fn query_ke_tabel_yang_belum_ada_ditolak() {
    let dir = timeline("as-of-unknown-table");

    let error = verge_error(&dir, &["query", "--table", "orders", "--as-of", "HEAD"]);

    assert!(
        !error.is_empty(),
        "tabel tanpa commit harus ditolak: {error}"
    );
}
