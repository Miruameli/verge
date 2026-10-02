//! File: `commit_tests.rs`
//!
//! Deskripsi: Test determinisme identitas dan encoding commit.
//! Layer: domain/commit/entities
//! Tanggung jawab: Membuktikan identifier commit turun dari isi commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - commit.rs, `commit_encoding.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use super::commit::Commit;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest::Digest;

/// Membuat commit dengan parameter yang bisa ditimpa per test.
fn commit(message: &str, parents: Vec<CommitId>) -> Commit {
    Commit::new(
        parents,
        Digest::of(b"tree"),
        "verge <verge@example.com>",
        message,
        1_760_000_000_000,
    )
}

#[test]
fn identifier_turun_dari_isi_commit() {
    assert_eq!(commit("a", vec![]).id(), commit("a", vec![]).id());
    assert_ne!(commit("a", vec![]).id(), commit("b", vec![]).id());
}

#[test]
fn encoding_deterministik_antar_panggilan() {
    assert_eq!(
        commit("same", vec![]).encode(),
        commit("same", vec![]).encode()
    );
}

#[test]
fn prefiks_panjang_mencegah_kolisi_field() {
    // Tanpa prefiks panjang, author="ab"/message="c" dan author="a"/
    // message="bc" akan menghasilkan byte yang sama.
    let left = Commit::new(vec![], Digest::of(b"tree"), "ab", "c", 0);
    let right = Commit::new(vec![], Digest::of(b"tree"), "a", "bc", 0);
    assert_ne!(left.id(), right.id(), "field berbeda harus berbeda id");
}

#[test]
fn parent_ikut_menentukan_identifier() {
    let root = commit("root", vec![]);
    let child = commit("child", vec![root.id()]);
    assert_ne!(root.id(), child.id());
}

#[test]
fn summary_mengambil_baris_pertama_pesan() {
    let commit = Commit::new(
        vec![],
        Digest::of(b"tree"),
        "author",
        "fix: something\n\nParagraf penjelasan.",
        0,
    );
    assert_eq!(commit.summary(), "fix: something");
    assert!(commit.message().contains("Paragraf penjelasan"));
}
