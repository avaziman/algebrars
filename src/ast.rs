use std::{cell::RefCell, io::empty, rc::Rc};

use crate::{lexer::Lexer, MathToken, Operation};

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: MathToken,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl TreeNode {
    pub fn new_val(token: MathToken) -> TreeNodeRef {
        Rc::new(RefCell::new(Self {
            val: token,
            left: None,
            right: None,
        }))
    }
}

struct AST {
    node: TreeNode,
}

impl AST {
    pub fn reverse_polish_notation(lexer: Lexer) -> Vec<MathToken> {
        let mut output = Vec::new();
        let mut operators: Vec<Operation> = Vec::new();
            println!("{:?}!", lexer.tokens);

        'outer: for token in lexer.tokens.into_iter() {
            println!("{:?}", token);
            match token {
                MathToken::Constant(_) | MathToken::Variable(_) => output.push(token),
                MathToken::Op(op) => {
                    if op == Operation::RParent {
                        while let Some(last_op) = operators.pop() {
                            if last_op == Operation::LParent {
                                continue 'outer;
                            } else {
                                output.push(MathToken::Op(last_op));
                            }
                        }
                        panic!("Parentheses Mismatch");
                    } else if op != Operation::LParent {
                        while let Some(last_op) = operators.last() {
                            if *last_op != Operation::LParent
                                && op.precedence() <= last_op.precedence()
                            {
                                output.push(MathToken::Op(operators.pop().unwrap()));
                            } else {
                                break;
                            }
                        }
                    }
                    operators.push(op)
                }
            }
        }

        output.extend(operators.into_iter().map(|op| MathToken::Op(op)).rev());

        output
    }

    // pub fn parse(lexer: Lexer) -> Self{
    //     // let mut node = TreeNode {}
    //     let mut last_token = None;
    //     let mut token_iter =lexer.tokens.into_iter();
    //     while let Some(token) = token_iter.next() {
    //         match token {
    //             MathToken::Constant(_) | MathToken::Variable(_) => break,
    //             MathToken::Op(op) => match op {
    //                 Operation::Minus => todo!(),
    //                 Operation::Plus => todo!(),
    //                 Operation::Multiply => {
    //                     TreeNode {
    //                         val: MathToken::Op(Operation::Multiply),
    //                         left: last_token,
    //                         right: last_token,
    //                     }
    //                 },
    //                 Operation::Divide => todo!(),
    //                 Operation::FractionDivide => todo!(),
    //                 Operation::Pow => todo!(),
    //                 Operation::Root => todo!(),
    //             },
    //         }

    //         last_token = Some(token);
    //     }

    //     AST {
    //      node
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn rpn() {
        assert_eq!(
            AST::reverse_polish_notation(Lexer::new("2 * x")),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Op(Operation::Multiply),
            ]
        );
    }

    #[test]
    fn rpn_precedence() {
        assert_eq!(
            AST::reverse_polish_notation(Lexer::new("2 * x + 1")),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Op(Operation::Multiply),
                MathToken::Constant(dec!(1)),
                MathToken::Op(Operation::Add),
            ]
        );
    }

    #[test]
    fn rpn_precedence2() {
        assert_eq!(
            AST::reverse_polish_notation(Lexer::new("2 * x + 1 * 3 + 4")),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Op(Operation::Multiply),
                MathToken::Constant(dec!(1)),
                MathToken::Constant(dec!(3)),
                MathToken::Op(Operation::Multiply),
                MathToken::Op(Operation::Add),
                MathToken::Constant(dec!(4)),
                MathToken::Op(Operation::Add),
            ]
        );
    }

    #[test]
    fn rpn_precedence_parentheses() {
        assert_eq!(
            AST::reverse_polish_notation(Lexer::new("2 * (x + 1)")),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Constant(dec!(1)),
                MathToken::Op(Operation::Add),
                MathToken::Op(Operation::Multiply),
            ]
        );
    }

        #[test]
    fn rpn_precedence_double_parentheses() {
        assert_eq!(
            AST::reverse_polish_notation(Lexer::new("2 * (4 + (x + 1))")),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Constant(dec!(4)),
                MathToken::Variable("x".to_string()),
                MathToken::Constant(dec!(1)),
                MathToken::Op(Operation::Add),
                MathToken::Op(Operation::Add),
                MathToken::Op(Operation::Multiply),
            ]
        );
    }
}
