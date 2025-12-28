// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: lib.rs
// Location: crates/hvmx-memory/src/lib.rs
// Purpose: Memory management root module
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

pub mod unified;
pub mod prefetch;
pub mod tile;

pub use unified::UnifiedAllocator;
pub use prefetch::{PrefetchStrategy, PrefetchHint};
pub use tile::TileConfig;

use thiserror::Error;

/// Memory management errors
#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Out of memory: requested {0} bytes")]
    OutOfMemory(usize),
    
    #[error("Invalid alignment: {0}")]
    InvalidAlignment(usize),
    
    #[error("Null pointer dereference")]
    NullPointer,
    
    #[error("Memory not unified on this device")]
    NotUnified,
}

/// Memory region descriptor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    pub ptr: usize,
    pub size: usize,
    pub device_accessible: bool,
    pub host_accessible: bool,
}

impl Region {
    pub fn new(ptr: usize, size: usize) -> Self {
        Self {
            ptr,
            size,
            device_accessible: true,
            host_accessible: true,
        }
    }
    
    pub fn is_unified(&self) -> bool {
        self.device_accessible && self.host_accessible
    }
}

/// Memory statistics
#[derive(Debug, Clone, Default)]
pub struct MemStats {
    pub allocated: usize,
    pub peak: usize,
    pub allocations: u64,
    pub deallocations: u64,
}

impl MemStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn record_alloc(&mut self, size: usize) {
        self.allocated += size;
        self.allocations += 1;
        if self.allocated > self.peak {
            self.peak = self.allocated;
        }
    }
    
    pub fn record_dealloc(&mut self, size: usize) {
        self.allocated = self.allocated.saturating_sub(size);
        self.deallocations += 1;
    }
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_creation() {
        let region = Region::new(0x1000, 4096);
        assert_eq!(region.ptr, 0x1000);
        assert_eq!(region.size, 4096);
        assert!(region.is_unified());
    }

    #[test]
    fn test_region_unified() {
        let mut region = Region::new(0x2000, 8192);
        assert!(region.is_unified());
        
        region.host_accessible = false;
        assert!(!region.is_unified());
    }

    #[test]
    fn test_memstats_alloc() {
        let mut stats = MemStats::new();
        stats.record_alloc(1024);
        
        assert_eq!(stats.allocated, 1024);
        assert_eq!(stats.peak, 1024);
        assert_eq!(stats.allocations, 1);
    }

    #[test]
    fn test_memstats_peak() {
        let mut stats = MemStats::new();
        stats.record_alloc(1024);
        stats.record_alloc(2048);
        stats.record_dealloc(1024);
        
        assert_eq!(stats.allocated, 2048);
        assert_eq!(stats.peak, 3072);
    }

    #[test]
    fn test_memstats_dealloc() {
        let mut stats = MemStats::new();
        stats.record_alloc(4096);
        stats.record_dealloc(2048);
        
        assert_eq!(stats.allocated, 2048);
        assert_eq!(stats.deallocations, 1);
    }
}
