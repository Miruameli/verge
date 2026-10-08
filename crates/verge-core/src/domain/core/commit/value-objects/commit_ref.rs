//! File: `commit_ref.rs`
//!
//! Deskripsi: Value object `Ref` — pointer bernama ke dalam commit graph.
//! Layer: domain/commit/value-objects
//! Tanggung jawab: Membedakan branch (movable), tag (immutable), dan commit id.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/ident/value-objects/digest_text.rs
//!   - shared/kernel/result.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest_text::parse_hex;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Pointer bernama ke dalam commit graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ref {
    /// Pointer bergerak ke ujung sebuah garis kerja.
    Branch(String),
    /// Pointer immutable ke tepat satu commit.
    Tag(String),
    /// Identifier commit mentah.
    Commit(CommitId),
}

impl Ref {
    /// Mem-parse `heads/<name>`, `tags/<name>`, atau digest 64 karakter.
    ///
    /// Args:
    /// - text — referensi terkuotasi; nama telanjang ditolak agar ambigu.
    ///
    /// Returns:
    /// - Ok(Ref) — referensi valid.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) untuk nama
    /// kosong/berawalan titik, [`InvalidRef`](crate::VergeError::InvalidRef)
    /// untuk namespace yang tidak dikenal, dan
    /// [`InvalidDigest`](crate::VergeError::InvalidDigest) untuk teks non-digest.
    ///
    /// Example:
    /// ```
    /// use verge_core::Ref;
    ///
    /// assert_eq!(Ref::parse("heads/main").expect("branch"), Ref::Branch("main".into()));
    /// assert!(Ref::parse("main").is_err(), "nama telanjang ambigu");
    /// ```
    pub fn parse(text: &str) -> Result<Self> {
        match text.split_once('/') {
            Some(("heads", name)) => Self::branch(name.to_owned()),
            Some(("tags", name)) => Self::tag(name.to_owned()),
            Some(_) => Err(VergeError::InvalidRef(text.to_owned())),
            None => Ok(Self::Commit(parse_hex(text)?)),
        }
    }

    /// Membuat referensi branch setelah memvalidasi nama.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) bila nama
    /// kosong, diawali titik, atau memuat path separator.
    pub fn branch(name: String) -> Result<Self> {
        validate_name(&name)?;
        Ok(Self::Branch(name))
    }

    /// Membuat referensi tag setelah memvalidasi nama.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) bila nama
    /// kosong, diawali titik, atau memuat path separator.
    pub fn tag(name: String) -> Result<Self> {
        validate_name(&name)?;
        Ok(Self::Tag(name))
    }

    /// Mengembalikan nama polos untuk branch/tag, atau `None` untuk commit id.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Branch(name) | Self::Tag(name) => Some(name),
            Self::Commit(_) => None,
        }
    }
}

/// Memvalidasi nama branch/tag yang boleh dipakai pada path refs.
///
/// Aturan nama hidup di modul sendiri supaya berkas ini tetap berfokus pada
/// tipe `Ref`; path publiknya tidak berubah lewat re-export ini.
pub use crate::domain::commit::value_objects::branch_name_policy::validate_name;
