//! File: `file_commit_repository_tests.rs`
//!
//! Deskripsi: Test integrasi `FileCommitRepository` pada block store sementara.
//! Layer: infrastructure/commit/file-system
//! Tanggung jawab: Membuktikan dedup commit dan penolakan byte yang dimanipulasi.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `file_commit_repository.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::file_commit_repository::FileCommitRepository;
use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::infrastructure::storage::file_system::file_block_store::FileBlockStore;
use crate::shared::exceptions::verge_error::VergeError;

/// Direktori sementara yang unik untuk satu pengujian.
fn scratch(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let dir = std::env::temp_dir().join(format!(
        "verge-commit-{name}-{}-{nanos}",
        std::process::id()
    ));
    drop(fs::remove_dir_all(&dir));
    dir
}

/// Membangun commit contoh tanpa parent untuk tabel `users`.
fn sample(message: &str) -> Commit {
    let table = TableName::parse("users").expect("nama tabel valid");
    let tree = Digest::of(b"tree");
    Commit::new(Vec::new(), tree, &table, "ana", message, 1_700_000_000_000)
}

#[test]
fn commit_tersimpan_dapat_dimuat_kembali_dengan_identitas_sama() {
    let dir = scratch("roundtrip");
    let store = FileBlockStore::open(&dir).expect("buka store");
    let repo = FileCommitRepository::new(store.clone());
    let commit = sample("feat: seed");

    let outcome = repo.save(&commit).expect("simpan commit");
    assert_eq!(
        outcome.id,
        commit.id(),
        "nama blok wajib sama dengan id commit"
    );
    assert_eq!(repo.load(&commit.id()).expect("muat commit"), commit);
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn commit_identik_tidak_didua_gandakan_bloknya() {
    let dir = scratch("dedup");
    let store = FileBlockStore::open(&dir).expect("buka store");
    let repo = FileCommitRepository::new(store.clone());
    let commit = sample("feat: seed");

    assert!(repo.save(&commit).expect("tulis pertama").inserted);
    let second = repo.save(&commit).expect("tulis kedua");
    assert!(!second.inserted, "commit identik wajib deduplikasi");
    assert_eq!(
        repo.load(&commit.id()).expect("muat commit").message(),
        commit.message()
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn byte_commit_yang_dimanipulasi_ditolak_saat_dimuat() {
    let dir = scratch("tamper");
    let store = FileBlockStore::open(&dir).expect("buka store");
    let repo = FileCommitRepository::new(store.clone());
    let commit = sample("feat: seed");
    repo.save(&commit).expect("simpan commit");
    // Field timestamp diubah satu bit: byte tetap kanonik, jadi yang bisa
    // menangkapnya hanya pencocokan digest dengan nama blok.
    let mut bytes = commit.encode();
    let last = bytes.len().checked_sub(1).expect("encoding tidak kosong");
    bytes[last] ^= 0x01;
    fs::write(store.path_of(&commit.id()), &bytes).expect("tulis byte dimanipulasi");

    let error = repo
        .load(&commit.id())
        .expect_err("byte dimanipulasi wajib ditolak");
    match error {
        VergeError::MalformedCommit { reason } => {
            assert_eq!(reason, "digest does not match the block name");
        }
        other => panic!("ditolak dengan MalformedCommit, bukan {other:?}"),
    }
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn byte_commit_bukan_encoding_kanonik_ditolak() {
    let dir = scratch("garbage");
    let store = FileBlockStore::open(&dir).expect("buka store");
    let repo = FileCommitRepository::new(store.clone());
    let foreign = store.put(b"bukan commit").expect("tulis blok lain");

    let error = repo
        .load(&foreign.id)
        .expect_err("byte bukan commit wajib ditolak");
    assert!(
        matches!(error, VergeError::MalformedCommit { .. }),
        "ditolak dengan MalformedCommit, bukan {error:?}"
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn commit_yang_tidak_ada_dilaporkan_sebagai_commit_not_found() {
    let dir = scratch("missing");
    let store = FileBlockStore::open(&dir).expect("buka store");
    let repo = FileCommitRepository::new(store);
    let absent = Digest::of(b"objek commit yang tidak pernah ditulis");

    assert!(matches!(
        repo.load(&absent),
        Err(VergeError::CommitNotFound(id)) if id == absent
    ));
    drop(fs::remove_dir_all(&dir));
}
