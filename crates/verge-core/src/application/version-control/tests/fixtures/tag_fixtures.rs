//! File: `tag_fixtures.rs`
//!
//! Deskripsi: Helper bersama untuk test use case tag.
//! Layer: application/version-control/tests/fixtures
//! Tanggung jawab: Menyediakan dunia ber-commit tunggal dan konversi hex ke
//!   identifier agar setiap berkas test tag tidak menyiapkan dunia sendiri.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_timeline.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use super::commit_timeline::{commit_at, step};
use crate::application::version_control::fakes::world::FakeWorld;
use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::ident::value_objects::digest_text::{parse_hex, HexText};

/// Membangun dunia dengan satu commit tabel `users`.
///
/// KONTEKS: seluruh test tag butuh tepat satu commit sebagai target, sehingga
/// penyiapan dunia dikumpulkan di sini agar tidak diulang per berkas test.
pub fn one_commit() -> (FakeWorld, String) {
    let world = FakeWorld::new();
    let id = commit_at(&world, "id,name\n1,ana\n", step(0));
    (world, id.to_hex())
}

/// Mengubah hex fixture menjadi identifier commit.
pub fn commit_of(id: &str) -> Digest {
    parse_hex(id).expect("id fixture adalah hex")
}
