//! File: `world_ports.rs`
//!
//! Deskripsi: Implementasi port domain atop dunia in-memory.
//! Layer: application/version-control/fakes
//! Tanggung jawab: Menyartialkan port version control untuk test use case.
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
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)
//!   - ADR-0006 (Prolly tree untuk tabel)

use std::path::Path;

use crate::application::version_control::fakes::world::{FakeSource, FakeWorld};
use crate::domain::commit::codec::commit_decoding::decode;
use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::storage::value_objects::put_outcome::PutOutcome;
use crate::domain::table::ports::table_source::TableSource;
use crate::domain::table::ports::table_workspace::TableWorkspace;
use crate::domain::table::value_objects::table_name::TableName;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

impl TableSource for FakeSource<'_> {
    fn read_all(&self, _path: &Path) -> Result<Vec<u8>> {
        Ok(self.contents())
    }
}

impl TableWorkspace for FakeWorld {
    fn stage(&self, name: &TableName, root: BlockId) -> Result<()> {
        self.working
            .borrow_mut()
            .insert(name.as_str().to_owned(), root);
        Ok(())
    }

    fn staged(&self, name: &TableName) -> Result<Option<BlockId>> {
        Ok(self.working.borrow().get(name.as_str()).copied())
    }
}

impl Store for FakeWorld {
    fn put(&self, data: &[u8]) -> Result<PutOutcome> {
        let id = BlockId::of(data);
        let mut blocks = self.blocks.borrow_mut();
        let inserted = !blocks.contains_key(&id);
        blocks.entry(id).or_insert_with(|| data.to_vec());
        Ok(PutOutcome { id, inserted })
    }

    fn get(&self, id: &BlockId) -> Result<Vec<u8>> {
        self.blocks
            .borrow()
            .get(id)
            .cloned()
            .ok_or(VergeError::BlockNotFound {
                id: *id,
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "block"),
            })
    }

    fn contains(&self, id: &BlockId) -> bool {
        self.blocks.borrow().contains_key(id)
    }
}

impl CommitRepository for FakeWorld {
    fn save(&self, commit: &Commit) -> Result<PutOutcome> {
        let inserted = !self.commits.borrow().contains_key(&commit.id());
        self.commits
            .borrow_mut()
            .insert(commit.id(), commit.encode());
        Ok(PutOutcome {
            id: commit.id(),
            inserted,
        })
    }

    fn load(&self, id: &CommitId) -> Result<Commit> {
        let bytes = self
            .commits
            .borrow()
            .get(id)
            .cloned()
            .ok_or(VergeError::CommitNotFound(*id))?;
        decode(&bytes)
    }
}

impl RefPointer for FakeWorld {
    fn head_branch(&self) -> Result<String> {
        Ok(self.head.borrow().clone())
    }

    fn resolve(&self, branch: &str) -> Result<Option<CommitId>> {
        Ok(self.branches.borrow().get(branch).copied())
    }

    fn advance(&self, branch: &str, id: CommitId) -> Result<()> {
        self.branches.borrow_mut().insert(branch.to_owned(), id);
        Ok(())
    }
}
