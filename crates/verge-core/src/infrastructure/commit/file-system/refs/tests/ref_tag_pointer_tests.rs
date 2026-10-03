//! File: `ref_tag_pointer_tests.rs`
//!
//! Deskripsi: Test adapter pointer tag pada filesystem.
//! Layer: infrastructure/commit/file-system/refs/tests
//! Tanggung jawab: Membuktikan tag immutable di disk, penolakan nama tidak
//!   aman, dan daftar tag yang mengabaikan berkas sementara.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.3.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `pointer_fixtures.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use std::fs;
use std::path::PathBuf;

use super::pointer_fixtures::pointer;
use crate::config::repository_layout::RepositoryLayout;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::ident::value_objects::digest::Digest;
use crate::infrastructure::commit::file_system::refs::file_tag_pointer::FileTagPointer;
use crate::shared::exceptions::verge_error::VergeError;

/// Menyiapkan repository kosong dengan adapter pointer tag.
fn tags(name: &str) -> (PathBuf, FileTagPointer) {
    let (dir, _) = pointer(name);
    (
        dir.clone(),
        FileTagPointer::new(RepositoryLayout::under(&dir)),
    )
}

#[test]
fn tag_baru_ditulis_dan_dibaca_kembali() {
    let (dir, tags) = tags("tag-roundtrip");
    let id = Digest::of(b"commit q3");

    tags.create("q3", id).expect("tag dibuat");

    assert_eq!(tags.resolve("q3").expect("tag terbaca"), Some(id));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn tag_yang_sudah_ada_tidak_bergeser_setelah_percobaan_kedua() {
    let (dir, tags) = tags("tag-immutable");
    let first = Digest::of(b"commit pertama");
    let second = Digest::of(b"commit kedua");
    tags.create("q3", first).expect("tag pertama");

    let error = tags.create("q3", second).expect_err("tag tidak ditimpa");

    assert!(matches!(error, VergeError::TagAlreadyExists(name) if name == "q3"));
    assert_eq!(
        tags.resolve("q3").expect("tag terbaca"),
        Some(first),
        "pointer tetap menunjuk commit semula"
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn nama_tag_tidak_aman_ditolak_sebelum_menyentuh_disk() {
    let (dir, tags) = tags("tag-nama");

    for name in ["../keluar", "refs/heads/main", ".hidden", "NUL", " "] {
        assert!(
            matches!(
                tags.create(name, Digest::of(b"commit")),
                Err(VergeError::InvalidName(_))
            ),
            "`{name}` harus ditolak sebagai InvalidName"
        );
    }
    assert!(
        tags.tags().expect("daftar tag").is_empty(),
        "tidak ada tag tertulis"
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn nama_tag_terlalu_panjang_ditolak_sebelum_menyentuh_disk() {
    let (dir, tags) = tags("tag-panjang");
    let name = "a".repeat(256);

    assert!(matches!(
        tags.create(&name, Digest::of(b"commit")),
        Err(VergeError::InvalidName(_))
    ));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn daftar_tag_mengabaikan_berkas_sementara_dan_terurut_naik() {
    let (dir, tags) = tags("tag-daftar");
    tags.create("zulu", Digest::of(b"commit z")).expect("tag z");
    tags.create("alfa", Digest::of(b"commit a")).expect("tag a");
    let tag_dir = RepositoryLayout::under(&dir).tags();
    fs::write(tag_dir.join("pointer-tmp-1-0.tmp"), "sisa\n").expect("sisa sementara");

    assert_eq!(
        tags.tags().expect("daftar tag"),
        vec!["alfa".to_owned(), "zulu".to_owned()]
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn hapus_tag_membuang_pointer_tanpa_menyentuh_blok() {
    let (dir, tags) = tags("tag-hapus");
    tags.create("q3", Digest::of(b"commit"))
        .expect("tag dibuat");

    tags.delete("q3").expect("tag dihapus");

    assert_eq!(tags.resolve("q3").expect("tag terbaca"), None);
    assert!(matches!(
        tags.delete("q3"),
        Err(VergeError::UnknownTag(name)) if name == "q3"
    ));
    drop(fs::remove_dir_all(&dir));
}
