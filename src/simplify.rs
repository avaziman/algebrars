use rust_decimal::{Decimal, MathematicalOps};

use crate::{
    ast::{TreeNodeRef, AST}, operands::Operands, MathToken, OperationToken
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
        
        for mut op in operators {
            Self::simplify_node(&mut op);
            operands.add(op);
        }
        let constants = operands.remove_constants();


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
        println!("constants {:?}", constants);
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
                // operands.push(result);
                operands.add(result);
            }
        }
        // else there is no constants to perform ops on

        // let mut operand_iter = constants.into_iter();
    }
}

pub enum Step {
    DoOpOn(Vec<TreeNodeRef>),
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use pretty_assertions::assert_eq;
    use crate::{
        ast::{TreeNodeRef, AST},
        lexer::Lexer,
        MathToken,
    };

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
            "2*x + x",
            TreeNodeRef::new_val(MathToken::Constant(dec!(10))),
        );
    }
}
