//! File: `file_block_store_tests.rs`
//!
//! Deskripsi: Test integrasi `FileBlockStore` di filesystem sementara.
//! Layer: infrastructure/storage/file-system
//! Tanggung jawab: Membuktikan dedup, layout, durability, dan kontrak blok hilang.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `file_block_store.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::file_block_store::FileBlockStore;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::storage::ports::block_store::Store;
use crate::shared::exceptions::verge_error::VergeError;

/// Direktori sementara yang unik untuk satu pengujian.
fn scratch(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let dir = std::env::temp_dir().join(format!("verge-fs-{name}-{}-{nanos}", std::process::id()));
    drop(fs::remove_dir_all(&dir));
    dir
}

#[test]
fn byte_identik_tidak_didua_gandakan() {
    let dir = scratch("dedup");
    let store = FileBlockStore::open(&dir).expect("buka store");
    let first = store.put(b"payload").expect("tulis pertama");
    let second = store.put(b"payload").expect("tulis kedua");
    assert!(first.inserted, "blok pertama baru dibuat");
    assert!(!second.inserted, "blok kedua wajib deduplikasi");
    assert_eq!(first.id, second.id);
    assert_eq!(store.get(&first.id).expect("baca balik"), b"payload");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn layout_memakai_fan_out_dan_nama_content_addressed() {
    let dir = scratch("layout");
    let store = FileBlockStore::open(&dir).expect("buka store");
    let outcome = store.put(b"layout").expect("tulis blok");
    let path = store.path_of(&outcome.id);
    let hex = outcome.id.to_hex();
    let expected = format!("{}/{}/{hex}", &hex[..2], &hex[2..4]);
    assert!(
        path.ends_with(&expected),
        "{path:?} tidak berakhir dengan {expected}"
    );
    assert!(path.is_file());
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn blok_hilang_dilaporkan_sebagai_error() {
    let dir = scratch("missing");
    let store = FileBlockStore::open(&dir).expect("buka store");
    let absent = Digest::of(b"tidak pernah ditulis");
    assert!(!store.contains(&absent));
    assert!(matches!(
        store.get(&absent),
        Err(VergeError::BlockNotFound { .. })
    ));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn blok_tetap_ada_setelah_store_dibuka_ulang() {
    let dir = scratch("durable");
    let outcome = FileBlockStore::open(&dir)
        .expect("buka store")
        .put(b"durable")
        .expect("tulis blok");
    let reopened = FileBlockStore::open(&dir).expect("buka ulang");
    assert!(reopened.contains(&outcome.id));
    assert_eq!(reopened.get(&outcome.id).expect("baca blok"), b"durable");
    drop(fs::remove_dir_all(&dir));
}
