//! File: `branch_name_policy.rs`
//!
//! Deskripsi: Aturan nama branch dan tag yang aman sebagai segmen path.
//! Layer: domain/commit/value-objects
//! Tanggung jawab: Menolak nama yang tidak aman di semua platform.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `shared/exceptions/verge_error.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Karakter yang ditolak Windows atau ambigu pada sebagian filesystem.
///
/// ALTERNATIF: menerima semua karakter selain separator path; ditolak karena
/// hasil validasi akan berbeda antar platform untuk input yang sama.
const FORBIDDEN_NAME_CHARS: [char; 7] = ['<', '>', ':', '"', '|', '?', '*'];

/// Nama device tercadang Windows yang tidak boleh dipakai sebagai nama berkas.
const RESERVED_DEVICE_NAMES: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// Memvalidasi nama branch/tag yang boleh dipakai pada path `refs/`.
///
/// KONTEKS: nama branch menjadi nama berkas pointer di `refs/heads/`, jadi nama
/// harus aman bukan hanya di Linux; KENAPA: nama yang ditolak Windows membuat
/// repository hasil transfer antar platform gagal dibuka, dan nama device
/// tercadang membuat penulisan pointer gagal dengan pesan yang membingungkan.
///
/// Args:
/// - name — kandidat nama.
///
/// Returns:
/// - Ok(()) — nama aman dipakai sebagai satu segmen path.
///
/// # Errors
///
/// Mengembalikan [`InvalidName`](VergeError::InvalidName) bila nama kosong,
/// diawali titik, memuat separator path atau karakter terlarang, diakhiri titik
/// atau spasi, atau merupakan nama device tercadang.
///
/// Example:
/// ```
/// use verge_core::domain::commit::value_objects::branch_name_policy::validate_name;
///
/// assert!(validate_name("feature/menyaring").is_err());
/// assert!(validate_name("NUL").is_err());
/// assert!(validate_name("percobaan-1").is_ok());
/// ```
pub fn validate_name(name: &str) -> Result<()> {
    if is_rejected(name) {
        return Err(VergeError::InvalidName(name.to_owned()));
    }
    Ok(())
}

/// Mengembalikan `true` bila `name` tidak boleh dipakai sebagai nama berkas.
///
/// Fungsi ini murni supaya aturan nama bisa diuji tanpa lewat `Result`; tidak
/// ada alokasi sehingga aman dipanggil di jalur validasi yang sering.
#[must_use]
pub fn is_rejected(name: &str) -> bool {
    name.is_empty()
        || name.starts_with('.')
        || name.contains('/')
        || name.contains('\\')
        || name
            .chars()
            .any(|ch| ch.is_control() || FORBIDDEN_NAME_CHARS.contains(&ch))
        || name.ends_with('.')
        || name.ends_with(' ')
        || is_reserved_device_name(name)
}

/// Mengembalikan `true` bila bagian sebelum titik pertama adalah nama device
/// tercadang; comparing ASCII case-insensitive mengikuti aturan Windows.
fn is_reserved_device_name(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name);
    RESERVED_DEVICE_NAMES
        .iter()
        .any(|reserved| stem.eq_ignore_ascii_case(reserved))
}
