//! File: `ref_pointer_branch_ops_tests.rs`
//!
//! Deskripsi: Test operasi daftar, switch, dan hapus pada adapter pointer.
//! Layer: infrastructure/commit/file-system/tests
//! Tanggung jawab: Membuktikan perilaku berkas pointer pada filesystem.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `pointer_fixtures.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use std::fs;
use std::path::PathBuf;

use super::pointer_fixtures::pointer;
use crate::config::repository_layout::{RepositoryLayout, HEAD_MAIN};
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::ident::value_objects::digest::Digest;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn daftar_branch_mengurutkan_nama_dan_mengabaikan_berkas_sementara() {
    let (dir, refs) = pointer("list");
    let layout = RepositoryLayout::under(&dir);
    refs.advance("main", Digest::of(b"m"))
        .expect("pointer main");
    refs.advance("zeta", Digest::of(b"a"))
        .expect("pointer zeta");
    refs.advance("alpha", Digest::of(b"b"))
        .expect("pointer alpha");
    fs::write(layout.heads().join("pointer-tmp-1-0.tmp"), "sisa\n").expect("sisa sementara");

    let names = refs.branches().expect("daftar branch");

    assert_eq!(
        names,
        vec!["alpha".to_owned(), "main".to_owned(), "zeta".to_owned()]
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn switch_menulis_head_lengkap_walaupun_isi_pointer_bukan_path() {
    let (dir, refs) = pointer("switch");
    refs.advance("eksperimen", Digest::of(b"a"))
        .expect("pointer eksperimen");

    refs.switch("eksperimen").expect("head dialihkan");

    let layout = RepositoryLayout::under(&dir);
    let raw = fs::read_to_string(layout.head_file()).expect("baca HEAD");
    assert_eq!(raw.trim_end(), "ref: refs/heads/eksperimen");
    assert!(!fs::read_dir(layout.heads())
        .expect("baca heads")
        .any(|entry| entry
            .expect("entri")
            .file_name()
            .to_string_lossy()
            .ends_with(".tmp")));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn switch_ke_branch_tidak_ada_ditolak_tanpa_menulis_head() {
    let (dir, refs) = pointer("switch-missing");
    let layout = RepositoryLayout::under(&dir);

    assert!(matches!(
        refs.switch("hantu"),
        Err(VergeError::UnknownBranch(name)) if name == "hantu"
    ));
    assert_eq!(
        fs::read_to_string(layout.head_file()).expect("baca HEAD"),
        HEAD_MAIN
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn hapus_branch_aktif_ditolak_dan_pointer_tetap_ada() {
    let (dir, refs) = pointer("delete-head");
    refs.advance("main", Digest::of(b"a"))
        .expect("pointer main");

    assert!(matches!(
        refs.delete("main"),
        Err(VergeError::BranchInUse(name)) if name == "main"
    ));
    assert!(layout_head_exists(&dir), "HEAD tetap menunjuk main");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn hapus_branch_menghapus_hanya_pointer_bukan_blok() {
    let (dir, refs) = pointer("delete");
    let layout = RepositoryLayout::under(&dir);
    refs.advance("main", Digest::of(b"m"))
        .expect("pointer main");
    refs.advance("eksperimen", Digest::of(b"a"))
        .expect("pointer eksperimen");
    let objects = layout.objects();
    let block = objects.join("blok-teszt");
    fs::create_dir_all(&objects).expect("buat direktori objek");
    fs::write(&block, "isi blok").expect("tulis blok");

    refs.delete("eksperimen").expect("branch dihapus");

    assert!(!layout.heads().join("eksperimen").exists());
    assert_eq!(
        refs.branches().expect("daftar branch"),
        vec!["main".to_owned()],
        "branch lain tidak tersentuh"
    );
    assert_eq!(
        fs::read_to_string(&block).expect("blok masih terbaca"),
        "isi blok",
        "hapus branch tidak boleh menyentuh blok data"
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn hapus_branch_yang_tidak_ada_melaporkan_nama_branch() {
    let (dir, refs) = pointer("delete-missing");

    assert!(matches!(
        refs.delete("hantu"),
        Err(VergeError::UnknownBranch(name)) if name == "hantu"
    ));
    drop(fs::remove_dir_all(&dir));
}

/// Mengembalikan `true` bila `HEAD` masih ada.
fn layout_head_exists(dir: &PathBuf) -> bool {
    RepositoryLayout::under(dir).head_file().exists()
}
