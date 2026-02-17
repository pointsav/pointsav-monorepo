use system_core::{PointSavResult, SystemError};

/// Represents a physical memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
}

impl MemoryRegion {
    /// Checks if another region is perfectly adjacent to this one.
    pub fn is_adjacent(&self, other: &MemoryRegion) -> bool {
        self.end + 1 == other.start || other.end + 1 == self.start
    }

    /// Merges two adjacent regions into one to solve seL4 toolchain overlap errors.
    pub fn merge(&self, other: &MemoryRegion) -> PointSavResult<MemoryRegion> {
        if !self.is_adjacent(other) {
            return Err(SystemError::MemoryMapInvalid);
        }
        Ok(MemoryRegion {
            start: core::cmp::min(self.start, other.start),
            end: core::cmp::max(self.end, other.end),
        })
    }
}
