// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: ir.rs
// Location: crates/hvmx-jit/src/ir.rs
// Purpose: Intermediate Representation for JIT
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

/// IR: Intermediate representation
#[derive(Debug, Clone)]
pub struct HVMIR {
    pub nodes: Vec<IRNode>,
}

#[derive(Debug, Clone)]
pub enum IRNode {
    Link { src: u32, dst: u32 },
    Alloc { size: usize },
    Free { ptr: u32 },
    Interact { a: u32, b: u32 },
}

impl HVMIR {
    pub fn new() -> Self {
        HVMIR { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: IRNode) {
        self.nodes.push(node);
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Default for HVMIR {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_creation() {
        let ir = HVMIR::new();
        assert_eq!(ir.len(), 0);
        assert!(ir.is_empty());
    }

    #[test]
    fn test_ir_add_node() {
        let mut ir = HVMIR::new();
        ir.add_node(IRNode::Link { src: 1, dst: 2 });
        assert_eq!(ir.len(), 1);
    }

    #[test]
    fn test_ir_multiple_nodes() {
        let mut ir = HVMIR::new();
        ir.add_node(IRNode::Alloc { size: 1024 });
        ir.add_node(IRNode::Link { src: 1, dst: 2 });
        ir.add_node(IRNode::Free { ptr: 1 });
        
        assert_eq!(ir.len(), 3);
    }
}
