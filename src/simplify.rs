use itertools::Itertools;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use crate::{
    math_tree::{MathTree, TreeNodeRef},
    operands::Operands,
    MathToken, OperationToken,
};

// since contrary to addition, substraction is not an orderless operation,
// for simplification purposes, it is easier to represent substration as addition of a negative term
// this allows for grouping of addition and substraction

impl MathTree {
    pub fn simplify(&mut self) {
        Self::simplify_node(&mut self.root);
    }

    fn simplify_node(node: &mut TreeNodeRef) {
        // let node = &mut self.root;
        let MathToken::Op(op) = node.val() else {
            return;
        };

        let mut borrow = node.0.borrow_mut();
        let operands = &mut borrow.operands;

        let operators = operands.remove_operators();
        // let mut multipliers = Vec::new();
        for mut op in operators {
            Self::simplify_node( &mut op);
            // if let MathToken::Op(OperationToken::Multiply) = op.val() {
            //     multipliers.push(op.clone());
            // }
            operands.add(op);
        }

        std::mem::drop(borrow);
        // Self::find_common_multiplier(multipliers);
        Self::perform_op(node);

        // self.
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math_tree::{MathTree, TreeNodeRef},
        MathToken, OperationToken,
    };
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    fn simplify_test(expr: &str, res: TreeNodeRef) {
        let mut simplified = MathTree::parse(expr);
        simplified.simplify();

        assert_eq!(simplified.root, res);
    }

    #[test]
    fn simplify_constants() {
        simplify_test("1 + 2", TreeNodeRef::new_val(MathToken::Constant(dec!(3))));

        simplify_test(
            "1 + 2 + 3",
            TreeNodeRef::new_val(MathToken::Constant(dec!(6))),
        );

        simplify_test(
            "1 + 2*2 + 3",
            TreeNodeRef::new_val(MathToken::Constant(dec!(8))),
        );

        simplify_test(
            "2 + 2^2",
            TreeNodeRef::new_val(MathToken::Constant(dec!(6))),
        );

        simplify_test(
            "2 + 2^3",
            TreeNodeRef::new_val(MathToken::Constant(dec!(10))),
        );
    }

    #[test]
    fn simplify_x() {
        simplify_test(
            "1*x",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x"))),
        );

        simplify_test("0*x", TreeNodeRef::new_val(MathToken::Constant(dec!(0))));

        simplify_test(
            "0 + x",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x"))),
        );

        simplify_test(
            "x + x",
            TreeNodeRef::new_vals(
                MathToken::Op(OperationToken::Multiply),
                vec![
                    TreeNodeRef::constant(dec!(2)),
                    TreeNodeRef::new_val(MathToken::Variable(String::from("x"))),
                ],
            ),
        );


        simplify_test(
            "2*x + x",
            TreeNodeRef::new_vals(
                MathToken::Op(OperationToken::Multiply),
                vec![
                    TreeNodeRef::constant(dec!(3)),
                    TreeNodeRef::new_val(MathToken::Variable(String::from("x"))),
                ],
            ),
        );
    }
}

