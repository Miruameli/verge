//! File: `pointer_dir.rs`
//!
//! Deskripsi: Penemuan dan pengalamatan pointer branch dalam satu direktori.
//! Layer: infrastructure/commit/file-system
//! Tanggung jawab: Mendaftarkan nama branch yang sah dan menyusun path pointer.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/branch_name_policy.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::commit::value_objects::branch_name_policy::validate_name;
use crate::infrastructure::commit::file_system::pointer_file::{TEMP_PREFIX, TEMP_SUFFIX};
use crate::shared::kernel::result::Result;

/// Mengembalikan nama branch yang punya pointer di `dir`, terurut menaik.
///
/// KONTEKS: berkas `.tmp-` dari penulisan yang gagal sengaja diabaikan; KENAPA:
/// entri sementara bukan branch dan tidak boleh muncul di keluaran pengguna.
///
/// # Errors
///
/// Mengembalikan [`Io`](VergeError::Io) bila direktori tidak dapat dibaca dan
/// [`InvalidName`](VergeError::InvalidName) bila ada entri yang bukan nama
/// branch valid, karena itu berarti direktori sudah rusak oleh tool lain.
pub fn list(dir: &Path) -> Result<Vec<String>> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => return Err(source.into()),
    };
    let mut names = Vec::new();
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(TEMP_PREFIX) || name.ends_with(TEMP_SUFFIX) {
            continue;
        }
        validate_name(&name)?;
        names.push(name);
    }
    names.sort_unstable();
    Ok(names)
}

/// Mengembalikan path pointer `branch` di dalam `heads` setelah nama divalidasi.
///
/// # Errors
///
/// Mengembalikan [`InvalidName`](VergeError::InvalidName) bila nama tidak aman
/// dipakai sebagai satu segmen path.
pub fn path_for(heads: &Path, branch: &str) -> Result<PathBuf> {
    validate_name(branch)?;
    Ok(heads.join(branch))
}
