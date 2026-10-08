//! File: `ref_pointer_tests.rs`
//!
//! Deskripsi: Test pembacaan pointer commit dan `HEAD`.
//! Layer: infrastructure/commit/file-system/tests
//! Tanggung jawab: Membuktikan parsing pointer dan penolakan pointer rusak.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `pointer_fixtures.rs`
//!   - `config/repository_layout.rs`
//!   - `domain/commit/repositories/ports/ref_pointer.rs`
//!   - `domain/ident/value-objects/digest.rs`
//!   - `domain/ident/value-objects/digest_text.rs`
//!   - `shared/exceptions/verge_error.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::fs;

use super::pointer_fixtures::pointer;
use crate::config::repository_layout::RepositoryLayout;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::ident::value_objects::digest_text::HexText;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn head_branch_mengembalikan_nama_tanpa_prefix() {
    let (dir, refs) = pointer("head");
    assert_eq!(refs.head_branch().expect("baca HEAD"), "main");
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn pointer_branch_berpindah_pindah_sesuai_advance() {
    let (dir, refs) = pointer("roundtrip");
    let layout = RepositoryLayout::under(&dir);
    let first = Digest::of(b"commit pertama");
    let second = Digest::of(b"commit kedua");

    refs.advance("main", first).expect("advance pertama");
    assert_eq!(refs.resolve("main").expect("resolve pertama"), Some(first));
    refs.advance("feature", second)
        .expect("advance branch lain");
    assert_eq!(
        refs.resolve("feature").expect("resolve feature"),
        Some(second)
    );
    assert_eq!(refs.resolve("main").expect("resolve main"), Some(first));
    assert_eq!(refs.head_branch().expect("baca HEAD"), "main");

    let raw = fs::read_to_string(layout.heads().join("main")).expect("baca pointer");
    assert_eq!(raw, format!("{}\n", first.to_hex()));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn branch_belum_punya_commit_menghasilkan_none() {
    let (dir, refs) = pointer("unborn");
    assert_eq!(
        refs.resolve("belum-pernah").expect("resolve branch kosong"),
        None
    );
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn pointer_hex_rusak_ditolak_sebagai_malformed_pointer() {
    let (dir, refs) = pointer("broken");
    let layout = RepositoryLayout::under(&dir);
    fs::write(layout.heads().join("main"), "bukan-hex\n").expect("tulis pointer rusak");

    assert!(matches!(
        refs.resolve("main"),
        Err(VergeError::MalformedPointer(_))
    ));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn nama_branch_tidak_valid_ditolak_sebagai_invalid_name() {
    let (dir, refs) = pointer("badname");
    let id = Digest::of(b"commit");

    assert!(matches!(
        refs.advance(".tersembunyi", id),
        Err(VergeError::InvalidName(_))
    ));
    assert!(matches!(
        refs.resolve("a/b"),
        Err(VergeError::InvalidName(_))
    ));
    drop(fs::remove_dir_all(&dir));
}

#[test]
fn head_tanpa_prefix_ref_ditolak() {
    let (dir, refs) = pointer("badhead");
    let layout = RepositoryLayout::under(&dir);
    fs::write(layout.head_file(), "main\n").expect("tulis HEAD rusak");

    assert!(matches!(
        refs.head_branch(),
        Err(VergeError::MalformedPointer(_))
    ));
    drop(fs::remove_dir_all(&dir));
}
