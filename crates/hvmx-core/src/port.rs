// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: port.rs
// Location: src/port.rs
// Purpose: Port type: 32-bit tagged pointers
// Author: scoobiii
// Date: 2025-12-28
// License: MIT OR Apache-2.0
// ==============================================================================


/// Port: 32-bit value (3-bit tag + 29-bit val)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Var = 0,
    Ref = 1,
    Num = 2,
    // ... outros
}

pub type Val = u32;

impl Port {
    pub fn new(tag: Tag, val: Val) -> Self {
        Port(((tag as u32) << 29) | (val & 0x1FFFFFFF))
    }
    
    pub fn tag(&self) -> Tag {
        match self.0 >> 29 {
            0 => Tag::Var,
            1 => Tag::Ref,
            2 => Tag::Num,
            _ => unreachable!(),
        }
    }
    
    pub fn val(&self) -> Val {
        self.0 & 0x1FFFFFFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_creation() {
        let port = Port::new(Tag::Var, 42);
        assert_eq!(port.tag(), Tag::Var);
        assert_eq!(port.val(), 42);
    }
}
