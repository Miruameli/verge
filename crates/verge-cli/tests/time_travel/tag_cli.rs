//! File: `tag_cli.rs`
//!
//! Deskripsi: Test end-to-end `verge tag`.
//! Layer: interfaces/cli/tests
//! Tanggung jawab: Membuktikan tag immutable dan dapat dipakai sebagai titik
//!   waktu pada `verge query`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `support/mod.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::support::{gap, head_id, stage_and_commit, verge_stdout};
use super::tag_fixtures::two_commits;

#[test]
fn tag_dibuat_dan_dicetak_beserta_commit_tertuju() {
    let dir = two_commits("tag-create");
    let head = head_id(&dir);

    verge_stdout(&dir, &["tag", "create", "q3"]);
    let listed = verge_stdout(&dir, &["tag", "list"]);

    assert!(listed.starts_with("q3 "), "tag tercetak pertama: {listed}");
    assert!(
        listed.contains(&head[..12]),
        "daftar tag menyebut commit yang ditunjuk:\n{listed}"
    );
}

#[test]
fn tag_menunjuk_commit_yang_diberikan_bukan_head() {
    let dir = two_commits("tag-revision");
    let first = head_id(&dir);
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n3,citra\n", "feat: citra");

    verge_stdout(&dir, &["tag", "create", "awal", "--revision", "HEAD~1"]);
    let listed = verge_stdout(&dir, &["tag", "list"]);

    assert!(listed.contains("awal "), "{listed}");
    assert!(
        listed.contains(&first[..12]),
        "tag menunjuk commit pertama, bukan HEAD:\n{listed}"
    );
}

#[test]
fn query_dengan_tag_menampilkan_keadaan_saat_tag_dibuat() {
    let dir = two_commits("tag-query");
    verge_stdout(&dir, &["tag", "create", "awal"]);
    gap();
    stage_and_commit(&dir, "id,name\n1,ana\n2,budi\n3,citra\n", "feat: citra");

    let queried = verge_stdout(&dir, &["query", "--table", "users", "--as-of", "awal"]);

    assert!(queried.contains("2,budi"), "{queried}");
    assert!(
        !queried.contains("3,citra"),
        "tag tidak ikut bergerak: {queried}"
    );
}
