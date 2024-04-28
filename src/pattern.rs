use std::collections::HashMap;

use crate::{
    math_tree::{MathTree, TreeNodeRef},
};

impl MathTree {
    // checks if a given expression matches a given pattern with variables,
    // returns the nodes in the given tree that match the gives variables
    pub fn like(node: &TreeNodeRef, pattern: &str) -> Option<HashMap<String, TreeNodeRef>> {
        let mut variables = HashMap::new();
        let pattern = MathTree::parse(pattern);

        if !Self::node_like(node, &pattern.root, &mut variables) {
            return None;
        }

        Some(variables)
    }

    fn node_like(
        _check_node: &TreeNodeRef,
        _pattern_node: &TreeNodeRef,
        _variables: &mut HashMap<String, TreeNodeRef>,
    ) -> bool {
        todo!()
        // match pattern_node.val().kind {
        //     // constants must match exactly
        //     MathTokenType::Constant => check_node == pattern_node,
        //     MathToken::Op(op) => {
        //         if let MathToken::Op(op2) = check_node.val() {
        //             let b1 = check_node.borrow();
        //             let b2= pattern_node.borrow();
        //             let iter1 = b1.operands.iter();
        //             let iter2 = b2.operands.iter();
        //             // operation type must match
        //             op == op2 &&
        //             // operands length must match
        //             b1.operands.len() == b2.operands.len() &&
        //             // all the childs must match the rest of the pattern
        //             iter1.zip(iter2).all(|((_, a), (_, b))| 
        //                 Self::node_like(a, b, variables))
        //         } else {
        //             false
        //         }
        //     }
        //     MathToken::Variable(v) => {
        //         // pattern expects a variable

        //         match variables.get(&v) {
        //             // if we saw that variable before, we expect it to be identical
        //             Some(x) =>  x == check_node,
        //             // if we haven't seen this variable before then it should be equal to this from now on
        //             None => {variables.insert(v, check_node.clone()); true},
        //         }
        //     },
        // }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rust_decimal_macros::dec;

    use crate::{math_tree::{MathTree, TreeNodeRef}, MathToken};

    #[test]
    fn like_test() {
        assert_eq!(MathTree::like(&MathTree::parse("2^3*2^4").root, "x^m*x^n"), Some(HashMap::from([
            ("x".to_string(), TreeNodeRef::constant(dec!(2))),
            ("m".to_string(), TreeNodeRef::constant(dec!(3))),
            ("n".to_string(), TreeNodeRef::constant(dec!(4))),
        ])));

        assert_eq!(MathTree::like(&MathTree::parse("(x + 2)^2").root, "(a + b)^2"), Some(HashMap::from([
            ("a".to_string(), TreeNodeRef::new_val(MathToken::variable(String::from("x")))),
            ("b".to_string(), TreeNodeRef::constant(dec!(2))),
        ])));

    }
}