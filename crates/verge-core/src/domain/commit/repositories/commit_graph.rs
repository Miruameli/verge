//! File: `commit_graph.rs`
//!
//! Deskripsi: Repository `CommitGraph` — indeks commit, branch, dan tag.
//! Layer: domain/commit/repositories
//! Tanggung jawab: Menyimpan commit dan mengelola pointer branch/tag.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/commit/value-objects/commit_ref.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::collections::BTreeMap;

use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::commit::value_objects::commit_ref::{validate_name, Ref};
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Indeks in-memory atas seluruh commit beserta branch dan tag.
///
/// Invariants:
/// - Commit hanya bisa masuk bila seluruh parent-nya sudah ada; dengan begitu
///   setiap sejarah bisa ditelusuri tanpa bolak-balik.
/// - Branch boleh bergerak, tag tidak.
///
/// Immutability: commit immutable; pointer branch mutable; tag immutable.
#[derive(Debug, Clone, Default)]
pub struct CommitGraph {
    /// Commit berdasarkan identifier.
    commits: BTreeMap<CommitId, Commit>,
    /// Branch (pointer bergerak) berdasarkan nama.
    branches: BTreeMap<String, CommitId>,
    /// Tag (pointer immutable) berdasarkan nama.
    tags: BTreeMap<String, CommitId>,
}

impl CommitGraph {
    /// Membuat graph kosong.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Menambahkan commit ke dalam graph.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`MissingParent`](crate::VergeError::MissingParent) bila
    /// ada parent yang belum dikenal; commit yang ditolak tidak diindeks.
    pub fn insert(&mut self, commit: Commit) -> Result<()> {
        for parent in commit.parents() {
            if !self.commits.contains_key(parent) {
                return Err(VergeError::MissingParent(*parent));
            }
        }
        self.commits.insert(commit.id(), commit);
        Ok(())
    }

    /// Mengembalikan commit dengan identifier tertentu.
    #[must_use]
    pub fn commit(&self, id: &CommitId) -> Option<&Commit> {
        self.commits.get(id)
    }

    /// Mengembalikan jumlah commit yang diketahui.
    #[must_use]
    pub fn len(&self) -> usize {
        self.commits.len()
    }

    /// Melaporkan apakah graph tidak memuat commit apa pun.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commits.is_empty()
    }

    /// Mengarahkan branch `name` ke commit `id`.
    ///
    /// Branch hanya satu operasi tulis pointer: tidak ada data yang disalin,
    /// sehingga pembuatan branch O(1) terhadap ukuran data.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) untuk nama
    /// tidak valid dan [`CommitNotFound`](crate::VergeError::CommitNotFound)
    /// bila commit tidak dikenal.
    pub fn set_branch(&mut self, name: &str, id: CommitId) -> Result<()> {
        validate_name(name)?;
        if !self.commits.contains_key(&id) {
            return Err(VergeError::CommitNotFound(id));
        }
        self.branches.insert(name.to_owned(), id);
        Ok(())
    }

    /// Membuat tag `name` yang menunjuk commit `id`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`TagAlreadyExists`](crate::VergeError::TagAlreadyExists)
    /// bila nama sudah dipakai, [`InvalidName`](crate::VergeError::InvalidName)
    /// untuk nama tidak valid, dan
    /// [`CommitNotFound`](crate::VergeError::CommitNotFound) bila commit
    /// tidak dikenal.
    pub fn set_tag(&mut self, name: &str, id: CommitId) -> Result<()> {
        validate_name(name)?;
        if self.tags.contains_key(name) {
            return Err(VergeError::TagAlreadyExists(name.to_owned()));
        }
        if !self.commits.contains_key(&id) {
            return Err(VergeError::CommitNotFound(id));
        }
        self.tags.insert(name.to_owned(), id);
        Ok(())
    }

    /// Menyelesaikan referensi menjadi identifier commit.
    #[must_use]
    pub fn resolve(&self, reference: &Ref) -> Option<CommitId> {
        match reference {
            Ref::Branch(name) => self.branches.get(name).copied(),
            Ref::Tag(name) => self.tags.get(name).copied(),
            Ref::Commit(id) => self.commits.contains_key(id).then_some(*id),
        }
    }

    /// Mengembalikan seluruh nama branch terurut leksikografis.
    pub fn branch_names(&self) -> impl Iterator<Item = &str> {
        self.branches.keys().map(String::as_str)
    }

    /// Mengembalikan seluruh nama tag terurut leksikografis.
    pub fn tag_names(&self) -> impl Iterator<Item = &str> {
        self.tags.keys().map(String::as_str)
    }
}
