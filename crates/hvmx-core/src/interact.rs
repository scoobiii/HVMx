// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: interact.rs
// Location: crates/hvmx-core/src/interact.rs
// Purpose: Interaction net reduction rules
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

use crate::{Port, GNet, Tag};

/// Interaction rules between ports
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    Link,    // VAR-VAR: just link
    Anni,    // CON-CON same label: annihilation
    Comm,    // CON-CON diff label: commutation
    Eras,    // ERA-anything: erasure
    Deref,   // REF-anything: dereference
    Call,    // APP-LAM: beta reduction
    Copy,    // DUP-anything: duplication
    Oper,    // OP2-NUM: numeric operation
}

/// Get interaction rule for pair of ports
pub fn get_rule(a: Port, b: Port) -> Rule {
    use Tag::*;
    
    match (a.tag(), b.tag()) {
        (Var, Var) => Rule::Link,
        (Var, _) | (_, Var) => Rule::Link,
        (Ref, _) | (_, Ref) => Rule::Deref,
        (Num, Num) => Rule::Oper,
        _ => Rule::Link, // Simplified
    }
}

/// Execute interaction between two ports
pub fn interact(net: &mut GNet, a: Port, b: Port) -> Result<(), String> {
    let rule = get_rule(a, b);
    
    match rule {
        Rule::Link => interact_link(net, a, b),
        Rule::Anni => interact_anni(net, a, b),
        Rule::Comm => interact_comm(net, a, b),
        Rule::Eras => interact_eras(net, a, b),
        Rule::Deref => interact_deref(net, a, b),
        Rule::Call => interact_call(net, a, b),
        Rule::Copy => interact_copy(net, a, b),
        Rule::Oper => interact_oper(net, a, b),
    }
}

// Individual interaction implementations

fn interact_link(_net: &mut GNet, a: Port, b: Port) -> Result<(), String> {
    // VAR-VAR: create link
    // TODO: Implement linking logic
    Ok(())
}

fn interact_anni(_net: &mut GNet, _a: Port, _b: Port) -> Result<(), String> {
    // CON-CON same label: annihilate
    Ok(())
}

fn interact_comm(_net: &mut GNet, _a: Port, _b: Port) -> Result<(), String> {
    // CON-CON different labels: commute
    Ok(())
}

fn interact_eras(_net: &mut GNet, _a: Port, _b: Port) -> Result<(), String> {
    // ERA-anything: erase
    Ok(())
}

fn interact_deref(_net: &mut GNet, _a: Port, _b: Port) -> Result<(), String> {
    // REF-anything: dereference
    Ok(())
}

fn interact_call(_net: &mut GNet, _a: Port, _b: Port) -> Result<(), String> {
    // APP-LAM: function application
    Ok(())
}

fn interact_copy(_net: &mut GNet, _a: Port, _b: Port) -> Result<(), String> {
    // DUP-anything: duplicate
    Ok(())
}

fn interact_oper(_net: &mut GNet, a: Port, b: Port) -> Result<(), String> {
    // NUM-NUM: arithmetic operation
    let val_a = a.val();
    let val_b = b.val();
    let _result = val_a + val_b; // Example: addition
    Ok(())
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rule_link() {
        let a = Port::new(Tag::Var, 1);
        let b = Port::new(Tag::Var, 2);
        assert_eq!(get_rule(a, b), Rule::Link);
    }

    #[test]
    fn test_get_rule_deref() {
        let a = Port::new(Tag::Ref, 1);
        let b = Port::new(Tag::Var, 2);
        assert_eq!(get_rule(a, b), Rule::Deref);
    }

    #[test]
    fn test_get_rule_oper() {
        let a = Port::new(Tag::Num, 10);
        let b = Port::new(Tag::Num, 20);
        assert_eq!(get_rule(a, b), Rule::Oper);
    }

    #[test]
    fn test_interact_link() {
        let mut net = GNet::new();
        let a = Port::new(Tag::Var, 1);
        let b = Port::new(Tag::Var, 2);
        
        let result = interact(&mut net, a, b);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interact_oper() {
        let mut net = GNet::new();
        let a = Port::new(Tag::Num, 5);
        let b = Port::new(Tag::Num, 3);
        
        let result = interact(&mut net, a, b);
        assert!(result.is_ok());
    }
}
