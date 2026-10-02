//! File: `file_table_workspace_tests.rs`
//!
//! Deskripsi: Test integrasi `FileTableWorkspace` pada repository sementara.
//! Layer: infrastructure/table/file-system
//! Tanggung jawab: Membuktikan staging idempoten dan integritas pointer.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `file_table_workspace.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::file_table_workspace::FileTableWorkspace;
use crate::config::repository_layout::RepositoryLayout;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::infrastructure::storage::file_system::file_block_store::FileBlockStore;
use crate::shared::exceptions::verge_error::VergeError;

/// Direktori sementara yang unik untuk satu pengujian.
fn scratch(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    std::env::temp_dir().join(format!("verge-ws-{name}-{}-{nanos}", std::process::id()))
}

/// Bundle fixture: folder kerja, layout, workspace, store, dan tabel `users`.
struct Fixture {
    /// Folder kerja sementara yang dihapus setelah pengujian.
    dir: PathBuf,
    /// Layout repository di atas `dir`.
    layout: RepositoryLayout,
    /// Workspace yang diuji.
    workspace: FileTableWorkspace,
    /// Handle store yang sama dengan dipakai workspace.
    store: FileBlockStore,
    /// Tabel yang selalu dipakai di semua pengujian.
    table: TableName,
}

/// Menyiapkan repository, workspace, dan tabel `users` di direktori sementara.
fn fixture(name: &str) -> Fixture {
    let dir = scratch(name);
    let layout = RepositoryLayout::under(&dir);
    let store = FileBlockStore::open(layout.objects()).expect("buka store");
    let workspace = FileTableWorkspace::new(layout.clone(), store.clone());
    let table = TableName::parse("users").expect("nama tabel valid");
    Fixture {
        dir,
        layout,
        workspace,
        store,
        table,
    }
}

/// Menghitung jumlah berkas di bawah `dir` untuk membuktikan blok tidak ganda.
fn count_files(dir: &Path) -> usize {
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    entries
        .filter_map(std::result::Result::ok)
        .map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                count_files(&path)
            } else {
                1
            }
        })
        .sum()
}

#[test]
fn stage_menulis_blok_dan_pointer_ke_blok_yang_sama() {
    let fix = fixture("stage");
    let data = b"id,name\n1,ana\n";

    let id = fix.workspace.stage(&fix.table, data).expect("stage tabel");
    assert_eq!(fix.workspace.staged(&fix.table).expect("staged"), Some(id));
    assert_eq!(fix.store.get(&id).expect("baca blok"), data);

    let raw = fs::read_to_string(fix.layout.working_file(&fix.table)).expect("baca pointer");
    assert_eq!(raw, format!("{}\n", id.to_hex()));
    drop(fs::remove_dir_all(&fix.dir));
}

#[test]
fn staging_ulang_dengan_isi_sama_tidak_menggandakan_blok() {
    let fix = fixture("idempoten");
    let data = b"id,name\n1,ana\n";

    let first = fix
        .workspace
        .stage(&fix.table, data)
        .expect("stage pertama");
    let second = fix.workspace.stage(&fix.table, data).expect("stage kedua");
    assert_eq!(first, second, "isi identik wajib menunjuk blok yang sama");
    assert_eq!(count_files(&fix.layout.objects()), 1);
    assert_eq!(
        fix.workspace.staged(&fix.table).expect("staged"),
        Some(first)
    );
    drop(fs::remove_dir_all(&fix.dir));
}

#[test]
fn tabel_belum_di_stage_menghasilkan_none() {
    let fix = fixture("belum");
    assert_eq!(fix.workspace.staged(&fix.table).expect("staged"), None);
    drop(fs::remove_dir_all(&fix.dir));
}

#[test]
fn pointer_kerja_rusak_ditolak_sebagai_malformed_pointer() {
    let fix = fixture("rusak");
    fs::create_dir_all(fix.layout.table_dir(&fix.table)).expect("buat direktori tabel");
    let pointer = fix.layout.working_file(&fix.table);
    fs::write(&pointer, "bukan-hex\n").expect("tulis pointer rusak");

    assert!(matches!(
        fix.workspace.staged(&fix.table),
        Err(VergeError::MalformedPointer(_))
    ));
    drop(fs::remove_dir_all(&fix.dir));
}
