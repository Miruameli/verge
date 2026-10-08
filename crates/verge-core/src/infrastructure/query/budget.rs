//! File: `budget.rs`
//!
//! Deskripsi: Resource budget untuk SQL query execution — melacak
//!   penggunaan memori dan deadline waktu per-row.
//! Layer: infrastructure/query
//! Tanggung jawab: Memberi batas pada eksekusi query agar tidak
//!   menghabiskan memori atau berjalan selamanya.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #90 (M5 Part 3: executor resource limits)
//!   - #92 (M5 Part 2: SQL executor)
//!
//! Related ADR:
//!   - ADR-0014 (Executor resource limits)

use std::time::{Duration, Instant};

use crate::shared::exceptions::verge_error::VergeError;

/// Batas memori default: 64 MiB (issue #90).
const DEFAULT_MAX_MEMORY: usize = 64 * 1024 * 1024;

/// Batas waktu default: 10 detik (issue #90).
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// Batas sumber daya untuk query SQL eksekusi.
///
/// Melacak akumulasi memori keluaran dan deadline waktu.  Di-check
/// per-baris di `execute_plan` dan per-commit di `build_commit_rows`.
/// Batas default: 64 MiB memori, 10 detik waktu (issue #90).
#[derive(Debug, Clone)]
pub struct ScanBudget {
    /// Batas memori dalam byte.
    max_memory: usize,
    /// Deadline waktu eksekusi.
    deadline: Instant,
    /// Akumulasi byte output yang ditulis.
    memory_used: usize,
}

impl ScanBudget {
    /// Membuat budget dengan batas memori dan timeout default (issue #90).
    #[must_use]
    pub fn new() -> Self {
        Self {
            max_memory: DEFAULT_MAX_MEMORY,
            deadline: Instant::now() + DEFAULT_TIMEOUT,
            memory_used: 0,
        }
    }

    /// Membuat budget kosong (tanpa batas) untuk test/verification.
    #[must_use]
    pub fn unlimited() -> Self {
        Self {
            max_memory: usize::MAX,
            deadline: Instant::now() + Duration::from_secs(3600),
            memory_used: 0,
        }
    }

    /// Membuat budget dengan batas kustom.
    #[must_use]
    pub fn with_limits(max_memory: usize, timeout: Duration) -> Self {
        Self {
            max_memory,
            deadline: Instant::now() + timeout,
            memory_used: 0,
        }
    }

    /// Mencatat byte output yang ditulis dan memeriksa limit memori.
    ///
    /// # Errors
    ///
    /// Returns `VergeError::QueryResourceLimit` if memory limit exceeded.
    pub fn track_memory(&mut self, bytes: usize) -> Result<(), VergeError> {
        self.memory_used += bytes;
        if self.memory_used > self.max_memory {
            return Err(VergeError::QueryResourceLimit {
                limit_type: "memory",
                detail: format!(
                    "output exceeds {} bytes after writing {}",
                    self.max_memory, self.memory_used
                ),
            });
        }
        Ok(())
    }

    /// Memeriksa apakah deadline waktu masih valid.
    ///
    /// # Errors
    ///
    /// Returns `VergeError::QueryResourceLimit` if timeout exceeded.
    pub fn check_time(&self) -> Result<(), VergeError> {
        if Instant::now() > self.deadline {
            return Err(VergeError::QueryResourceLimit {
                limit_type: "time",
                detail: "query exceeded configured time limit".to_string(),
            });
        }
        Ok(())
    }
}

impl Default for ScanBudget {
    fn default() -> Self {
        Self::new()
    }
}
