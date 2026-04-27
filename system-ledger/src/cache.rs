//! Recent-N checkpoint cache. Lookup by `(origin, tree_size)` and
//! `(origin, root_hash)`. LRU eviction. Skeleton — implementation
//! lands in the next commit per task #18.

use system_core::SignedCheckpoint;

/// Bounded checkpoint cache. Single-writer; not thread-safe.
pub struct CheckpointCache {
    capacity: usize,
    entries: Vec<SignedCheckpoint>,
}

impl CheckpointCache {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
