//! File: `tag_command.rs`
//!
//! Deskripsi: Use case pembuatan dan penghapusan tag.
//! Layer: application/version-control/use-cases/refs/tagging
//! Tanggung jawab: Menerjemahkan perintah tag menjadi perubahan pointer.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/{ref_pointer,tag_pointer}.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::application::version_control::dtos::tag_list::TagList;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::application::version_control::revision_target::{self, RevisionTarget};
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::commit::value_objects::branch_name_policy::validate_name;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Membuat tag `name` yang menunjuk commit hasil resolusi `revision`.
///
/// KENAPA nama divalidasi di sini juga: tag menjadi nama berkas di `refs/tags/`,
/// sehingga nama yang tidak aman harus ditolak sebelum menyentuh disk.
///
/// # Errors
///
/// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) untuk nama
/// yang tidak aman sebagai segmen path,
/// [`TagAlreadyExists`](crate::VergeError::TagAlreadyExists) bila nama sudah
/// dipakai — tag tidak pernah ditimpa — dan error dari port revisi.
pub fn create_tag(
    name: &str,
    revision: &str,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
) -> Result<()> {
    validate_name(name)?;
    reject_instant(revision)?;
    let id = resolve_revision(revision, refs, tags, commits, None)?;
    tags.create(name, id)
}

/// Menghapus tag `name`.
///
/// # Errors
///
/// Mengembalikan [`InvalidName`](crate::VergeError::InvalidName) untuk nama tidak
/// aman dan [`UnknownTag`](crate::VergeError::UnknownTag) bila tag belum ada.
pub fn delete_tag(name: &str, tags: &dyn TagPointer) -> Result<()> {
    validate_name(name)?;
    tags.delete(name)
}

/// Menyusun daftar tag beserta commit yang ditunjuknya.
///
/// # Errors
///
/// Mengembalikan error dari port tag maupun commit.
pub fn list_tags(
    names: &[String],
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
) -> Result<TagList> {
    let mut entries = Vec::with_capacity(names.len());
    for name in names {
        if let Some(commit) = tags.resolve(name)? {
            let target = commits.load(&commit)?;
            entries.push((name.clone(), commit, target.summary().to_owned()));
        }
    }
    Ok(TagList { entries })
}

/// Menolak revisi berupa waktu UTC saat membuat tag.
///
/// KENAPA: tag harus menunjuk satu commit tertentu, sedangkan `AS OF <waktu>`
/// belum punya jawaban tanpa tabel — tag tidak punya konsep tabel. Menolaknya
/// di sini memberi pesan yang bisa ditindaklanjuti, bukan `unknown revision`.
fn reject_instant(revision: &str) -> Result<()> {
    let is_instant = matches!(
        revision_target::classify(revision)?,
        RevisionTarget::Instant(_)
    );
    if is_instant {
        return Err(VergeError::TagCannotPointAtInstant(
            revision.trim().to_owned(),
        ));
    }
    Ok(())
}
