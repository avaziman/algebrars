


use crate::{
    math_tree::{MathTree, TreeNodeRef},
    stepper::Steps,
    MathToken,
};

// since contrary to addition, substraction is not an orderless operation,
// for simplification purposes, it is easier to represent substration as addition of a negative term
// this allows for grouping of addition and substraction

impl MathTree {
    pub fn simplify(&mut self, steps: &mut Steps) {
        Self::simplify_node(&mut self.root, steps);
    }

    fn simplify_node(node: &mut TreeNodeRef, steps: &mut Steps) {
        // let node = &mut self.root;
        if !node.val().is_operator() {
            return;
        }

        let mut borrow = node.borrow_mut();

        let operators = borrow.operands.remove_operators();
        // let mut multipliers = Vec::new();
        for mut op in operators {
            Self::simplify_node(&mut op, steps);
            // if let MathToken::Op(OperationToken::Multiply) = op.val() {
            //     multipliers.push(op.clone());
            // }
            borrow.add_operand(op);
        }

        // println!("simplifying {:#?}", borrow);
        let operands_len = borrow.operands.len();
        std::mem::drop(borrow);
        // Self::find_common_multiplier(multipliers);
        Self::perform_op(node, steps);

        if node.borrow().operands.len() > operands_len {
            Self::perform_op(node, steps);
        }

        // self.
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math_tree::{MathTree, TreeNodeRef},
        stepper::Steps,
        MathToken, OperationToken,
    };
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    fn simplify_test(expr: &str, res: TreeNodeRef) {
        let mut simplified = MathTree::parse(expr);
        let mut steps = Steps::new();
        simplified.simplify(&mut steps);

        assert_eq!(simplified.root, res);
    }

    #[test]
    fn simplify_constants() {
        simplify_test("1 + 2", TreeNodeRef::constant(dec!(3)));

        simplify_test("1 + 2 + 3", TreeNodeRef::constant(dec!(6)));

        simplify_test("1 + 2*2 + 3", TreeNodeRef::constant(dec!(8)));

        simplify_test("2 + 2^2", TreeNodeRef::constant(dec!(6)));

        simplify_test("2 + 2^3", TreeNodeRef::constant(dec!(10)));
    }

    #[test]
    fn simplify_x() {
        simplify_test(
            "1*x",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test("0*x", TreeNodeRef::constant(dec!(0)));

        simplify_test(
            "0 + x",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test(
            "x + x",
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Multiply),
                vec![
                    TreeNodeRef::constant(dec!(2)),
                    TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
                ],
            ),
        );

        // simplify_test(
        //     "2*x + x",
        //     TreeNodeRef::new_vals(
        //         MathToken::Op(OperationToken::Multiply),
        //         vec![
        //             TreeNodeRef::constant(dec!(3)),
        //             TreeNodeRef::new_val(MathToken::Variable(String::from("x"))),
        //         ],
        //     ),
        // );
    }

    #[test]
    fn zero_and_double_add_subs() {
        simplify_test(
            "+x",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test(
            "-x",
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Multiply),
                vec![

                    TreeNodeRef::constant(dec!(-1)),
                    TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
                ],
            ),
        );

        simplify_test(
            "+(+x)",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test(
            "-(-x)",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test("-(-2)", TreeNodeRef::constant(dec!(2)));

        // lex: 5 sub ( sub 2 )
        // pf: 52-- sub(sub(5, 2)) = sub(3) = -3 WRONG!
        // pf: 5-2- sub(5, sub(2)) = sub(5, -2) = 7 right => -2 needs to be parsed as a decimal not substract
        simplify_test("5-(-2)", TreeNodeRef::constant(dec!(7)));
    }
}
