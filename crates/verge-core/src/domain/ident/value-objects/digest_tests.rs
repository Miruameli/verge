//! File: `digest_tests.rs`
//!
//! Deskripsi: Test vector dan konversi hex untuk `Digest`.
//! Layer: domain/ident/value-objects
//! Tanggung jawab: Membuktikan digest deterministik dan hex round-trip.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - digest.rs, `digest_text.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use super::digest::Digest;
use super::digest_text::{parse_hex, HexText};

/// Test vector FIPS 180-2 untuk SHA-256("abc").
const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

/// Test vector FIPS 180-2 untuk SHA-256 string kosong.
const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

#[test]
fn hash_sesuai_test_vector_fips_180_2() {
    assert_eq!(Digest::of(b"abc").to_hex(), ABC_SHA256);
    assert_eq!(Digest::of(b"").to_hex(), EMPTY_SHA256);
}

#[test]
fn hex_bisa_diparse_kembali() {
    let digest = Digest::of(b"verge");
    assert_eq!(parse_hex(&digest.to_hex()).expect("hex valid"), digest);
}

#[test]
fn hex_tidak_kanonik_ditolak() {
    assert!(parse_hex("abc").is_err(), "panjang salah");
    assert!(parse_hex(&"z".repeat(64)).is_err(), "bukan hex");
    assert!(
        parse_hex(&"A".repeat(64)).is_err(),
        "huruf besar bukan kanonik"
    );
}

#[test]
fn input_berbeda_menghasilkan_id_berbeda() {
    assert_ne!(Digest::of(b"a"), Digest::of(b"b"));
}
