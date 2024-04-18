use itertools::Itertools;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use crate::{
    ast::{TreeNodeRef, AST},
    operands::Operands,
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

        let mut borrow = node.0.borrow_mut();
        let operands = &mut borrow.operands;

        let operators = operands.remove_operators();
        let mut multipliers = Vec::new();
        for mut op in operators {
            Self::simplify_node(&mut op);
            if let MathToken::Op(OperationToken::Multiply) = op.val() {
                multipliers.push(op.clone());
            }

            operands.add(op);
        }

        Self::find_common_multiplier(multipliers);

        // perform constant arithmatic
        let mut constants = operands.remove_constants();

        match op {
            OperationToken::Subtract | OperationToken::Add => {
                // +-0 changes nothing
                constants.retain(|c| *c != Decimal::ZERO);
            }
            OperationToken::Multiply => {
                // single zero zeros out everything in mul
                if constants.iter().find(|c| c.is_zero()).is_some() {
                    std::mem::drop(borrow);
                    *node = TreeNodeRef::new_val(MathToken::Constant(Decimal::ZERO));
                    return;
                }

                // *1 changes nothing
                constants.retain(|c| *c != Decimal::ONE);
            }
            // OperationToken::Divide => todo!(),
            _ => {}
        }
        
        // eliminate the operators if its useless
        if (operands.len() + constants.len()) == 1 && op.info().arity > 1 {
            let value = operands.iter().next().unwrap().clone();
            std::mem::drop(borrow);
            *node = value;
            return;
        }

        let op = match op {
            OperationToken::Subtract => |a, b| a - b,
            OperationToken::Add => |a, b| a + b,
            OperationToken::Multiply => |a, b| a * b,
            OperationToken::Divide => |a, b| a / b,
            OperationToken::Pow => |a: Decimal, b| a.powd(b),
            OperationToken::FractionDivide => todo!(),
            OperationToken::Root => todo!(),
            OperationToken::LParent | OperationToken::RParent => unreachable!(),
        };

        let mut operand_iter = constants.into_iter();
        if let Some(mut result) = operand_iter.next() {
            for operand in operand_iter {
                result = op(result, operand);
            }

            let result = TreeNodeRef::new_val(MathToken::Constant(result));
            if operands.is_empty() {
                // operation completed, no operands left
                std::mem::drop(borrow);
                *node = result;
            } else {
                // operation partaly complete
                operands.add(result);
            }
        }
    }
}

pub enum Step {
    DoOpOn(Vec<TreeNodeRef>),
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{TreeNodeRef, AST},
        lexer::Lexer,
        MathToken,
    };
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    fn simplify_test(expr: &str, res: TreeNodeRef) {
        let mut simplified = AST::parse(Lexer::new(expr));
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
            "2*x + x",
            TreeNodeRef::new_val(MathToken::Constant(dec!(10))),
        );
    }
}
