// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: mod.rs
// Location: crates/hvmx-jit/src/backend/mod.rs
// Purpose: GPU backend module exports
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "metal")]
pub mod metal;

#[cfg(feature = "cuda")]
pub mod cuda;

pub mod cpu;

// Re-exports
#[cfg(feature = "vulkan")]
pub use vulkan::VulkanBackend;

use crate::runtime::{GPUBackend, GPUInfo, GPUVendor};

/// Detect available GPU backend
pub fn detect_backend() -> Option<GPUVendor> {
    #[cfg(feature = "vulkan")]
    {
        if let Ok(_) = vulkan::VulkanBackend::new() {
            return Some(detect_vulkan_vendor());
        }
    }

    None
}

#[cfg(feature = "vulkan")]
fn detect_vulkan_vendor() -> GPUVendor {
    // TODO: Actually detect vendor from Vulkan
    GPUVendor::Unknown
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_detection() {
        let vendor = detect_backend();
        // May be None if no GPU available
        println!("Detected backend: {:?}", vendor);
    }

    #[cfg(feature = "vulkan")]
    #[test]
    fn test_vulkan_available() {
        let result = vulkan::VulkanBackend::new();
        if result.is_ok() {
            println!("Vulkan backend available!");
        }
    }
}
