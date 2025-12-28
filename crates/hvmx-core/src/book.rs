// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: book.rs
// Location: crates/hvmx-core/src/book.rs
// Purpose: Definition book (function storage)
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

use std::collections::HashMap;
use crate::GNet;

/// Book: stores function definitions
#[derive(Debug, Clone)]
pub struct Book {
    defs: HashMap<String, Def>,
}

/// Definition: a named function/term
#[derive(Debug, Clone)]
pub struct Def {
    pub name: String,
    pub arity: usize,
    pub net: GNet,
}

impl Book {
    pub fn new() -> Self {
        Book {
            defs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, def: Def) {
        self.defs.insert(name, def);
    }

    pub fn get(&self, name: &str) -> Option<&Def> {
        self.defs.get(name)
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}

impl Default for Book {
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
    fn test_book_creation() {
        let book = Book::new();
        assert_eq!(book.len(), 0);
        assert!(book.is_empty());
    }

    #[test]
    fn test_book_insert() {
        let mut book = Book::new();
        let def = Def {
            name: "test".to_string(),
            arity: 1,
            net: GNet::new(),
        };
        
        book.insert("test".to_string(), def);
        assert_eq!(book.len(), 1);
    }

    #[test]
    fn test_book_get() {
        let mut book = Book::new();
        let def = Def {
            name: "func".to_string(),
            arity: 2,
            net: GNet::new(),
        };
        
        book.insert("func".to_string(), def.clone());
        let retrieved = book.get("func");
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "func");
    }

    #[test]
    fn test_book_get_missing() {
        let book = Book::new();
        let result = book.get("missing");
        assert!(result.is_none());
    }
}
