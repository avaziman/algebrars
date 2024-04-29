use std::{cell::RefCell, rc::Rc};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    lexer::Lexer,
    operands::{OperandPos, Operands, OperandsIt},
    MathToken, MathTokenType, OperationToken,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    val: MathToken,
    // pub childs: Vec<TreeNodeRef>, // left: Option<TreeNodeRef>,
    // right: Option<TreeNodeRef>,
    // #[cfg(target_arch = "wasm32")]
    // #[wasm_bindgen(getter_with_clone)]
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub operands: Operands,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Serialize, Deserialize)]
pub struct TreeNodeRef(Rc<RefCell<TreeNode>>);

impl PartialEq for TreeNodeRef {
    fn eq(&self, other: &TreeNodeRef) -> bool {
        self.borrow().val == other.borrow().val && self.borrow().operands == other.borrow().operands
    }
}

impl std::fmt::Debug for TreeNodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TreeNodeRef {
    pub fn new_val(token: MathToken) -> Self {
        Self(Rc::new(RefCell::new(TreeNode::new_val(token))))
    }

    pub fn new_vals(token: MathToken, childs: Vec<TreeNodeRef>) -> Self {
        Self(Rc::new(RefCell::new(TreeNode::new_vals(token, childs))))
    }

    pub fn constant(dec: Decimal) -> Self {
        Self::new_val(MathToken::constant(dec))
    }

    pub fn val(&self) -> MathToken {
        self.borrow().val.clone()
    }

    pub fn add_operand(&self, operand: TreeNodeRef) {
        self.borrow_mut().add_operand(operand);
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
}

impl TreeNodeRef {
    pub fn replace(&self, new: TreeNodeRef) {
        self.0.replace(new.borrow().clone());
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, TreeNode> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, TreeNode> {
        self.0.borrow_mut()
    }
}

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TreeNode {
    pub fn new_val(token: MathToken) -> TreeNode {
        let mut orderless = false;
        if let Some(op) = token.operation {
            if op.info().orderless {
                orderless = true;
            }
        }
        Self {
            val: token,
            operands: Operands::default(),
        }
    }

    pub fn new_vals(token: MathToken, childs: Vec<TreeNodeRef>) -> TreeNode {
        // let operands = Operands::from_iter(childs);
        let mut node = TreeNode::new_val(token);

        for o in childs {
            node.add_operand(o);
        }

        node
    }

    // merges orderless
    pub fn add_operand(&mut self, operand: TreeNodeRef) {
        let op_token = self.val.clone();

        if let Some(op) = op_token.operation {
            // merge operands that use the same operator and are orderless
            if op.info().orderless && operand.val() == op_token {
                self.operands.extend(&operand.borrow().operands);
                return;
            }
        }

        // not mergeable, add regular operand
        self.operands.push(operand);
    }

    // #[wasm_bindgen(getter)]
    pub fn operands(&self) -> Operands {
        self.operands.clone()
    }
}

impl TreeNode {
    pub fn operand_iter<'a>(
        &'a self,
    ) -> std::iter::Map<OperandsIt, impl FnMut(OperandPos) -> (OperandPos, &'a TreeNodeRef)> {
        let iter = if self.val == MathToken::operator(OperationToken::Multiply) {
            self.operands.iter_mul()
        } else {
            self.operands.iter()
        };

        iter.map(|pos| (pos, &self.operands[pos]))
    }
}

// abstract syntax tree
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathTree {
    pub(crate) root: TreeNodeRef,
}

pub struct TreePos(pub Vec<OperandPos>);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MathTree {
    pub fn parse(str: &str) -> Self {
        let rpn = Self::reverse_polish_notation(Lexer::new(str));
        let mut nodes: Vec<TreeNodeRef> = Vec::new();

        for token in rpn.into_iter() {
            let Some(op) = token.operation else {
                nodes.push(TreeNodeRef::new_val(token));
                continue;
            };

            let op_info = op.info();
            let split_at = nodes.len() - op_info.arity as usize;
            let operands = nodes.split_off(split_at);
            nodes.push(TreeNodeRef::new_vals(token, operands));
        }

        MathTree {
            root: nodes.pop().unwrap(),
        }
    }
}

impl MathTree {
    // postfix notation
    pub fn reverse_polish_notation(mut lexer: Lexer) -> Vec<MathToken> {
        let mut output = Vec::new();
        let mut operators: Vec<OperationToken> = Vec::new();

        // there won't be two consecutive operators (not parenthesis) unless its unary +-
        // (because there must be operand before (and after) operator in prefix
        let mut insert: Vec<(usize, MathToken)> = Vec::new();
        let mut last_token: Option<MathToken> = None;
        for (i, a) in lexer.tokens.iter().enumerate() {
            if let Some(op) = a.operation {
                // two cases where there can be unary operator:
                // before nothing: -x
                // before LParent: (-x)
                let unary = match last_token {
                    Some(s) => s.operation == Some(OperationToken::LParent),
                    None => true,
                };

                if unary {
                    match op {
                        OperationToken::Subtract | OperationToken::Add => {
                            insert.push((i, MathToken::constant(dec!(0))))
                        }
                        // OperationToken::Add => todo!(),
                        _ => {}
                    }
                }
            }
            last_token = Some(a.clone());
        }

        for (i, a) in insert {
            lexer.tokens.insert(i, a);
        }

        'outer: for token in lexer.tokens.into_iter() {
            match token.kind {
                MathTokenType::Constant | MathTokenType::Variable => output.push(token),
                MathTokenType::Operator => {
                    let op = token.operation.unwrap();
                    if op == OperationToken::RParent {
                        while let Some(last_op) = operators.pop() {
                            if last_op == OperationToken::LParent {
                                continue 'outer;
                            } else {
                                output.push(MathToken::operator(last_op));
                            }
                        }
                        panic!("Parentheses Mismatch");
                    } else if op != OperationToken::LParent {
                        while let Some(last_op) = operators.last() {
                            if *last_op != OperationToken::LParent
                                && op.info().precedence <= last_op.info().precedence
                            {
                                output.push(MathToken::operator(operators.pop().unwrap()));
                            } else {
                                break;
                            }
                        }
                    }
                    operators.push(op)
                }
            }
        }

