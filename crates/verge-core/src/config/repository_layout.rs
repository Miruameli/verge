//! File: `repository_layout.rs`
//!
//! Deskripsi: Konstanta layout direktori repository Verge.
//! Layer: config
//! Tanggung jawab: Menetapkan nama folder dan isi HEAD untuk repo lokal.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi eksternal)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::path::{Path, PathBuf};

/// Direktori internal repository di dalam folder kerja pengguna.
pub const REPO_DIR: &str = ".verge";

/// Direktori tempat seluruh blok disimpan.
pub const OBJECTS_DIR: &str = "objects";

/// Direktori yang memuat namespace pointer (`heads/`, `tags/`).
pub const REFS_DIR: &str = "refs";

/// Direktori pointer branch.
pub const HEADS_DIR: &str = "heads";

/// Direktori pointer tag.
pub const TAGS_DIR: &str = "tags";

/// Nama berkas yang menunjuk branch aktif.
pub const HEAD_FILE: &str = "HEAD";

/// Branch yang dipakai untuk repository baru.
pub const DEFAULT_BRANCH: &str = "main";

/// Isi `HEAD` untuk repository baru.
pub const HEAD_MAIN: &str = "ref: refs/heads/main\n";

/// Kumpulan path repository relatif terhadap folder kerja pengguna.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryLayout {
    /// Path direktori `.verge`.
    pub root: PathBuf,
}

impl RepositoryLayout {
    /// Menyusun layout untuk folder kerja `workspace`.
    #[must_use]
    pub fn under(workspace: impl AsRef<Path>) -> Self {
        Self {
            root: workspace.as_ref().join(REPO_DIR),
        }
    }

    /// Path direktori objek.
    #[must_use]
    pub fn objects(&self) -> PathBuf {
        self.root.join(OBJECTS_DIR)
    }

    /// Path direktori pointer branch.
    #[must_use]
    pub fn heads(&self) -> PathBuf {
        self.root.join(REFS_DIR).join(HEADS_DIR)
    }

    /// Path direktori pointer tag.
    #[must_use]
    pub fn tags(&self) -> PathBuf {
        self.root.join(REFS_DIR).join(TAGS_DIR)
    }

    /// Path berkas `HEAD`.
    #[must_use]
    pub fn head_file(&self) -> PathBuf {
        self.root.join(HEAD_FILE)
    }
}
