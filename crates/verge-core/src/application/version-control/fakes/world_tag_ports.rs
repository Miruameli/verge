//! File: `world_tag_ports.rs`
//!
//! Deskripsi: Implementasi port tag atop dunia in-memory.
//! Layer: application/version-control/fakes
//! Tanggung jawab: Menyimpan pointer tag selama test, termasuk penolakan nama
//!   yang sudah dipakai.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `world.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::application::version_control::fakes::world::FakeWorld;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

impl TagPointer for FakeWorld {
    /// Menolak nama yang sudah dipakai: tag immutable, tidak pernah ditimpa.
    fn create(&self, name: &str, id: CommitId) -> Result<()> {
        let mut tags = self.tags.borrow_mut();
        if tags.contains_key(name) {
            return Err(VergeError::TagAlreadyExists(name.to_owned()));
        }
        tags.insert(name.to_owned(), id);
        Ok(())
    }

    fn resolve(&self, name: &str) -> Result<Option<CommitId>> {
        Ok(self.tags.borrow().get(name).copied())
    }

    fn tags(&self) -> Result<Vec<String>> {
        let mut names: Vec<String> = self.tags.borrow().keys().cloned().collect();
        names.sort();
        Ok(names)
    }

    fn delete(&self, name: &str) -> Result<()> {
        self.tags
            .borrow_mut()
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| VergeError::UnknownTag(name.to_owned()))
    }
}