        output.extend(
            operators
                .into_iter()
                .map(|op| MathToken::operator(op))
                .rev(),
        );

        output
    }

    // O(n) where n is the amount of leafs between the root and the desired remove
    // pub fn remove(&mut self, mut pos: TreePos) {
    //     let mut node = self.root.clone();
    //     let last_pos = pos.0.pop().expect("empty pos");

    //     for p in pos.0 {
    //         let val = node.borrow().operands[p].clone();
    //         node = val;
    //     }

    //     node.borrow_mut().operands.remove(last_pos);
    // }
    pub fn copy(&self) -> MathTree {
        MathTree {
            root: Self::copy_node(&self.root),
        }
    }
    fn copy_node(node: &TreeNodeRef) -> TreeNodeRef {
        // let res = TreeNodeRef::new_vals(node.val(), childs)
        let mut children = Vec::new();
        for (_i, c) in node.0.borrow().operands().iter_order() {
            children.push(Self::copy_node(c));
        }

        TreeNodeRef::new_vals(node.val(), children)
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
                MathToken::constant(dec!(2)),
                MathToken::variable("x".to_string()),
                MathToken::operator(OperationToken::Multiply),
            ]
        );

        assert_eq!(
            MathTree::parse(txt).root,
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                    TreeNodeRef::new_val(MathToken::variable("x".to_string()))
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
                MathToken::constant(dec!(2)),
                MathToken::variable("x".to_string()),
                MathToken::operator(OperationToken::Multiply),
                MathToken::constant(dec!(1)),
                MathToken::operator(OperationToken::Add),
            ]
        );

        assert_eq!(
            MathTree::parse(txt).root,
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Add),
                vec![
                    TreeNodeRef::new_vals(
                        MathToken::operator(OperationToken::Multiply),
                        vec![
                            TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::variable("x".to_string()))
                        ]
                    ),
                    TreeNodeRef::new_val(MathToken::constant(dec!(1))),
                ]
            )
        );
    }

    #[test]
    fn rpn_precedence2() {
        assert_eq!(
            MathTree::reverse_polish_notation(Lexer::new("2 * x + 1 * 3 + 4")),
            vec![
                MathToken::constant(dec!(2)),
                MathToken::variable("x".to_string()),
                MathToken::operator(OperationToken::Multiply),
                MathToken::constant(dec!(1)),
                MathToken::constant(dec!(3)),
                MathToken::operator(OperationToken::Multiply),
                MathToken::operator(OperationToken::Add),
                MathToken::constant(dec!(4)),
                MathToken::operator(OperationToken::Add),
            ]
        );
    }

    #[test]
    fn rpn_precedence_parentheses() {
        assert_eq!(
            MathTree::reverse_polish_notation(Lexer::new("2 * (x + 1)")),
            vec![
                MathToken::constant(dec!(2)),
                MathToken::variable("x".to_string()),
                MathToken::constant(dec!(1)),
                MathToken::operator(OperationToken::Add),
                MathToken::operator(OperationToken::Multiply),
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
                MathToken::constant(dec!(2)),
                MathToken::constant(dec!(4)),
                MathToken::variable("x".to_string()),
                MathToken::constant(dec!(1)),
                MathToken::operator(OperationToken::Add),
                MathToken::operator(OperationToken::Add),
                MathToken::operator(OperationToken::Multiply),
            ]
        );

        assert_eq!(
            MathTree::parse(txt).root,
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::operator(OperationToken::Add),
                        vec![
                            TreeNodeRef::new_val(MathToken::constant(dec!(4))),
                            TreeNodeRef::new_val(MathToken::variable("x".to_string())),
                            TreeNodeRef::new_val(MathToken::constant(dec!(1))),
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
                MathToken::constant(dec!(2)),
                MathToken::variable("x".to_string()),
                MathToken::constant(dec!(1)),
                MathToken::operator(OperationToken::Add),
                MathToken::constant(dec!(2)),
                MathToken::constant(dec!(3)),
                MathToken::operator(OperationToken::Add),
                MathToken::operator(OperationToken::Add),
                MathToken::operator(OperationToken::Multiply),
            ]
        );

        assert_eq!(
            MathTree::parse(txt).root,
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::operator(OperationToken::Add),
                        vec![
                            TreeNodeRef::new_val(MathToken::variable("x".to_string())),
                            TreeNodeRef::new_val(MathToken::constant(dec!(1))),
                            TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::constant(dec!(3))),
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
                MathToken::operator(OperationToken::Add),
                vec![
                    TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                    TreeNodeRef::new_val(MathToken::constant(dec!(3))),
                    TreeNodeRef::new_vals(
                        MathToken::operator(OperationToken::Multiply),
                        vec![
                            TreeNodeRef::new_val(MathToken::constant(dec!(4))),
                            TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::constant(dec!(3))),
                        ]
                    )
                ],
            )
        );

        let lexer = "2 + 2^2";

        assert_eq!(
            MathTree::parse(lexer).root,
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Add),
                vec![
                    // TreeNodeRef::new_val(MathToken::Constant(dec!(0))),
                    TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::operator(OperationToken::Pow),
                        vec![
                            TreeNodeRef::new_val(MathToken::constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::constant(dec!(2))),
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
