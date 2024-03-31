
use crate::{ast::{TreeNodeRef, AST}, MathToken, OperationToken};

// since contrary to addition, substraction is not an orderless operation,
// for simplification purposes, it is easier to represent substration as addition of a negative term
// this allows for grouping of addition and substraction

impl AST {
    pub fn simplify(&self) {

    }

    fn simplify_node(node: TreeNodeRef) {
        if let MathToken::Op(op) = node.val()  {
            let operands = &node.0.borrow().childs;

            match op {
                OperationToken::Subtract => todo!(),
                OperationToken::Add => {
                            
                },
                OperationToken::Multiply => todo!(),
                OperationToken::Divide => todo!(),
                OperationToken::FractionDivide => todo!(),
                OperationToken::Pow => todo!(),
                OperationToken::Root => todo!(),
                OperationToken::LParent => todo!(),
                OperationToken::RParent => todo!(),
            }
        }
    }
}