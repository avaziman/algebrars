use std::{cell::RefCell, io::empty, rc::Rc};

use crate::{lexer::Lexer, MathToken, Operation};

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: MathToken,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

#[derive(Clone)]
pub struct TreeNodeRef(pub Rc<RefCell<TreeNode>>);

impl PartialEq for TreeNodeRef {
    fn eq(&self, other: &TreeNodeRef) -> bool {
        self.0.borrow().val == other.0.borrow().val
            && self.0.borrow().left == other.0.borrow().left
            && self.0.borrow().right == other.0.borrow().right
    }
}

impl std::fmt::Debug for TreeNodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}

impl TreeNodeRef {
    pub fn new_val(token: MathToken) -> Self {
        Self(Rc::new(RefCell::new(TreeNode::new_val(token))))
    }

    pub fn new_vals(
        token: MathToken,
        left: Option<TreeNodeRef>,
        right: Option<TreeNodeRef>,
    ) -> Self {
        Self(Rc::new(RefCell::new(TreeNode::new_vals(
            token, left, right,
        ))))
    }
}

impl TreeNode {
    pub fn new_val(token: MathToken) -> TreeNode {
        Self {
            val: token,
            left: None,
            right: None,
        }
    }

    pub fn new_vals(
        token: MathToken,
        left: Option<TreeNodeRef>,
        right: Option<TreeNodeRef>,
    ) -> TreeNode {
        Self {
            val: token,
            left,
            right,
        }
    }
}

#[derive(Debug)]
struct AST {
    root: TreeNodeRef,
}

impl AST {
    pub fn reverse_polish_notation(lexer: Lexer) -> Vec<MathToken> {
        let mut output = Vec::new();
        let mut operators: Vec<Operation> = Vec::new();
        println!("{:?}!", lexer.tokens);

        'outer: for token in lexer.tokens.into_iter() {
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

    pub fn parse(lexer: Lexer) -> Self {
        let rpn = Self::reverse_polish_notation(lexer);
        let mut nodes = Vec::new();

        for token in rpn.into_iter() {
            if let MathToken::Op(_op) = &token {
                // let mut operand_iter = operator_it.clone().rev();
                // if node.is_none() {
                // check if unary op
                let right = nodes.pop();
                let left = nodes.pop();

                nodes.push(TreeNodeRef::new_vals(token, left, right))

                //     node = Some(TreeNode::new_vals(token, left, right));
                // }else {

                // }
            } else {
                nodes.push(TreeNodeRef::new_val(token));
            }
        }

        AST {
            root: nodes.pop().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn rpn() {
        let lexer = Lexer::new("2 * x");
        assert_eq!(
            AST::reverse_polish_notation(lexer.clone()),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Op(Operation::Multiply),
            ]
        );

        assert_eq!(
            AST::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(Operation::Multiply),
                Some(TreeNodeRef::new_val(MathToken::Constant(dec!(2)))),
                Some(TreeNodeRef::new_val(MathToken::Variable("x".to_string())))
            )
        );
    }

    #[test]
    fn rpn_precedence() {
        let lexer = Lexer::new("2 * x + 1");
        assert_eq!(
            AST::reverse_polish_notation(lexer.clone()),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Op(Operation::Multiply),
                MathToken::Constant(dec!(1)),
                MathToken::Op(Operation::Add),
            ]
        );

        assert_eq!(
            AST::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(Operation::Add),
                Some(TreeNodeRef::new_vals(
                    MathToken::Op(Operation::Multiply),
                    Some(TreeNodeRef::new_val(MathToken::Constant(dec!(2)))),
                    Some(TreeNodeRef::new_val(MathToken::Variable("x".to_string())))
                )),
                Some(TreeNodeRef::new_val(MathToken::Constant(dec!(1)))),
            )
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
        let lexer = Lexer::new("2 * (4 + (x + 1))");
        assert_eq!(
            AST::reverse_polish_notation(lexer.clone()),
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

        assert_eq!(
            AST::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(Operation::Multiply),
                Some(TreeNodeRef::new_val(MathToken::Constant(dec!(2)))),
                Some(TreeNodeRef::new_vals(
                    MathToken::Op(Operation::Add),
                    Some(TreeNodeRef::new_val(MathToken::Constant(dec!(4)))),
                    Some(TreeNodeRef::new_vals(
                        MathToken::Op(Operation::Add),
                        Some(TreeNodeRef::new_val(MathToken::Variable("x".to_string()))),
                        Some(TreeNodeRef::new_val(MathToken::Constant(dec!(1)))),
                    ))
                )),
            )
        );
    }
}
