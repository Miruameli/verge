//! File: `tag_rejection_cli.rs`
//!
//! Deskripsi: Test penolakan perintah tag.
//! Layer: interfaces/cli/tests/time-travel
//! Tanggung jawab: Membuktikan tag immutable dan input tidak valid ditolak
//!   sebelum pointer apa pun ditulis.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `tag_fixtures.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::support::{gap, stage_and_commit, verge_error, verge_stdout};
use super::tag_fixtures::two_commits;

#[test]
fn tag_yang_sudah_ada_ditolak_dan_tidak_bergeser() {
    let dir = two_commits("tag-immutable");
    verge_stdout(&dir, &["tag", "create", "q3"]);
    gap();
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n3,citra\n", "feat: citra");

    let error = verge_error(&dir, &["tag", "create", "q3"]);
    let queried = verge_stdout(&dir, &["query", "--table", "users", "--as-of", "q3"]);

    assert!(error.contains("already exists"), "{error}");
    assert!(
        !queried.contains("3,citra"),
        "tag lama harus tetap menunjuk keadaan lama:\n{queried}"
    );
}

#[test]
fn prefix_refs_tags_menunjuk_tag_secara_eksplisit() {
    let dir = two_commits("tag-explicit");
    verge_stdout(&dir, &["tag", "create", "q3"]);

    let queried = verge_stdout(
        &dir,
        &["query", "--table", "users", "--as-of", "refs/tags/q3"],
    );

    assert!(queried.contains("2,budi"), "{queried}");
}

#[test]
fn tag_yang_tidak_ada_ditolak_dengan_pesan_yang_jelas() {
    let dir = two_commits("tag-unknown");

    let error = verge_error(&dir, &["query", "--table", "users", "--as-of", "tidak-ada"]);

    assert!(error.contains("invalid reference"), "{error}");
}

#[test]
fn tag_ke_timestamp_ditolak() {
    let dir = two_commits("tag-instant");

    let error = verge_error(
        &dir,
        &["tag", "create", "q3", "--revision", "2026-10-01T10:00:00Z"],
    );

    assert!(error.contains("cannot point at a timestamp"), "{error}");
}

#[test]
fn menghapus_tag_menyisakan_data_commit() {
    let dir = two_commits("tag-delete");
    verge_stdout(&dir, &["tag", "create", "q3"]);

    verge_stdout(&dir, &["tag", "delete", "q3"]);

    assert!(
        verge_stdout(&dir, &["tag", "list"]).trim().is_empty(),
        "tag hilang dari daftar"
    );
    let still_there = verge_stdout(&dir, &["show", "HEAD", "--table", "users"]);
    assert!(
        still_there.contains("2,budi"),
        "data commit tetap ada: {still_there}"
    );
}

#[test]
fn menghapus_tag_yang_tidak_ada_ditolak() {
    let dir = two_commits("tag-delete-unknown");

    let error = verge_error(&dir, &["tag", "delete", "tidak-ada"]);

    assert!(error.contains("does not exist"), "{error}");
}

#[test]
fn nama_tag_tidak_aman_ditolak() {
    let dir = two_commits("tag-unsafe");

    let error = verge_error(&dir, &["tag", "create", "../keluar"]);

    assert!(error.contains("invalid reference name"), "{error}");
}

#[test]
fn subperintah_tag_tidak_dikenal_ditolak() {
    let dir = two_commits("tag-subcommand");

    let error = verge_error(&dir, &["tag", "ubah", "q3"]);

    assert!(error.contains("unknown tag subcommand"), "{error}");
}
