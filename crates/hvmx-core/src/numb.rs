// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: numb.rs
// Location: crates/hvmx-core/src/numb.rs
// Purpose: Numeric operations (60-bit floats)
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

use std::ops::{Add, Sub, Mul, Div};

/// Numb: 60-bit numeric type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Numb(pub u64);

impl Numb {
    pub fn new(val: u64) -> Self {
        Numb(val & 0x0FFFFFFFFFFFFFFF) // 60-bit mask
    }

    pub fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    pub fn from_f64(f: f64) -> Self {
        Numb::new(f as u64)
    }
}

// Arithmetic operations

impl Add for Numb {
    type Output = Numb;
    
    fn add(self, other: Numb) -> Numb {
        Numb::new(self.0.wrapping_add(other.0))
    }
}

impl Sub for Numb {
    type Output = Numb;
    
    fn sub(self, other: Numb) -> Numb {
        Numb::new(self.0.wrapping_sub(other.0))
    }
}

impl Mul for Numb {
    type Output = Numb;
    
    fn mul(self, other: Numb) -> Numb {
        Numb::new(self.0.wrapping_mul(other.0))
    }
}

impl Div for Numb {
    type Output = Numb;
    
    fn div(self, other: Numb) -> Numb {
        if other.0 == 0 {
            Numb(0)
        } else {
            Numb::new(self.0 / other.0)
        }
    }
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numb_creation() {
        let n = Numb::new(42);
        assert_eq!(n.0, 42);
    }

    #[test]
    fn test_numb_add() {
        let a = Numb::new(10);
        let b = Numb::new(20);
        let c = a + b;
        assert_eq!(c.0, 30);
    }

    #[test]
    fn test_numb_sub() {
        let a = Numb::new(50);
        let b = Numb::new(20);
        let c = a - b;
        assert_eq!(c.0, 30);
    }

    #[test]
    fn test_numb_mul() {
        let a = Numb::new(5);
        let b = Numb::new(3);
        let c = a * b;
        assert_eq!(c.0, 15);
    }

    #[test]
    fn test_numb_div() {
        let a = Numb::new(20);
        let b = Numb::new(4);
        let c = a / b;
        assert_eq!(c.0, 5);
    }

    #[test]
    fn test_numb_div_by_zero() {
        let a = Numb::new(10);
        let b = Numb::new(0);
        let c = a / b;
        assert_eq!(c.0, 0); // Safe zero division
    }
}
