use std::{cell::RefCell, io::empty, rc::Rc};

use crate::{lexer::Lexer, MathToken, OperationToken};

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: MathToken,
    pub childs: Vec<TreeNodeRef>, // left: Option<TreeNodeRef>,
                                  // right: Option<TreeNodeRef>,
}

#[derive(Clone)]
pub struct TreeNodeRef(pub Rc<RefCell<TreeNode>>);

impl PartialEq for TreeNodeRef {
    fn eq(&self, other: &TreeNodeRef) -> bool {
        self.0.borrow().val == other.0.borrow().val
            && self.0.borrow().childs == other.0.borrow().childs
        // && self.0.borrow().left == other.0.borrow().left
        // && self.0.borrow().right == other.0.borrow().right
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

    pub fn new_vals(token: MathToken, childs: Vec<TreeNodeRef>) -> Self {
        Self(Rc::new(RefCell::new(TreeNode::new_vals(token, childs))))
    }

    pub fn val(&self) -> MathToken {
        self.0.borrow().val.clone()
    }

    //  pub fn right(&self) -> Option<TreeNodeRef> {
    //     self.0.borrow().left.clone()
    // }

    //     pub fn left(&self) ->  Option<TreeNodeRef> {
    //     self.0.borrow().right.clone()
    // }
}

impl TreeNode {
    pub fn new_val(token: MathToken) -> TreeNode {
        Self {
            val: token,
            // left: None,
            // right: None,
            childs: vec![],
        }
    }

    pub fn new_vals(token: MathToken, childs: Vec<TreeNodeRef>) -> TreeNode {
        Self { val: token, childs }
    }
}

// abstract syntax tree
#[derive(Debug)]
pub struct AST {
    pub(crate) root: TreeNodeRef,
}

impl AST {
    pub fn reverse_polish_notation(lexer: Lexer) -> Vec<MathToken> {
        let mut output = Vec::new();
        let mut operators: Vec<OperationToken> = Vec::new();

        'outer: for token in lexer.tokens.into_iter() {
            match token {
                MathToken::Constant(_) | MathToken::Variable(_) => output.push(token),
                MathToken::Op(op) => {
                    if op == OperationToken::RParent {
                        while let Some(last_op) = operators.pop() {
                            if last_op == OperationToken::LParent {
                                continue 'outer;
                            } else {
                                output.push(MathToken::Op(last_op));
                            }
                        }
                        panic!("Parentheses Mismatch");
                    } else if op != OperationToken::LParent {
                        while let Some(last_op) = operators.last() {
                            if *last_op != OperationToken::LParent
                                && op.info().precedence <= last_op.info().precedence
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
        let mut nodes: Vec<TreeNodeRef> = Vec::new();

        for token in rpn.into_iter() {
            if let MathToken::Op(op) = &token {
                // let mut operand_iter = operator_it.clone().rev();
                // if node.is_none() {
                // check if unary op
                // let right = nodes.pop();
                // let left = nodes.pop();
                let op_info = op.info();
                let mut operands = nodes.split_off(nodes.len() - op_info.arity as usize);

                if !op_info.orderless {
                    nodes.push(TreeNodeRef::new_vals(token, operands));

                    continue;
                }
                let last_operand = operands.iter().position(|t| t.val() == token);

                if let Some(pos) = last_operand {
                    let last_operands_node = operands.remove(pos);

                    for operand in operands {
                        // compare operators
                        let mut borrow = last_operands_node.0.borrow_mut();

                        if token == operand.val() {
                            borrow.childs.extend(operand.0.borrow().childs.clone());
                        } else {
                            borrow.childs.push(operand);
                        }
                    }

                    nodes.push(last_operands_node);
                } else {
                    nodes.push(TreeNodeRef::new_vals(token, operands));
                }
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
    // use pretty_assertions::assert_eq;
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
                MathToken::Op(OperationToken::Multiply),
            ]
        );

        assert_eq!(
            AST::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_val(MathToken::Variable("x".to_string()))
                ]
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
                MathToken::Op(OperationToken::Multiply),
                MathToken::Constant(dec!(1)),
                MathToken::Op(OperationToken::Add),
            ]
        );

        assert_eq!(
            AST::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(OperationToken::Add),
                vec![
                    TreeNodeRef::new_vals(
                        MathToken::Op(OperationToken::Multiply),
                        vec![
                            TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::Variable("x".to_string()))
                        ]
                    ),
                    TreeNodeRef::new_val(MathToken::Constant(dec!(1))),
                ]
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
                MathToken::Op(OperationToken::Multiply),
                MathToken::Constant(dec!(1)),
                MathToken::Constant(dec!(3)),
                MathToken::Op(OperationToken::Multiply),
                MathToken::Op(OperationToken::Add),
                MathToken::Constant(dec!(4)),
                MathToken::Op(OperationToken::Add),
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
                MathToken::Op(OperationToken::Add),
                MathToken::Op(OperationToken::Multiply),
            ]
        );
    }

    #[test]
    fn rpn_precedence_double_parentheses() {
        let lexer = Lexer::new("2 * (4 + (x + 1))");
        // 2 * (x + 5)
        assert_eq!(
            AST::reverse_polish_notation(lexer.clone()),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Constant(dec!(4)),
                MathToken::Variable("x".to_string()),
                MathToken::Constant(dec!(1)),
                MathToken::Op(OperationToken::Add),
                MathToken::Op(OperationToken::Add),
                MathToken::Op(OperationToken::Multiply),
            ]
        );

        assert_eq!(
            AST::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::Op(OperationToken::Add),
                        vec![
                            TreeNodeRef::new_val(MathToken::Variable("x".to_string())),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(1))),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(4))),
                        ]
                    )
                ],
            )
        );

        let lexer = Lexer::new("2 * (x + 1 + (2 + 3))");

        assert_eq!(
            AST::reverse_polish_notation(lexer.clone()),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Constant(dec!(1)),
                MathToken::Op(OperationToken::Add),
                MathToken::Constant(dec!(2)),
                MathToken::Constant(dec!(3)),
                MathToken::Op(OperationToken::Add),
                MathToken::Op(OperationToken::Add),
                MathToken::Op(OperationToken::Multiply),
            ]
        );

        assert_eq!(
            AST::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::Op(OperationToken::Add),
                        vec![
                            TreeNodeRef::new_val(MathToken::Variable("x".to_string())),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(1))),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(3))),
                        ]
                    )
                ],
            )
        );
    }

    #[test]
    fn tree_group_orderless() {
        let lexer = Lexer::new("2 + 3 + 4 * 2 * 3");

        assert_eq!(
            AST::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(OperationToken::Add),
                vec![
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_val(MathToken::Constant(dec!(3))),
                    TreeNodeRef::new_vals(
                        MathToken::Op(OperationToken::Multiply),
                        vec![
                            TreeNodeRef::new_val(MathToken::Constant(dec!(4))),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(3))),
                        ]
                    )
                ],
            )
        );
    }
}
