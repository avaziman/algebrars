use std::borrow::Borrow;

use crate::{math_tree::{TreeNodeRef, MathTree}, MathToken, OperationToken};

impl MathTree {
    // TODO: foil
    // expanding is:
    // calculating literal operations
    // expanding brackets:
    //  > multiplying each element inside the brackets by the multiplier
    pub fn expand(node: TreeNodeRef) {
        if let MathToken::Op(OperationToken::Multiply) = node.val()  {
            
        }
    }

    pub fn expand_bracket(multiplier: TreeNodeRef) {

    }
}