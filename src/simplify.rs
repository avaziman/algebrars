use crate::{ast::{TreeNodeRef, AST}, MathToken, OperationToken};

// since contrary to addition, substraction is not an orderless operation,
// for simplification purposes, it is easier to represent substration as addition of a negative term
// this allows for grouping of addition and substraction

impl AST {
    pub fn simplify(&self) {

    }

    // fn simplify_node(node: TreeNodeRef) {
    //     if let MathToken::Op(op) = node.val()  {
    //         let left = node.left();
    //         let right = node.right();

    //         match op {
    //             OperationToken::Subtract => todo!(),
    //             OperationToken::Add => todo!(),
    //             OperationToken::Multiply => todo!(),
    //             OperationToken::Divide => todo!(),
    //             OperationToken::FractionDivide => todo!(),
    //             OperationToken::Pow => todo!(),
    //             OperationToken::Root => todo!(),
    //             OperationToken::LParent => todo!(),
    //             OperationToken::RParent => todo!(),
    //         }
    //     }
    // }
}