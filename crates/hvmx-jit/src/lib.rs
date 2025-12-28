// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: lib.rs
// Location: crates/hvmx-jit/src/lib.rs
// Purpose: JIT compiler root
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

pub mod ir;
pub mod runtime;
pub mod backend;

pub use ir::HVMIR;
