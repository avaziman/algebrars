use rust_decimal::Decimal;

use crate::{math_tree::TreeNodeRef, MathToken, OperationToken};

// מספר נגדי
// pub fn additive_inverse(node: TreeNodeRef) -> (OperationToken, TreeNodeRef) {
//     match node.val() {
//         MathToken::Constant(c) => if c > Decimal::ZERO {
//             (OperationToken::Subtract, node)
//         }else {
//             (OperationToken::Add, TreeNodeRef::constant(c.abs()))
//         },
//         MathToken::Variable(v) => 
//             (OperationToken::Subtract, node),
//         MathToken::Op(_) => todo!(),
//     }
// }