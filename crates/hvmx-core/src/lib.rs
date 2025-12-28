// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: lib.rs
// Location: src/lib.rs
// Purpose: Core runtime types and operations
// Author: scoobiii
// Date: 2025-12-28
// License: MIT OR Apache-2.0
// ==============================================================================


pub mod port;
pub mod net;
pub mod interact;
pub mod numb;
pub mod book;

// Re-exports
pub use port::{Port, Tag, Val};
pub use net::GNet;
pub use interact::interact;
pub use numb::Numb;
pub use book::Book;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_imports() {
        // Verifica que imports funcionam
        let _port: Port = Port::new(Tag::Var, 0);
    }
}
