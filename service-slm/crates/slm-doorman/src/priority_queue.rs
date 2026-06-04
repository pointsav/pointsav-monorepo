// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Priority queue — three-level interleaved drain for background work.
//!
//! Background work is filed into one of three priority levels, each backed by
//! its own directory:
//!
//! - **P0** (`queue/p0-local/`) — lightweight classification that runs on the
//!   local Tier A model; no GPU required.
//! - **P1** (`queue/p1-batch/`) — DataGraph extraction and similar work that
//!   needs a GPU node and structured output.
//! - **P2** (`queue/p2-brief/`) — apprenticeship briefs, DPO generation, and
//!   training-corpus work; the lowest priority.
//!
//! ## Interleaved drain (the anti-starvation property)
//!
//! A naive FIFO over a single queue lets a large batch of P2 training briefs
//! block a single urgent P1 extraction for hours. The
//! [`PriorityQueue::next_drain_level`] cursor instead rotates P0 → P1 → P2 on
//! each call, skipping empty levels. One P2 item is drained, then the cursor
//! gives P0 and P1 a turn before the next P2. A backlog at one level never
//! starves the others.
//!
//! This module owns the *selection order* and the *directory layout*. It does
//! not own the lease/flock mechanics that guard against two workers grabbing
//! the same file — that lives in the server's `queue.rs`. The drain worker
//! calls [`PriorityQueue::next_drain_level`] to pick a level, then uses the
//! existing lease machinery against that level's directory.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, Ordering};

use serde::{Deserialize, Serialize};

use crate::error::{DoormanError, Result};

/// The three priority levels, highest priority first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    /// Local Tier A classification; no GPU.
    P0,
    /// GPU batch extraction.
    P1,
    /// Apprenticeship / training corpus.
    P2,
}

impl Priority {
    /// All levels in priority order.
    pub const ALL: [Priority; 3] = [Priority::P0, Priority::P1, Priority::P2];

    /// The subdirectory name for this level, relative to the queue root.
    pub fn dir_name(self) -> &'static str {
        match self {
            Priority::P0 => "p0-local",
            Priority::P1 => "p1-batch",
            Priority::P2 => "p2-brief",
        }
    }

    /// Parse the `X-Foundry-Priority` header value (`"p0"`, `"p1"`, `"p2"`),
    /// defaulting to `P2` for any unrecognized or absent value — background
    /// work without an explicit priority is treated as lowest priority.
    pub fn from_header(value: Option<&str>) -> Self {
        match value.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
            Some("p0") => Priority::P0,
            Some("p1") => Priority::P1,
            _ => Priority::P2,
        }
    }

    fn index(self) -> u8 {
        match self {
            Priority::P0 => 0,
            Priority::P1 => 1,
            Priority::P2 => 2,
        }
    }

    fn from_index(i: u8) -> Priority {
        match i % 3 {
            0 => Priority::P0,
            1 => Priority::P1,
            _ => Priority::P2,
        }
    }
}

/// A count of pending items at each level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueDepth {
    pub p0: usize,
    pub p1: usize,
    pub p2: usize,
}

impl QueueDepth {
    /// Total pending items across all levels.
    pub fn total(&self) -> usize {
        self.p0 + self.p1 + self.p2
    }

    /// True when every level is empty.
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    fn at(&self, level: Priority) -> usize {
        match level {
            Priority::P0 => self.p0,
            Priority::P1 => self.p1,
            Priority::P2 => self.p2,
        }
    }
}

/// The priority queue. Owns the three level directories under a single root
/// and a rotating cursor for interleaved drain.
///
/// Cloneable via `Arc`. The cursor is an [`AtomicU8`] so the drain worker can
/// advance it without a mutex.
#[derive(Debug)]
pub struct PriorityQueue {
    root: PathBuf,
    /// The next level to *consider* draining. Rotates 0→1→2→0.
    cursor: AtomicU8,
}

