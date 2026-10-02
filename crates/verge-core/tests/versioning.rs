//! File: versioning.rs
//!
//! Deskripsi: Test integrasi milestone 1 — storage + commit graph.
//! Layer: tests (integration)
//! Tanggung jawab: Membuktikan histories immutable dan branch berbagi storage.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `verge_core` (API publik)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use verge_core::application::repository_bootstrap::use_cases::initialize_repository::initialize_repository;
use verge_core::config::repository_layout::RepositoryLayout;
use verge_core::infrastructure::storage::file_system::file_store_factory::FileStoreFactory;
use verge_core::infrastructure::storage::file_system::local_file_system::LocalFileSystem;
use verge_core::{Commit, CommitGraph, FileBlockStore, Ref, Store};

/// Direktori sementara yang unik untuk satu pengujian.
fn scratch(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let dir = std::env::temp_dir().join(format!("verge-it-{name}-{}-{nanos}", std::process::id()));
    drop(fs::remove_dir_all(&dir));
    dir
}

/// Membuat commit dengan author dan waktu tetap agar test deterministik.
fn commit(tree: verge_core::BlockId, parents: Vec<verge_core::CommitId>, message: &str) -> Commit {
    Commit::new(
        parents,
        tree,
        "verge <verge@example.com>",
        message,
        1_760_000_000_000,
    )
}

#[test]
fn sejarah_immutable_dan_branching_tidak_menyalin_data() {
    let dir = scratch("versioning");
    let layout = RepositoryLayout::under(&dir);
    let repository = initialize_repository(&layout, &FileStoreFactory, &LocalFileSystem)
        .expect("repository harus terbentuk");
    let store: &FileBlockStore = repository.store();
    let mut graph = CommitGraph::new();

    let base = store.put(b"users-v1").expect("tulis snapshot awal");
    let root = commit(base.id, vec![], "chore: seed users table");
    graph.insert(root.clone()).expect("insert root");
    graph.set_branch("main", root.id()).expect("branch main");

    let feature_tree = store.put(b"users-v2").expect("tulis snapshot fitur");
    let feature = commit(feature_tree.id, vec![root.id()], "feat: add index column");
    graph.insert(feature.clone()).expect("insert feature");
    graph
        .set_branch("feature", feature.id())
        .expect("branch feature");

    let main_tip = graph
        .resolve(&Ref::Branch("main".to_owned()))
        .expect("branch main resolve");
    let feature_tip = graph
        .resolve(&Ref::Branch("feature".to_owned()))
        .expect("branch feature resolve");
    assert!(graph.is_ancestor(main_tip, feature_tip));
    assert_eq!(graph.len(), 2, "branch tidak menambah commit");

    // Main tetap membaca snapshot-nya sendiri meski branch fitur bergerak.
    let main_tree = graph.commit(&main_tip).expect("commit main").tree();
    let feature_snapshot = graph.commit(&feature_tip).expect("commit feature").tree();
    assert_eq!(main_tree, base.id);
    assert_eq!(feature_snapshot, feature_tree.id);

    // Tag membekukan keadaan untuk audit.
    graph.set_tag("v0.1.0", main_tip).expect("buat tag");
    assert_eq!(
        graph.resolve(&Ref::Tag("v0.1.0".to_owned())),
        Some(main_tip)
    );

    // Snapshot lama masih terbaca persis seperti saat ditulis.
    assert_eq!(
        store.get(&main_tree).expect("baca snapshot lama"),
        b"users-v1"
    );

    // Menulis ulang isi yang sama tidak menggandakan penyimpanan.
    let rewritten = store.put(b"users-v1").expect("tulis ulang snapshot");
    assert!(!rewritten.inserted, "snapshot identik wajib deduplikasi");
    assert_eq!(rewritten.id, main_tree);

    // Seluruh sejarah dapat dibaca dari blok yang tersimpan.
    for entry in graph.first_parent_log(feature_tip, 10) {
        assert!(store.contains(&entry.tree()), "snapshot harus tersedia");
    }

    drop(fs::remove_dir_all(&dir));
}

#[test]
fn repository_baru_menolak_pembuatan_ganda() {
    let dir = scratch("bootstrap");
    let layout = RepositoryLayout::under(&dir);
    initialize_repository(&layout, &FileStoreFactory, &LocalFileSystem).expect("bootstrap pertama");
    let second = initialize_repository(&layout, &FileStoreFactory, &LocalFileSystem);
    assert!(second.is_err(), "repository kedua harus ditolak");
    drop(fs::remove_dir_all(&dir));
}
