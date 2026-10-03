//! File: `query_read_cli.rs`
//!
//! Deskripsi: Test end-to-end `verge query`.
//! Layer: interfaces/cli/tests
//! Tanggung jawab: Membuktikan `AS OF` mengembalikan keadaan yang benar.
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

use super::query_fixtures::{commit_time, timeline};
use super::support::{verge_error, verge_stdout};

#[test]
fn query_pada_waktu_commit_pertama_menampilkan_isi_kesaat_itu() {
    let dir = timeline("as-of-first");
    let oldest = commit_time(&dir, 2);

    let table = verge_stdout(&dir, &["query", "--table", "users", "--as-of", &oldest]);

    assert!(table.contains("1,ana"), "{table}");
    assert!(!table.contains("2,budi"), "baris kedua belum ada: {table}");
    assert!(
        !table.contains("3,citra"),
        "baris ketiga belum ada: {table}"
    );
}

#[test]
fn query_pada_waktu_commit_terakhir_sama_dengan_show_head() {
    let dir = timeline("as-of-latest");
    let newest = commit_time(&dir, 0);

    let queried = verge_stdout(&dir, &["query", "--table", "users", "--as-of", &newest]);
    let head = verge_stdout(&dir, &["show", "HEAD", "--table", "users"]);

    assert_eq!(queried, head, "waktu terakhir sama dengan HEAD");
}

#[test]
fn unix_milidetik_dipakai_sebagai_waktu() {
    let dir = timeline("as-of-unix");

    let queried = verge_stdout(
        &dir,
        &["query", "--table", "users", "--as-of", "@99999999999999"],
    );

    assert!(
        queried.contains("3,citra"),
        "milidetik melewati batas tinggi: {queried}"
    );
}

#[test]
fn unix_milidetik_menunjuk_waktu_yang_lebih_kecil_dari_komiterakhir() {
    let dir = timeline("as-of-unix-early");

    // 1 January 1970 jauh sebelum commit pertama repository ini.
    let error = verge_error(&dir, &["query", "--table", "users", "--as-of", "@0"]);

    assert!(error.contains("no commit at or before"), "{error}");
}

#[test]
fn tiga_commit_memiliki_waktu_yang_saling_berbeda() {
    // Jaga keaslian fixture: bila jam sistem terlalu kasar, seluruh test
    // `AS OF` di berkas ini akan menghasilkan jawaban yang salah diam-diam.
    let dir = timeline("as-of-distinct");
    let times: Vec<String> = (0..3).map(|index| commit_time(&dir, index)).collect();

    let mut sorted = times.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), 3, "waktu commit harus berbeda: {times:?}");
}

#[test]
fn timestamp_dicetak_log_dapat_dipakai_langsung_sebagai_as_of() {
    // Alur audit yang paling sering dipakai: baca log, salin waktunya, query.
    let dir = timeline("as-of-copy-paste");
    let printed = commit_time(&dir, 1);

    let queried = verge_stdout(&dir, &["query", "--table", "users", "--as-of", &printed]);

    assert!(queried.contains("2,budi"), "{queried}");
    assert!(!queried.contains("3,citra"), "{queried}");
}
