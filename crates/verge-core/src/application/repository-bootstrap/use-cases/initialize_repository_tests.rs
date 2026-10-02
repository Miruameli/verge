//! File: `initialize_repository_tests.rs`
//!
//! Deskripsi: Test use case bootstrap dengan port palsu.
//! Layer: application/repository-bootstrap/use-cases
//! Tanggung jawab: Membuktikan urutan pembuatan, guard, dan rollback.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `initialize_repository.rs`, `initialize_repository_fakes.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use super::initialize_repository::initialize_repository;
use super::initialize_repository_fakes::{FakeFactory, FakeFileSystem};
use crate::config::repository_layout::{RepositoryLayout, HEAD_MAIN};
use crate::shared::exceptions::verge_error::VergeError;

use crate::domain::storage::ports::metadata_writer::MetadataWriter;
/// Factory yang selalu berhasil, dipakai pada skenario happy path.
fn working_factory() -> FakeFactory {
    FakeFactory { fail: false }
}

/// Factory yang gagal membuka store, dipakai pada skenario rollback.
fn failing_factory() -> FakeFactory {
    FakeFactory { fail: true }
}

/// Layout repository di workspace yang tidak pernah dipakai test lain.
fn layout() -> RepositoryLayout {
    RepositoryLayout::under("/workspace/verge-dokumentasi")
}

#[test]
fn bootstrap_membuat_layout_lengkap() {
    let filesystem = FakeFileSystem::default();
    let repo = initialize_repository(&layout(), &working_factory(), &filesystem)
        .expect("bootstrap harus berhasil");
    let layout = repo.layout();
    assert!(
        filesystem.exists(&layout.heads()),
        "namespace branch harus ada"
    );
    assert!(filesystem.exists(&layout.tags()), "namespace tag harus ada");
    assert_eq!(
        filesystem.contents_of(&layout.head_file()),
        Some(HEAD_MAIN.as_bytes().to_vec()),
        "HEAD harus menunjuk branch default"
    );
}

#[test]
fn bootstrap_menolak_repository_yang_sudah_ada() {
    let filesystem = FakeFileSystem::default();
    let layout = layout();
    initialize_repository(&layout, &working_factory(), &filesystem).expect("bootstrap pertama");
    let second = initialize_repository(&layout, &working_factory(), &filesystem);
    assert!(matches!(
        second,
        Err(VergeError::RepositoryAlreadyExists(_))
    ));
}

#[test]
fn rollback_membersihkan_layout_saat_gagal_menulis() {
    let layout = layout();
    let filesystem = FakeFileSystem::failing_on(layout.head_file());
    let result = initialize_repository(&layout, &working_factory(), &filesystem);
    assert!(result.is_err(), "gagal menulis HEAD harus gagal");
    assert!(
        !filesystem.exists(&layout.heads()),
        "direktori harus dibersihkan"
    );
    assert!(!filesystem.exists(&layout.root), "root repo harus dihapus");
}

#[test]
fn rollback_membersihkan_layout_saat_store_gagal_dibuka() {
    let layout = layout();
    let filesystem = FakeFileSystem::default();
    let result = initialize_repository(&layout, &failing_factory(), &filesystem);
    assert!(result.is_err(), "store gagal dibuka harus gagal");
    assert!(!filesystem.exists(&layout.root), "root repo harus dihapus");
}
