use itertools::Itertools;

use crate::{
    math_tree::{MathTree, TreeNodeRef},
    operands::OperandPos,
    MathTokenType, OperationToken,
};

pub fn symmetrical_scan(node: TreeNodeRef) {
    // division will always have only two operands as it is not orderless
    let mut res = Vec::new();
    let ((lval, _), (rval, _)) = node.borrow().calculate_iter().collect_tuple().unwrap();
    symmetrical_scan_node(lval, node.clone(), rval, node, &mut res);

    for (parent, pos) in res {
        // lparent.borrow_mut().operands_mut().remove(left_pos);
        // rparent.borrow_mut().operands_mut().remove(right_pos);
        parent.borrow_mut().remove_operand(pos);
    }
}

fn symmetrical_scan_node(
    left_pos: OperandPos,
    lparent: TreeNodeRef,
    right_pos: OperandPos,
    rparent: TreeNodeRef,
    res: &mut Vec<(TreeNodeRef, OperandPos)>,
) {
    // let

    let left = lparent.borrow()[left_pos].clone();
    let right = rparent.borrow()[right_pos].clone();

    if left == right {
        res.push((lparent.clone(), left_pos));
        res.push((rparent.clone(), right_pos));
        println!("FOUND SYMMETRY OF {:?}", left);
        // if the nodes are equal we dont have to dig deeper
        return;
    }

    if left.val().is_operator() && left.val().operation.unwrap().is_mul_or_div() {
        let operands = left.borrow().calculate_iter().map(|x| x.0).collect_vec();

        for pos in operands {
            symmetrical_scan_node(pos, left.clone(), right_pos, rparent.clone(), res);
        }
    }

    if right.val().is_operator() && right.val().operation.unwrap().is_mul_or_div()  {
        let operands = right.borrow().calculate_iter().map(|x| x.0).collect_vec();

        for pos in operands {
            symmetrical_scan_node(left_pos, lparent.clone(), pos, right.clone(), res);
        }
    }
}

impl MathTree {
    pub fn cancel_symmetrical() {}
}
