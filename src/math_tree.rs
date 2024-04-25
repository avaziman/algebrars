use std::{cell::RefCell, io::empty, rc::Rc};

use itertools::Itertools;
use rust_decimal::{prelude::Zero, Decimal};
use rust_decimal_macros::dec;

use crate::{
    lexer::Lexer,
    operands::{OperandPos, Operands},
    MathToken, OperationToken,
};

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: MathToken,
    // pub childs: Vec<TreeNodeRef>, // left: Option<TreeNodeRef>,
    // right: Option<TreeNodeRef>,
    pub operands: Operands,
}

#[derive(Clone)]
pub struct TreeNodeRef(pub Rc<RefCell<TreeNode>>);

impl PartialEq for TreeNodeRef {
    fn eq(&self, other: &TreeNodeRef) -> bool {
        self.0.borrow().val == other.0.borrow().val
            && self.0.borrow().operands == other.0.borrow().operands
        // && self.0.borrow().left == other.0.borrow().left
        // && self.0.borrow().right == other.0.borrow().right
    }
}

impl Eq for TreeNodeRef {}

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

    pub fn constant(dec: Decimal) -> Self {
        Self::new_val(MathToken::Constant(dec))
    }

    pub fn zero() -> Self {
        Self::constant(dec!(0))
    }

    pub fn one() -> Self {
        Self::constant(dec!(1))
    }

    pub fn two() -> Self {
        Self::constant(dec!(2))
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
            operands: Operands::new(),
        }
    }

    pub fn new_vals(token: MathToken, childs: Vec<TreeNodeRef>) -> TreeNode {
        let operands = Operands::from_iter(childs);
        Self {
            val: token,
            operands,
        }
    }

    pub fn operand_iter(&self) -> impl Iterator<Item = (OperandPos, &TreeNodeRef)> {
        if self.val == MathToken::Op(OperationToken::Multiply) {
            self.operands.iter_mul()
        } else {
            self.operands.iter()
        }
    }
}

// abstract syntax tree
#[derive(Debug)]
pub struct MathTree {
    pub(crate) root: TreeNodeRef,
}

pub struct TreePos(pub Vec<OperandPos>);

