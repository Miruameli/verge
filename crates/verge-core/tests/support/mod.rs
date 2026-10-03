//! File: mod.rs
//!
//! Deskripsi: Harness bersama untuk test integrasi Verge.
//! Layer: tests
//! Tanggung jawab: Menyusun repository sementara beserta adapter port-nya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `flow.rs`
//!   - `verge_core` (application, infrastructure, config)
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

// Modul ini dipakai dua test binary berbeda; sebagian helper tidak terpakai di
// keduanya, jadi peringatan dead code dimatikan tepat di batas modul ini.
#![allow(dead_code)]

#[path = "flow.rs"]
pub mod flow;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use verge_core::application::repository_bootstrap::use_cases::initialize_repository::initialize_repository;
use verge_core::config::repository_layout::RepositoryLayout;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::commit::file_system::refs::file_ref_pointer::FileRefPointer;
use verge_core::infrastructure::commit::file_system::refs::file_tag_pointer::FileTagPointer;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;
use verge_core::infrastructure::storage::file_system::file_store_factory::FileStoreFactory;
use verge_core::infrastructure::storage::file_system::local_file_system::LocalFileSystem;
use verge_core::infrastructure::table::file_system::file_table_workspace::FileTableWorkspace;

/// Adapter yang sudah dirangkai untuk seluruh port repository.
pub struct Harness {
    /// Layout repository pada folder kerja sementara.
    pub layout: RepositoryLayout,
    /// Object store blok.
    pub store: FileBlockStore,
    /// Port data kerja tabel.
    pub workspace_port: FileTableWorkspace,
    /// Port penyimpanan objek commit.
    pub commits: FileCommitRepository,
    /// Port pointer branch.
    pub refs: FileRefPointer,
    /// Port pointer tag.
    pub tags: FileTagPointer,
}

/// Membuat folder kerja sementara yang unik per pemanggilan.
pub fn workspace(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let path =
        std::env::temp_dir().join(format!("verge-it-{label}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&path).expect("buat folder sementara");
    path
}

/// Menghapus folder kerja beserta isinya tanpa panik bila sudah hilang.
pub fn cleanup(path: &Path) {
    let _ = fs::remove_dir_all(path);
}

/// Menulis berkas sumber data tabel dan mengembalikan path-nya.
pub fn write_source(workspace: &Path, name: &str, contents: &[u8]) -> PathBuf {
    let path = workspace.join(name);
    fs::write(&path, contents).expect("tulis sumber data");
    path
}

/// Menyusun repository kosong beserta adapter port-nya.
pub fn harness(workspace: &Path) -> Harness {
    let layout = RepositoryLayout::under(workspace);
    initialize_repository(&layout, &FileStoreFactory, &LocalFileSystem)
        .expect("bootstrap repository");
    let store = FileBlockStore::open(layout.objects()).expect("buka block store");
    Harness {
        layout: layout.clone(),
        workspace_port: FileTableWorkspace::new(layout.clone()),
        commits: FileCommitRepository::new(store.clone()),
        refs: FileRefPointer::new(layout.clone()),
        tags: FileTagPointer::new(layout),
        store,
    }
}

/// Menghitung jumlah berkas blok yang tersimpan di object store.
pub fn count_blocks(layout: &RepositoryLayout) -> usize {
    fn walk(dir: &Path, total: &mut usize) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, total);
            } else {
                *total += 1;
            }
        }
    }
    let mut total = 0;
    walk(&layout.objects(), &mut total);
    total
}
