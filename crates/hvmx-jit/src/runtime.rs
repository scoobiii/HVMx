// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: runtime.rs
// Location: crates/hvmx-jit/src/runtime.rs
// Purpose: JIT runtime core with adaptive backend selection
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

use std::collections::HashMap;
use anyhow::Result;
use hvmx_core::GNet;
use crate::ir::HVMIR;

#[cfg(feature = "vulkan")]
use crate::backend::vulkan::VulkanBackend;

/// GPU vendor detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GPUVendor {
    NvidiaDesktop,
    QualcommAdreno,
    ARMMali,
    AppleSilicon,
    AMDDesktop,
    IntelXe,
    Unknown,
}

/// GPU information
#[derive(Debug, Clone)]
pub struct GPUInfo {
    pub vendor: GPUVendor,
    pub compute_units: u32,
    pub shared_memory: usize,
    pub is_unified_memory: bool,
}

/// Compiled kernel handle
#[derive(Clone)]
pub struct CompiledKernel {
    pub id: u64,
    pub workgroup_size: (u32, u32),
}

/// GPU backend trait
pub trait GPUBackend: Send + Sync {
    fn compile(&self, ir: &HVMIR) -> Result<CompiledKernel>;
    fn execute(&self, kernel: &CompiledKernel, net: &mut GNet) -> Result<()>;
    fn get_info(&self) -> GPUInfo;
}

/// Main JIT runtime
pub struct HVMRuntime {
    backend: Box<dyn GPUBackend>,
    kernel_cache: HashMap<u64, CompiledKernel>,
}

impl HVMRuntime {
    /// Create new runtime with automatic backend detection
    pub fn new() -> Result<Self> {
        let backend = Self::detect_and_create_backend()?;
        Ok(Self {
            backend,
            kernel_cache: HashMap::new(),
        })
    }

    /// Detect best GPU backend available
    fn detect_and_create_backend() -> Result<Box<dyn GPUBackend>> {
        #[cfg(feature = "vulkan")]
        {
            match VulkanBackend::new() {
                Ok(backend) => return Ok(Box::new(backend)),
                Err(e) => log::warn!("Vulkan not available: {}", e),
            }
        }

        anyhow::bail!("No GPU backend available")
    }

    /// Evaluate a network on GPU
    pub fn eval(&mut self, net: &mut GNet) -> Result<()> {
        // 1. Convert net to IR
        let ir = self.net_to_ir(net)?;

        // 2. Compile or retrieve from cache
        let cache_key = self.hash_ir(&ir);
        let kernel = if let Some(cached) = self.kernel_cache.get(&cache_key) {
            cached.clone()
        } else {
            let compiled = self.backend.compile(&ir)?;
            self.kernel_cache.insert(cache_key, compiled.clone());
            compiled
        };

        // 3. Execute on GPU
        self.backend.execute(&kernel, net)?;

        Ok(())
    }

    /// Convert GNet to IR
    fn net_to_ir(&self, net: &GNet) -> Result<HVMIR> {
        let mut ir = HVMIR::new();
        
        // Convert each redex to IR instructions
        for (a, b) in &net.redexes {
            ir.add_node(crate::ir::IRNode::Interact { 
                a: a.val(), 
                b: b.val() 
            });
        }

        Ok(ir)
    }

    /// Hash IR for cache lookup
    fn hash_ir(&self, ir: &HVMIR) -> u64 {
        // Simple hash based on IR length for now
        ir.len() as u64
    }

    /// Get backend info
    pub fn backend_info(&self) -> GPUInfo {
        self.backend.get_info()
    }
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        // Note: May fail if no GPU available
        let result = HVMRuntime::new();
        if result.is_ok() {
            let runtime = result.unwrap();
            let info = runtime.backend_info();
            println!("GPU detected: {:?}", info.vendor);
        }
    }

    #[test]
    fn test_gpu_vendor_enum() {
        let vendor = GPUVendor::QualcommAdreno;
        assert_eq!(vendor, GPUVendor::QualcommAdreno);
    }

    #[test]
    fn test_compiled_kernel() {
        let kernel = CompiledKernel {
            id: 42,
            workgroup_size: (16, 16),
        };
        assert_eq!(kernel.id, 42);
    }

    #[test]
    fn test_net_to_ir_conversion() {
        let runtime = match HVMRuntime::new() {
            Ok(r) => r,
            Err(_) => return, // Skip if no GPU
        };
        
        let net = GNet::new();
        let result = runtime.net_to_ir(&net);
        assert!(result.is_ok());
    }

    #[test]
    fn test_kernel_cache() {
        let mut runtime = match HVMRuntime::new() {
            Ok(r) => r,
            Err(_) => return,
        };
        
        assert_eq!(runtime.kernel_cache.len(), 0);
    }
}
