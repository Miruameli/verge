//! File: `merge_base_tests.rs`
//!
//! Deskripsi: Test pencarian merge base.
//! Layer: tests (integration)
//! Tanggung jawab: Membuktikan base terdekat pada rantai first-parent.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/merge_base.rs`
//!   - `infrastructure/commit/file-system/**`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use std::fs;
use std::path::PathBuf;

use verge_core::config::repository_layout::RepositoryLayout;
use verge_core::domain::commit::entities::commit::Commit;
use verge_core::domain::ident::value_objects::digest::Digest;
use verge_core::domain::merge::merge_base::find_merge_base;
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;

use super::commit_chain_fixture::commit_chain;

/// Menyiapkan direktori sementara yang unik per nama test.
fn scratch(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("verge-merge-{name}-{}", std::process::id()))
}

/// Membuka repository commit pada `dir` untuk dibaca.
fn reader(dir: &std::path::Path) -> FileCommitRepository {
    let layout = RepositoryLayout::under(dir);
    let store = FileBlockStore::open(layout.objects()).expect("buka store");
    FileCommitRepository::new(store)
}

#[test]
fn base_adalah_leluhur_terdekat_yang_bersama() {
    let dir = scratch("nearest");
    let main = commit_chain(&dir, &["a", "b", "c", "d"]);

    let found = find_merge_base(main[3], main[1], &reader(&dir)).expect("cari base");

    assert_eq!(found, Some(main[1]), "base adalah leluhur terdekat");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn base_sama_dengan_akar_bila_branch_tidak_pernah_cabang() {
    let dir = scratch("root");
    let chain = commit_chain(&dir, &["a", "b", "c"]);

    let found = find_merge_base(chain[2], chain[0], &reader(&dir)).expect("cari base");

    assert_eq!(found, Some(chain[0]), "akar adalah leluhur bersama");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn ujung_yang_sama_menjadi_base_itu_sendiri() {
    let dir = scratch("same");
    let chain = commit_chain(&dir, &["a", "b"]);

    let found = find_merge_base(chain[1], chain[1], &reader(&dir)).expect("cari base");

    assert_eq!(found, Some(chain[1]));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn chain_yang_tidak_bercabang_tidak_punya_base_bersama() {
    let dir = scratch("split");
    let left = commit_chain(&dir, &["a", "b"]);
    let right = commit_chain(&dir, &["z"]);

    let found = find_merge_base(left[1], right[0], &reader(&dir)).expect("cari base");

    assert_eq!(found, None, "tidak ada leluhur bersama");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn commit_yang_tidak_ada_di_store_ditolak_bukan_ditebak() {
    let dir = scratch("missing");
    let chain = commit_chain(&dir, &["a"]);
    let table = TableName::parse("users").expect("nama tabel");
    let asing = Commit::new(
        Vec::new(),
        Digest::of(b"tree"),
        &table,
        "ana",
        "feat: tidak pernah disimpan",
        0,
    );

    let found = find_merge_base(chain[0], asing.id(), &reader(&dir));

    assert!(
        found.is_err(),
        "commit tak dikenal harus gagal, bukan dianggap base"
    );
    drop(fs::remove_dir_all(&dir));
}