impl MathTree {
    // postfix notation
    pub fn reverse_polish_notation(mut lexer: Lexer) -> Vec<MathToken> {
        let mut output = Vec::new();
        let mut operators: Vec<OperationToken> = Vec::new();

        // there won't be two consecutive operators (not parenthesis) unless its unary +-
        // (because there must be operand before (and after) operator in prefix
        let mut insert: Vec<(usize, MathToken)> = Vec::new();
        let mut last_token = None;
        for (i, a) in lexer.tokens.iter().enumerate() {
            if let MathToken::Op(op) = a {
                // two cases where there can be unary operator:
                // before nothing: -x
                // before LParent: (-x)
                match last_token {
                    None | Some(MathToken::Op(OperationToken::LParent)) => {
                        match op {
                            OperationToken::Subtract | OperationToken::Add => {
                                insert.push((i, MathToken::Constant(dec!(0))))
                            }
                            // OperationToken::Add => todo!(),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            last_token = Some(a.clone());
        }

        for (i, a) in insert {
            lexer.tokens.insert(i, a);
        }

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

    pub fn parse(str: &str) -> Self {
        let rpn = Self::reverse_polish_notation(Lexer::new(str));
        let mut nodes: Vec<TreeNodeRef> = Vec::new();

        for token in rpn.into_iter() {
            let MathToken::Op(op) = token else {
                nodes.push(TreeNodeRef::new_val(token));
                continue;
            };

            // if nodes.len() == 1 {
            //     match op {
            //         // allow plus and minus to take one operand only (unary):
            //         // +x = 0+x = x
            //         // -x = 0-x = -1*x
            //         OperationToken::Add => {
            //             // x stays the same
            //             continue;
            //         }
            //         // allow one operand on minus
            //         OperationToken::Subtract => match nodes.last().unwrap().val() {
            //             MathToken::Constant(c) => {
            //                 nodes.pop();
            //                 nodes.push(TreeNodeRef::constant(-c));
            //                 continue;
            //             }
            //             // operation or variables multiply * (-1)
            //             _ => {
            //                 // nodes.pop
            //                 op = OperationToken::Multiply;
            //                 token = MathToken::Op(op);
            //                 // op_info = op.info();
            //                 nodes.push(TreeNodeRef::constant(dec!(-1)));
            //             }
            //         },
            //         _ => {}
            //     }
            // }

            let op_info = op.info();
            let split_at = nodes.len() - op_info.arity as usize;
            let mut operands = nodes.split_off(split_at);
        }

        MathTree {
            root: nodes.pop().unwrap(),
        }
    }

    // merges orderless
    pub fn add_operand(node: TreeNodeRef, operand: TreeNodeRef) {
        let mut borrow = node.0.borrow_mut();
        let op_token = node.val();

        if let MathToken::Op(op) = op_token {
            // merge operands that use the same operator and are orderless
            if op.info().orderless && node.val() == op_token {
                borrow.operands.extend(&node.0.borrow().operands);
                return;
            }
        }

        borrow.operands.add(operand);
    }

    // O(n) where n is the amount of leafs between the root and the desired remove
    pub fn remove(&mut self, mut pos: TreePos) {
        let mut node = self.root.clone();
        let last_pos = pos.0.pop().expect("empty pos");

        for p in pos.0 {
            let val = node.0.borrow().operands[p].clone();
            node = val;
        }

        node.0.borrow_mut().operands.remove(last_pos);
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
        let txt = "2 * x";
        let lexer = Lexer::new(txt);
        assert_eq!(
            MathTree::reverse_polish_notation(lexer.clone()),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Op(OperationToken::Multiply),
            ]
        );

        assert_eq!(
            MathTree::parse(txt).root,
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
        let txt = "2 * x + 1";
        let lexer = Lexer::new(txt);

        assert_eq!(
            MathTree::reverse_polish_notation(lexer.clone()),
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string()),
                MathToken::Op(OperationToken::Multiply),
                MathToken::Constant(dec!(1)),
                MathToken::Op(OperationToken::Add),
            ]
        );

        assert_eq!(
            MathTree::parse(txt).root,
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
            MathTree::reverse_polish_notation(Lexer::new("2 * x + 1 * 3 + 4")),
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
            MathTree::reverse_polish_notation(Lexer::new("2 * (x + 1)")),
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
        let txt = "2 * (4 + (x + 1))";
        let lexer = Lexer::new(txt);
        // 2 * (x + 5)
        assert_eq!(
            MathTree::reverse_polish_notation(lexer.clone()),
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
            MathTree::parse(txt).root,
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

        let txt = "2 * (x + 1 + (2 + 3))";
        let lexer = Lexer::new(&txt);

        assert_eq!(
            MathTree::reverse_polish_notation(lexer.clone()),
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
            MathTree::parse(txt).root,
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
        let lexer = "2 + 3 + 4 * 2 * 3";

        assert_eq!(
            MathTree::parse(lexer).root,
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

        let lexer = "2 + 2^2";

        assert_eq!(
            MathTree::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::Op(OperationToken::Add),
                vec![
                    // TreeNodeRef::new_val(MathToken::Constant(dec!(0))),
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::Op(OperationToken::Pow),
                        vec![
                            TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                        ]
                    )
                ],
            )
        );

        // let lexer = "-(-x)";

        // assert_eq!(
        //     MathTree::parse(lexer).root,
        //     TreeNodeRef::new_vals(
        //         MathToken::Op(OperationToken::Multiply),
        //         vec![
        //             TreeNodeRef::new_val(MathToken::Constant(dec!(-1))),
        //             TreeNodeRef::new_val(MathToken::Constant(dec!(-1))),
        //             TreeNodeRef::new_val(MathToken::Variable(String::from("x"))),
        //         ],
        //     )
        // );
    }
}
