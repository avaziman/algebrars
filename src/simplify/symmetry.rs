use itertools::Itertools;

use crate::{
    factorization,
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

    let mut multipliers = Vec::new();
    // // println!("COMPARING L {:#?} {:#?}", left, right);
    if left == right {
        res.push((lparent.clone(), left_pos));
        res.push((rparent.clone(), right_pos));
        println!(
            "FOUND SYMMETRY OF {:?} in {:#?} {:#?}",
            left, lparent, rparent
        );
        // if the nodes are equal we dont have to dig deeper
        return;
    }

    // if left.val().is_operator()
    // // && !right.val().is_operator()
    // {
    //     // let operands = left.borrow().calculate_iter().map(|x| x.0).collect_vec();
    //     if left.val().operation.unwrap().is_mul_or_div() {
    //         multipliers.extend(left.borrow().calculate_iter().map(|x| x.1.clone()));
    //     }
    //     // for pos in operands {
    //     //     symmetrical_scan_node(pos, left.clone(), right_pos, rparent.clone(), res);
    //     // }
    // } else {
        multipliers.push(left.clone());
    // }

    // if right.val().is_operator()
    // // && !left.val().is_operator()
    // {
    //     // let operands = right.borrow().calculate_iter().map(|x| x.0).collect_vec();
    //     if right.val().operation.unwrap().is_mul_or_div() {
    //         multipliers.extend(right.borrow().calculate_iter().map(|x| x.1.clone()));
    //     }
    //     // for pos in operands {
    //     //     symmetrical_scan_node(left_pos, lparent.clone(), pos, right.clone(), res);
    //     // }
    // } else {
        multipliers.push(right.clone());
    // }

    println!("SYMMETRICALLY FACTORING OUT: {:#?}", multipliers);
    if let Some(factor_out) = MathTree::find_common_variable(multipliers) {
        println!("SYMMETRICALLY FACTORED OUT: {:#?}", factor_out);
        lparent
            .borrow_mut()
            .replace_operand(left_pos, left.divide(factor_out.clone()));
        rparent
            .borrow_mut()
            .replace_operand(right_pos, right.divide(factor_out));
    }
}

impl MathTree {
    pub fn cancel_symmetrical() {}
}
