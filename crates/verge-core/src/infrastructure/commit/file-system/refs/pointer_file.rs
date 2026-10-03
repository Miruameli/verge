//! File: `pointer_file.rs`
//!
//! Deskripsi: Pembacaan dan penulisan berkas pointer branch.
//! Layer: infrastructure/commit/file-system
//! Tanggung jawab: Menyediakan operasi I/O atomik atas satu berkas pointer.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/branch_name_policy.rs`
//!   - `shared/exceptions/verge_error.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use std::fs;
use std::path::Path;

use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::ident::value_objects::digest_text::parse_hex;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Membaca isi pointer; `Ok(None)` bila berkasnya belum ada.
///
/// Args:
/// - path — path berkas pointer.
///
/// Returns:
/// - Ok(Option<String>) — isi pointer tanpa baris baru akhir, atau `None`.
///
/// # Errors
///
/// Mengembalikan [`Io`](VergeError::Io) bila berkas ada tapi tidak dapat dibaca.
pub fn read(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(raw) => Ok(Some(raw.trim_end_matches(['\n', '\r']).to_owned())),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(source.into()),
    }
}

/// Mem-parse isi pointer menjadi `CommitId`.
///
/// # Errors
///
/// Mengembalikan [`MalformedPointer`](VergeError::MalformedPointer) bila isi
/// bukan hex huruf kecil sepanjang tepat 64 karakter.
pub fn parse(raw: &str) -> Result<CommitId> {
    parse_hex(raw).map_err(|_| VergeError::MalformedPointer(raw.to_owned()))
}

/// Menghapus berkas pointer `path`.
///
/// # Errors
///
/// Mengembalikan [`Io`](VergeError::Io) bila berkas tidak dapat dihapus dan
/// bukan [`NotFound`](std::io::ErrorKind::NotFound).
pub fn remove(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
            Err(VergeError::UnknownBranch(display_name(path)))
        }
        Err(source) => Err(source.into()),
    }
}
/// Penanda berkas sementara yang tidak pernah dibaca sebagai branch.
pub const TEMP_PREFIX: &str = "pointer-tmp";

/// Akhiran berkas sementara, dipakai saat menyaring sisa penulisan gagal.
pub const TEMP_SUFFIX: &str = ".tmp";

/// Penomoran sementara agar dua tulisan dalam satu proses tidak berebut nama.
static TEMP_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Menulis pointer secara atomik: temp file + rename di direktori yang sama.
///
/// KONTEKS: `fs::write` langsung bisa meninggalkan pointer kosong bila proses
/// mati di tengah menulis; ALTERNATIF: nama sementara diturunkan dari isi
/// pointer, tetapi `HEAD` berisi `ref: refs/heads/<nama>` sehingga nama itu
/// menjadi path bersarang yang tidak pernah ada.
///
/// # Errors
///
/// Mengembalikan [`Io`](VergeError::Io) bila direktori induk tidak dapat
/// dibuat atau penulisan/rename gagal.
pub fn write(path: &Path, content: &str) -> Result<()> {
    let Some(parent) = path.parent() else {
        return Err(VergeError::Io(std::io::Error::other("tanpa induk")));
    };
    fs::create_dir_all(parent)?;
    let temp = parent.join(temp_name());
    fs::write(&temp, format!("{content}\n")).and_then(|()| fs::rename(&temp, path))?;
    Ok(())
}

/// Menulis pointer baru dan menolak bila nama sudah dipakai.
///
/// KONTEKS: pemeriksaan "sudah ada" lalu tulis (`read` lalu `write`) raced —
/// dua proses bisa sama-sama lolos pemeriksaan lalu menimpa pointer yang sama.
/// `create_new` membuat pembuatan berkas bersifat atomik di kernel, sehingga
/// hanya satu penulis yang berhasil. ALTERNATIF: `hard_link` dari berkas
/// sementara; ditolak karena tidak didukung di semua filesystem yang dipakai.
///
/// Returns:
/// - `Ok(true)` — pointer ditulis.
/// - `Ok(false)` — pointer sudah ada dan tidak disentuh.
///
/// # Errors
///
/// Mengembalikan [`Io`](VergeError::Io) bila direktori induk tidak dapat dibuat
/// atau penulisan gagal.
pub fn write_new(path: &Path, content: &str) -> Result<bool> {
    let Some(parent) = path.parent() else {
        return Err(VergeError::Io(std::io::Error::other("tanpa induk")));
    };
    fs::create_dir_all(parent)?;
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(mut file) => {
            std::io::Write::write_all(&mut file, format!("{content}\n").as_bytes())?;
            Ok(true)
        }
        Err(source) if source.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(source) => Err(source.into()),
    }
}

/// Mengembalikan nama berkas sementara yang unik dalam satu proses.
fn temp_name() -> String {
    let sequence = TEMP_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{TEMP_PREFIX}-{}-{sequence}.tmp", std::process::id())
}

/// Mengembalikan nama terakhir segmen `path` sebagai pesan galat.
fn display_name(path: &Path) -> String {
    path.file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
}
