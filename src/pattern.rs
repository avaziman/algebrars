use std::{collections::HashMap, rc::Rc};

use crate::{
    math_tree::{MathTree, TreeNodeRef}, MathToken
};

impl MathTree {
    // checks if a given expression matches a given pattern with variables,
    // returns the nodes in the given tree that match the gives variables
    pub fn like(node: &TreeNodeRef, pattern: &str) -> Option<HashMap<Rc<String>, TreeNodeRef>> {
        let mut variables = HashMap::new();
        let pattern = MathTree::parse(pattern).unwrap();

        if !Self::node_like(node, &pattern.root, &mut variables) {
            return None;
        }

        Some(variables)
    }

    fn node_like(
        check_node: &TreeNodeRef,
        pattern_node: &TreeNodeRef,
        variables: &mut HashMap<Rc<String>, TreeNodeRef>,
    ) -> bool {
        let val = pattern_node.val();
        match val{
            // constants must match exactly
            MathToken::Constant(check_node) => check_node == pattern_node,
            MathToken::Operation(op) => {
                if let MathToken::Operation(op2) = check_node.val() {
                    let b1 = check_node.borrow();
                    let b2= pattern_node.borrow();
                    let iter1 = b1.operands().iter_order();
                    let iter2 = b2.operands().iter_order();
                    // operation type must match
                    op == op2 &&
                    // operands length must match
                    b1.operands().len() == b2.operands().len() &&
                    // all the childs must match the rest of the pattern
                    iter1.zip(iter2).all(|((_,a), (_, b))| 
                        Self::node_like(a, b, variables))
                } else {
                    false
                }
            }
            MathToken::Variable(v) => {
                // pattern expects a variable
                match variables.get(&v) {
                    // if we saw that variable before, we expect it to be identical
                    Some(x) =>  x == check_node,
                    // if we haven't seen this variable before then it should be equal to this from now on
                    None => {variables.insert(v, check_node.clone()); true},
                }
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rust_decimal_macros::dec;
    use pretty_assertions::assert_eq;

    use crate::{math_tree::{MathTree, TreeNodeRef}, MathToken};

    #[test]
    fn like_test() {
        assert_eq!(MathTree::like(&MathTree::parse("2^3*2^4").unwrap().root, "x^m*x^n"), Some(HashMap::from([
            ("x".to_string().into(), TreeNodeRef::constant(dec!(2))),
            ("m".to_string().into(), TreeNodeRef::constant(dec!(3))),
            ("n".to_string().into(), TreeNodeRef::constant(dec!(4))),
        ])));

        
        assert_eq!(MathTree::like(&MathTree::parse("(x + 2)^2").unwrap().root, "(a + b)^2"), Some(HashMap::from([
            ("a".to_string().into(), TreeNodeRef::new_val(MathToken::Variable(String::from("x").into()))),
            ("b".to_string().into(), TreeNodeRef::constant(dec!(2))),
            ])));
            
    }
}