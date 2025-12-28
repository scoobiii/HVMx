// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: unified.rs
// Location: crates/hvmx-memory/src/unified.rs
// Purpose: Unified memory allocator for CPU+GPU
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

use std::collections::HashMap;
use anyhow::Result;
use crate::{MemoryError, Region, MemStats};

/// Unified memory allocator
/// 
/// Manages memory that is accessible from both CPU and GPU.
/// Critical for mobile SoCs (Snapdragon, Apple Silicon) that have
/// unified memory architecture.
pub struct UnifiedAllocator {
    regions: HashMap<usize, Region>,
    stats: MemStats,
    is_unified: bool,
    next_ptr: usize,
}

impl UnifiedAllocator {
    /// Create new unified allocator
    pub fn new(is_unified: bool) -> Self {
        Self {
            regions: HashMap::new(),
            stats: MemStats::new(),
            is_unified,
            next_ptr: 0x10000, // Start at 64KB
        }
    }

    /// Allocate unified memory region
    pub fn alloc(&mut self, size: usize, alignment: usize) -> Result<Region> {
        if !self.is_unified {
            return Err(MemoryError::NotUnified.into());
        }

        if !alignment.is_power_of_two() {
            return Err(MemoryError::InvalidAlignment(alignment).into());
        }

        // Align pointer
        let aligned_ptr = (self.next_ptr + alignment - 1) & !(alignment - 1);
        
        let region = Region {
            ptr: aligned_ptr,
            size,
            device_accessible: true,
            host_accessible: true,
        };

        self.regions.insert(aligned_ptr, region);
        self.stats.record_alloc(size);
        self.next_ptr = aligned_ptr + size;

        Ok(region)
    }

    /// Free memory region
    pub fn free(&mut self, ptr: usize) -> Result<()> {
        if let Some(region) = self.regions.remove(&ptr) {
            self.stats.record_dealloc(region.size);
            Ok(())
        } else {
            Err(MemoryError::NullPointer.into())
        }
    }

    /// Check if pointer is valid
    pub fn is_valid(&self, ptr: usize) -> bool {
        self.regions.contains_key(&ptr)
    }

    /// Get region info
    pub fn get_region(&self, ptr: usize) -> Option<&Region> {
        self.regions.get(&ptr)
    }

    /// Get memory statistics
    pub fn stats(&self) -> &MemStats {
        &self.stats
    }

    /// Check if allocator supports unified memory
    pub fn is_unified(&self) -> bool {
        self.is_unified
    }

    /// Prefetch region to GPU
    pub fn prefetch_to_device(&self, _ptr: usize) -> Result<()> {
        // TODO: Platform-specific prefetch hints
        Ok(())
    }

    /// Prefetch region to CPU
    pub fn prefetch_to_host(&self, _ptr: usize) -> Result<()> {
        // TODO: Platform-specific prefetch hints
        Ok(())
    }

    /// Get total allocated memory
    pub fn total_allocated(&self) -> usize {
        self.stats.allocated
    }

    /// Get peak memory usage
    pub fn peak_usage(&self) -> usize {
        self.stats.peak
    }
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_alloc() {
        let mut alloc = UnifiedAllocator::new(true);
        let region = alloc.alloc(4096, 16).unwrap();
        
        assert_eq!(region.size, 4096);
        assert!(region.is_unified());
        assert!(alloc.is_valid(region.ptr));
    }

    #[test]
    fn test_unified_alloc_alignment() {
        let mut alloc = UnifiedAllocator::new(true);
        let region = alloc.alloc(1024, 256).unwrap();
        
        assert_eq!(region.ptr % 256, 0); // Check alignment
    }

    #[test]
    fn test_unified_alloc_not_unified() {
        let mut alloc = UnifiedAllocator::new(false);
        let result = alloc.alloc(1024, 16);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_unified_free() {
        let mut alloc = UnifiedAllocator::new(true);
        let region = alloc.alloc(2048, 16).unwrap();
        
        assert!(alloc.is_valid(region.ptr));
        alloc.free(region.ptr).unwrap();
        assert!(!alloc.is_valid(region.ptr));
    }

    #[test]
    fn test_unified_stats() {
        let mut alloc = UnifiedAllocator::new(true);
        alloc.alloc(1024, 16).unwrap();
        alloc.alloc(2048, 16).unwrap();
        
        let stats = alloc.stats();
        assert_eq!(stats.allocated, 3072);
        assert_eq!(stats.allocations, 2);
    }

    #[test]
    fn test_unified_peak() {
        let mut alloc = UnifiedAllocator::new(true);
        let r1 = alloc.alloc(4096, 16).unwrap();
        alloc.alloc(8192, 16).unwrap();
        alloc.free(r1.ptr).unwrap();
        
        assert_eq!(alloc.peak_usage(), 12288);
        assert_eq!(alloc.total_allocated(), 8192);
    }

    #[test]
    fn test_unified_get_region() {
        let mut alloc = UnifiedAllocator::new(true);
        let region = alloc.alloc(1024, 16).unwrap();
        
        let retrieved = alloc.get_region(region.ptr).unwrap();
        assert_eq!(retrieved.size, 1024);
    }

    #[test]
    fn test_unified_invalid_alignment() {
        let mut alloc = UnifiedAllocator::new(true);
        let result = alloc.alloc(1024, 15); // Not power of 2
        
        assert!(result.is_err());
    }

    #[test]
    fn test_unified_multiple_allocs() {
        let mut alloc = UnifiedAllocator::new(true);
        let r1 = alloc.alloc(512, 8).unwrap();
        let r2 = alloc.alloc(1024, 8).unwrap();
        let r3 = alloc.alloc(2048, 8).unwrap();
        
        assert!(r2.ptr > r1.ptr);
        assert!(r3.ptr > r2.ptr);
    }

    #[test]
    fn test_unified_prefetch() {
        let mut alloc = UnifiedAllocator::new(true);
        let region = alloc.alloc(4096, 16).unwrap();
        
        assert!(alloc.prefetch_to_device(region.ptr).is_ok());
        assert!(alloc.prefetch_to_host(region.ptr).is_ok());
    }
}
