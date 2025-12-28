// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: net.rs
// Location: src/net.rs
// Purpose: GNet: Interaction net structure
// Author: scoobiii
// Date: 2025-12-28
// License: MIT OR Apache-2.0
// ==============================================================================


use crate::Port;

pub struct GNet {
    pub nodes: Vec<u64>,
    pub redexes: Vec<(Port, Port)>,
}

impl GNet {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            redexes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnet_creation() {
        let net = GNet::new();
        assert_eq!(net.nodes.len(), 0);
    }
}