impl PriorityQueue {
    /// Open (creating if needed) the three level directories under `root`.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        for level in Priority::ALL {
            let dir = root.join(level.dir_name());
            std::fs::create_dir_all(&dir).map_err(|e| DoormanError::PriorityQueueIo {
                path: dir.display().to_string(),
                reason: format!("create priority queue dir: {e}"),
            })?;
        }
        Ok(Self {
            root,
            cursor: AtomicU8::new(0),
        })
    }

    /// The absolute directory for a level.
    pub fn level_dir(&self, level: Priority) -> PathBuf {
        self.root.join(level.dir_name())
    }

    /// Count pending `.jsonl` files at each level. A "pending" file is any
    /// regular file ending in `.jsonl` (lease/temp files use other suffixes).
    pub fn depth(&self) -> QueueDepth {
        QueueDepth {
            p0: self.count_level(Priority::P0),
            p1: self.count_level(Priority::P1),
            p2: self.count_level(Priority::P2),
        }
    }

    /// Pick the next level to drain, honoring interleaving and skipping empty
    /// levels, then advance the cursor. Returns `None` when all levels are
    /// empty.
    ///
    /// The cursor starts at the position after the last returned level, so
    /// successive calls rotate P0 → P1 → P2 → P0. If the cursor lands on an
    /// empty level, it advances (up to three probes) to the next non-empty
    /// level. Higher-priority work is therefore never blocked behind a large
    /// lower-priority backlog: each full rotation gives every non-empty level
    /// exactly one turn.
    pub fn next_drain_level(&self) -> Option<Priority> {
        let depth = self.depth();
        if depth.is_empty() {
            return None;
        }
        let start = self.cursor.load(Ordering::SeqCst);
        for offset in 0..3u8 {
            let level = Priority::from_index(start.wrapping_add(offset));
            if depth.at(level) > 0 {
                // Advance the cursor to one past the chosen level.
                self.cursor
                    .store(level.index().wrapping_add(1) % 3, Ordering::SeqCst);
                return Some(level);
            }
        }
        None
    }

    /// Write a payload to the given level as `<id>.jsonl`. The caller supplies
    /// a unique id (e.g. a brief id or ULID). Overwrites an existing file with
    /// the same id (last-writer-wins for same-content retries).
    pub fn enqueue(&self, level: Priority, id: &str, payload: &[u8]) -> Result<PathBuf> {
        let dir = self.level_dir(level);
        let path = dir.join(format!("{id}.jsonl"));
        // Write to a temp file then rename for atomicity (no half-written
        // file is ever visible to the drain worker).
        let tmp = dir.join(format!(".{id}.jsonl.tmp"));
        std::fs::write(&tmp, payload).map_err(|e| DoormanError::PriorityQueueIo {
            path: tmp.display().to_string(),
            reason: format!("write temp: {e}"),
        })?;
        std::fs::rename(&tmp, &path).map_err(|e| DoormanError::PriorityQueueIo {
            path: path.display().to_string(),
            reason: format!("rename temp into place: {e}"),
        })?;
        Ok(path)
    }

    /// Return the path of the oldest (lexicographically-first) pending file at
    /// a level, or `None` if the level is empty. Filenames are expected to be
    /// time-sortable (ULID / ISO timestamp prefix), so lexicographic order is
    /// FIFO within a level. Does NOT remove the file — the caller leases it
    /// via the server's lease machinery.
    pub fn peek_level(&self, level: Priority) -> Option<PathBuf> {
        let dir = self.level_dir(level);
        let mut names: Vec<PathBuf> = std::fs::read_dir(&dir)
            .ok()?
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.extension().and_then(|s| s.to_str()) == Some("jsonl")
                    && !p
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with('.'))
                        .unwrap_or(false)
            })
            .collect();
        names.sort();
        names.into_iter().next()
    }

    fn count_level(&self, level: Priority) -> usize {
        let dir = self.level_dir(level);
        std::fs::read_dir(&dir)
            .map(|rd| {
                rd.flatten()
                    .filter(|e| {
                        let p = e.path();
                        p.extension().and_then(|s| s.to_str()) == Some("jsonl")
                            && !e
                                .file_name()
                                .to_str()
                                .map(|n| n.starts_with('.'))
                                .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_root(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "slm-prio-queue-{label}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn open_creates_three_level_dirs() {
        let root = tmp_root("open");
        let q = PriorityQueue::open(&root).unwrap();
        for level in Priority::ALL {
            assert!(q.level_dir(level).is_dir());
        }
    }

    #[test]
    fn empty_queue_drains_nothing() {
        let root = tmp_root("empty");
        let q = PriorityQueue::open(&root).unwrap();
        assert!(q.depth().is_empty());
        assert_eq!(q.next_drain_level(), None);
    }

    #[test]
    fn enqueue_increments_depth() {
        let root = tmp_root("depth");
        let q = PriorityQueue::open(&root).unwrap();
        q.enqueue(Priority::P0, "a", b"{}").unwrap();
        q.enqueue(Priority::P1, "b", b"{}").unwrap();
        q.enqueue(Priority::P2, "c", b"{}").unwrap();
        q.enqueue(Priority::P2, "d", b"{}").unwrap();
        let depth = q.depth();
        assert_eq!(depth.p0, 1);
        assert_eq!(depth.p1, 1);
        assert_eq!(depth.p2, 2);
        assert_eq!(depth.total(), 4);
    }

    #[test]
    fn interleaved_drain_rotates_p0_p1_p2() {
        let root = tmp_root("interleave");
        let q = PriorityQueue::open(&root).unwrap();
        // One item at each level.
        q.enqueue(Priority::P0, "a", b"{}").unwrap();
        q.enqueue(Priority::P1, "b", b"{}").unwrap();
        q.enqueue(Priority::P2, "c", b"{}").unwrap();
        // Without removing items, the cursor still rotates the *consideration*
        // order. We assert the order of levels selected across three calls.
        assert_eq!(q.next_drain_level(), Some(Priority::P0));
        assert_eq!(q.next_drain_level(), Some(Priority::P1));
        assert_eq!(q.next_drain_level(), Some(Priority::P2));
        // And wraps back to P0.
        assert_eq!(q.next_drain_level(), Some(Priority::P0));
    }

    #[test]
    fn p0_not_starved_by_large_p2_backlog() {
        // The anti-starvation property: a big P2 backlog plus one P0 item.
        // The P0 item must be selected within one full rotation, not after
        // the entire P2 backlog.
        let root = tmp_root("starve");
        let q = PriorityQueue::open(&root).unwrap();
        for i in 0..100 {
            q.enqueue(Priority::P2, &format!("p2-{i:03}"), b"{}")
                .unwrap();
        }
        q.enqueue(Priority::P0, "urgent", b"{}").unwrap();
        // Within the first three selections, P0 must appear.
        let picks: Vec<_> = (0..3).filter_map(|_| q.next_drain_level()).collect();
        assert!(
            picks.contains(&Priority::P0),
            "P0 must be selected within one rotation, got {picks:?}"
        );
    }

    #[test]
    fn empty_levels_are_skipped() {
        let root = tmp_root("skip");
        let q = PriorityQueue::open(&root).unwrap();
        // Only P2 has work.
        q.enqueue(Priority::P2, "only", b"{}").unwrap();
        // Every call must return P2 (P0 and P1 are empty and skipped).
        assert_eq!(q.next_drain_level(), Some(Priority::P2));
        assert_eq!(q.next_drain_level(), Some(Priority::P2));
    }

    #[test]
    fn peek_returns_lexicographically_first() {
        let root = tmp_root("peek");
        let q = PriorityQueue::open(&root).unwrap();
        q.enqueue(Priority::P1, "20260604-b", b"{}").unwrap();
        q.enqueue(Priority::P1, "20260604-a", b"{}").unwrap();
        let first = q.peek_level(Priority::P1).unwrap();
        assert!(first.file_name().unwrap().to_str().unwrap().contains("-a"));
    }

    #[test]
    fn enqueue_is_atomic_no_temp_left_behind() {
        let root = tmp_root("atomic");
        let q = PriorityQueue::open(&root).unwrap();
        q.enqueue(Priority::P0, "x", b"{}").unwrap();
        // No .tmp file should remain.
        let leftovers: Vec<_> = std::fs::read_dir(q.level_dir(Priority::P0))
            .unwrap()
            .flatten()
            .filter(|e| e.file_name().to_str().unwrap().ends_with(".tmp"))
            .collect();
        assert!(leftovers.is_empty());
        // And the depth counter ignores dotfiles/temp anyway.
        assert_eq!(q.depth().p0, 1);
    }

    #[test]
    fn priority_header_parsing() {
        assert_eq!(Priority::from_header(Some("p0")), Priority::P0);
        assert_eq!(Priority::from_header(Some("P1")), Priority::P1);
        assert_eq!(Priority::from_header(Some("p2")), Priority::P2);
        assert_eq!(Priority::from_header(None), Priority::P2);
        assert_eq!(Priority::from_header(Some("weird")), Priority::P2);
    }
}
