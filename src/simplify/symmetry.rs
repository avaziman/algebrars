use itertools::Itertools;

use crate::{math_tree::TreeNodeRef, operands::OperandPos, MathTokenType, OperationToken};

// division will always have only two operands as it is not orderless
pub fn symmetrical_scan(node: TreeNodeRef) {
    let (lval, rval) = node.borrow().operands.iter().collect_tuple().unwrap();
    symmetrical_scan_node(lval, node.clone(), rval, node);
}

fn symmetrical_scan_node(
    left_pos: OperandPos,
    lparent: TreeNodeRef,
    right_pos: OperandPos,
    rparent: TreeNodeRef,
) {
    let left = lparent.borrow().operands[left_pos].clone();
    let right = rparent.borrow().operands[right_pos].clone();
    let lval = left.val();
    let rval = right.val();

    match (lval.kind, rval.kind) {
        (MathTokenType::Constant, MathTokenType::Constant) => {
            if lval == rval {
                lparent.borrow_mut().operands.remove(left_pos);
                rparent.borrow_mut().operands.remove(right_pos);
            }
        }
        // (MathTokenType::Constant, MathTokenType::Variable) => todo!(),
        // (MathTokenType::Constant, MathTokenType::Operator) => todo!(),
        // (MathTokenType::Variable, MathTokenType::Constant) => todo!(),
        // (MathTokenType::Variable, MathTokenType::Variable) => todo!(),
        // (MathTokenType::Variable, MathTokenType::Operator) => todo!(),
        (MathTokenType::Operator, MathTokenType::Constant) => {
            if lval.operation.unwrap() == OperationToken::Multiply {
                let (lval1, rval1) = left.borrow().operands.iter_mul().collect_tuple().unwrap();

                symmetrical_scan_node(lval1, left.clone(), right_pos, rparent.clone());
            }
        },
        // (MathTokenType::Operator, MathTokenType::Variable) => todo!(),
        (MathTokenType::Operator, MathTokenType::Operator) => {
            match (lval.operation.unwrap(), rval.operation.unwrap()) {
                (OperationToken::Multiply, OperationToken::Multiply) | (OperationToken::Divide, OperationToken::Divide) => {
                    let (lval1, rval1) = left.borrow().operands.iter().collect_tuple().unwrap();
                    let (lval2, rval2) = right.borrow().operands.iter().collect_tuple().unwrap();

                    symmetrical_scan_node(lval1, left.clone(), lval2, right.clone());
                    symmetrical_scan_node(rval1, left, rval2, right);
                }
                _ => {}
            }
        }
        _ => {}
    }
    if left_pos == right_pos {}
}
