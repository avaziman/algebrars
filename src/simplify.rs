use std::cmp::Ordering;

use crate::{
    ast::{TreeNodeRef, AST},
    MathToken, OperationToken,
};

// since contrary to addition, substraction is not an orderless operation,
// for simplification purposes, it is easier to represent substration as addition of a negative term
// this allows for grouping of addition and substraction

impl AST {
    pub fn simplify(&mut self) {
        Self::simplify_node(&mut self.root);
    }

    fn simplify_node(node: &mut TreeNodeRef) {
        let MathToken::Op(op) = node.val() else {
            return;
        };
        // operands.sort_by(|a, b| match a.val() {
        //     MathToken::Constant(_) => Ordering::Greater,
        //     MathToken::Variable(_) => match b.val() {
        //         MathToken::Constant(_) => Ordering::Less,
        //         MathToken::Variable(_) => Ordering::Equal,
        //         MathToken::Op(_) => Ordering::Greater,
        //     },
        //     MathToken::Op(_) => Ordering::Less,
        // });
        // let constant_operands = operands.iter().filter_map(|f| {
        //     if let MathToken::Constant(c) = f.val() {
        //         Some(c)
        //     } else {
        //         None
        //     }
        // });
        let mut borrow = node.0.borrow_mut();
        let operands = &mut borrow.childs;
        for op in operands.iter_mut() {
            Self::simplify_node(op);
        }

        let mut constants = Vec::new();
        operands.retain(|o| {
            if let MathToken::Constant(c) = o.val() {
                constants.push(c);
                false
            } else {
                true
            }
        });

        let result = match op {
            OperationToken::Subtract => todo!(),
            OperationToken::Add => constants.iter().sum(),
            OperationToken::Multiply => constants
                .iter()
                .zip(constants.iter().skip(1))
                .map(|(a, b)| a * b)
                .sum(),
            OperationToken::Divide => todo!(),
            OperationToken::FractionDivide => todo!(),
            OperationToken::Pow => todo!(),
            OperationToken::Root => todo!(),
            OperationToken::LParent => todo!(),
            OperationToken::RParent => todo!(),
        };
        let result = TreeNodeRef::new_val(MathToken::Constant(result));
        if operands.is_empty() {
            // we have completed the operation, no operands left
            std::mem::drop(borrow);
            *node = result;
        } else {
            operands.push(result);
        }
    }
}

pub enum Step {
    DoOpOn(Vec<TreeNodeRef>)
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::{
        ast::{TreeNodeRef, AST},
        lexer::Lexer,
        MathToken, OperationToken,
    };

    fn simplify_test(expr: &str, res: TreeNodeRef) {
        let mut simplified = AST::parse(Lexer::new(expr));
        simplified.simplify();

        assert_eq!(simplified.root, res);
    }

    #[test]
    fn simplify() {
        simplify_test("1 + 2", TreeNodeRef::new_val(MathToken::Constant(dec!(3))));

        simplify_test(
            "1 + 2 + 3",
            TreeNodeRef::new_val(MathToken::Constant(dec!(6))),
        );

        simplify_test(
            "1 + 2*2 + 3",
            TreeNodeRef::new_val(MathToken::Constant(dec!(8))),
        );
    }
}
