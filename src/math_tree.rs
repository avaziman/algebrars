use std::{cell::RefCell, collections::HashMap, f32::consts::E, ops::Index, rc::Rc};

use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    bounds::Bound,
    lexer::Lexer,
    operands::{OperandPos, OperandsIt},
    MathToken , OperationToken,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Clone, Serialize, Deserialize)]
pub struct TreeNode {
    val: MathToken,
    // pub childs: Vec<TreeNodeRef>, // left: Option<TreeNodeRef>,
    // right: Option<TreeNodeRef>,
    // #[cfg(target_arch = "wasm32")]
    // #[wasm_bindgen(getter_with_clone)]
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    operands: Operands,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Serialize, Deserialize)]
pub struct TreeNodeRef(Rc<RefCell<TreeNode>>);

impl std::hash::Hash for TreeNodeRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // To_L self.borrow();
    }
}

impl Index<OperandPos> for TreeNode {
    type Output = TreeNodeRef;

    fn index(&self, index: OperandPos) -> &Self::Output {
        &self.operands[index]
    }
}

impl PartialEq for TreeNodeRef {
    fn eq(&self, other: &TreeNodeRef) -> bool {
        let other_borrow = other.borrow();
        self.borrow().val == other_borrow.val
            && self
                .borrow()
                .calculate_iter()
                .eq(other_borrow.calculate_iter())
    }
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("val", &self.val)
            .field(
                "operands",
                &self.calculate_iter().map(|x| x.1).collect_vec(),
            )
            .finish()
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
        Self::new_val(MathToken::Constant(dec))
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
    pub(crate) fn parse(str: &str) -> Self {
        MathTree::parse(str).unwrap().root
    }

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
    
    fn try_merge(&mut self, operand: &TreeNodeRef) -> bool {
        if let MathToken::Operation(op) = self.val {
            // merge operands that use the same operator and are orderless
            if op.info().orderless && operand.val() == self.val {
                self.operands.extend(&operand.borrow().operands);
                return true;
            }
        }

        false
    }

    // merges orderless
    pub fn add_operand(&mut self, operand: TreeNodeRef) {
        if !self.try_merge(&operand) {
            // not mergeable, add regular operand
            self.operands.push(operand);
        }
    }

    pub fn replace_operand(&mut self, op_pos: OperandPos, with: TreeNodeRef) -> bool {
        if self.try_merge(&with) {
            self.operands.remove(op_pos);
            true
        } else {
            self.operands.replace_val(op_pos, with);
            false
        }
    }

    pub fn operands_result(&mut self, a: OperandPos, b: OperandPos, res: TreeNodeRef) {
        self.operands.remove(a);
        self.operands.remove(b);
        self.add_operand(res);
    }

    pub fn remove_operand(&mut self, pos: OperandPos) {
        self.operands.remove(pos);
        if self.operands.is_empty() {
            // operation cancels out
            self.operands.push(match self.val.operation.unwrap() {
                OperationToken::Add | OperationToken::Subtract => TreeNodeRef::zero(),
                OperationToken::Multiply | OperationToken::Divide => TreeNodeRef::one(),
                _ => unreachable!(),
            })
        }
    }

    // #[wasm_bindgen(getter)]
    pub fn operands(&self) -> &Operands {
        &self.operands
    }
    pub fn operands_mut(&mut self) -> &mut Operands {
        &mut self.operands
    }
}


impl TreeNode {
    pub fn calculate_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (OperandPos, &'a TreeNodeRef)> + 'a> {
        // if let MathToken::Operation(op) = self.val{
        //     if op.info().orderless {
        //         // constants first
        //         let iter = self.operands.iter_mul();
        //         return Box::new(iter.map(|pos| (pos, &self.operands[pos])));
        //     }
        // }
        Box::new(self.operands.iter())
    }

    pub fn display_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (OperandPos, &'a TreeNodeRef)> + 'a> {
        if MathToken::Operation(OperationToken::Multiply) == self.val {
            let iter = self.operands.iter_mul();
            return Box::new(iter.map(|pos| (pos, &self.operands[pos])));
        }
        // operators then variables then constants
        // let iter = self.operands.iter();
        // return Box::new(iter.map(|pos| (pos, &self.operands[pos])));
        Box::new(self.operands.iter_order())
    }
}
pub type VarBounds = HashMap<Rc<String>, Vec<Bound>>;

// abstract syntax tree
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathTree {
    pub(crate) root: TreeNodeRef,
    pub(crate) bounds: VarBounds,
}

pub struct TreePos(pub Vec<OperandPos>);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParseError {
    MissingOperand,
    ParenthesesMismatch,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MathTree {
    pub fn parse(str: &str) -> Result<MathTree, ParseError> {
        let rpn = Self::reverse_polish_notation(Lexer::new(str))?;
        let mut nodes: Vec<TreeNodeRef> = Vec::new();

        for token in rpn.into_iter() {
            let MathToken::Operation(op) = token else {
                nodes.push(TreeNodeRef::new_val(token));
                continue;
            };

            let op_info = op.info();
            let split_at = nodes.len() - op_info.arity as usize;
            let operands = nodes.split_off(split_at);
            if operands.len() != op_info.arity as usize {
                return Err(ParseError::MissingOperand);
            }
            nodes.push(TreeNodeRef::new_vals(token, operands));
        }

        Ok(MathTree {
            root: nodes.pop().unwrap(),
            bounds: HashMap::new(),
        })
    }

    pub(crate) fn add_op(&mut self, op_token: OperationToken, node: TreeNodeRef) {
        self.root = self.root.op(op_token, node);
    }
}

impl MathTree {
    // postfix notation
    pub fn reverse_polish_notation(mut lexer: Lexer) -> Result<Vec<MathToken>, ParseError> {
        let mut output = Vec::new();
        let mut operators: Vec<OperationToken> = Vec::new();

        // there won't be two consecutive operators (not parenthesis) unless its unary +-
        // (because there must be operand before (and after) operator in prefix
        let mut insert: Vec<(usize, MathToken)> = Vec::new();
        let mut last_token: Option<MathToken> = None;
        for (i, a) in lexer.tokens.iter().enumerate() {
            if let MathToken::Operation(op) = a {
                // two cases where there can be unary operator:
                // before nothing: -x
                // before LParent: (-x)
                let unary = match last_token {
                    Some(s) => s == MathToken::Operation(OperationToken::LParent),
                    None => true,
                };

                if unary {
                    match op {
                        OperationToken::Subtract | OperationToken::Add => {
                            insert.push((i, MathToken::Constant(dec!(0))))
                        }
                        // OperationToken::Add => todo!(),
                        OperationToken::LParent => {}
                        _ => unreachable!(),
                    }
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
                MathToken::Operation(op) => {
                    if op == OperationToken::RParent {
                        while let Some(last_op) = operators.pop() {
                            if last_op == OperationToken::LParent {
                                continue 'outer;
                            } else {
                                output.push(MathToken::Operation(last_op));
                            }
                        }
                        return Err(ParseError::ParenthesesMismatch);
                    } else if op != OperationToken::LParent {
                        while let Some(last_op) = operators.last() {
                            if *last_op != OperationToken::LParent
                                && op.info().precedence <= last_op.info().precedence
                            {
                                output.push(MathToken::Operation(operators.pop().unwrap()));
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
                .map(|op| MathToken::Operation(op))
                .rev(),
        );

        Ok(output)
    }

    // O(n) where n is the amount of leafs between the root and the deired remove
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
            bounds: self.bounds.clone(),
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

    pub fn find(&self, val: &MathToken) -> Option<TreeNodeRef> {
        Self::find_node(&self.root, val)
    }

    pub fn find_node(node: &TreeNodeRef, val: &MathToken) -> Option<TreeNodeRef> {
        if node.val() == *val {
            return Some(node.clone());
        }

        let borrow = node.borrow();
        for op in borrow.operands.iter() {
            if let Some(s) = Self::find_node(&borrow.operands[op], val) {
                return Some(s);
            }
        }

        None
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
            Ok(vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string().into()),
                MathToken::Operation(OperationToken::Multiply),
            ])
        );

        assert_eq!(
            MathTree::parse(txt).unwrap().root,
            TreeNodeRef::new_vals(
                MathToken::Operation(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_val(MathToken::Variable("x".to_string().into()))
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
            Ok(vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string().into()),
                MathToken::Operation(OperationToken::Multiply),
                MathToken::Constant(dec!(1)),
                MathToken::Operation(OperationToken::Add),
            ])
        );

        assert_eq!(
            MathTree::parse(txt).unwrap().root,
            TreeNodeRef::new_vals(
                MathToken::Operation(OperationToken::Add),
                vec![
                    TreeNodeRef::new_vals(
                        MathToken::Operation(OperationToken::Multiply),
                        vec![
                            TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                            TreeNodeRef::new_val(MathToken::Variable("x".to_string().into()))
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
            Ok(vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string().into()),
                MathToken::Operation(OperationToken::Multiply),
                MathToken::Constant(dec!(1)),
                MathToken::Constant(dec!(3)),
                MathToken::Operation(OperationToken::Multiply),
                MathToken::Operation(OperationToken::Add),
                MathToken::Constant(dec!(4)),
                MathToken::Operation(OperationToken::Add),
            ])
        );
    }

    #[test]
    fn rpn_precedence_parentheses() {
        assert_eq!(
            MathTree::reverse_polish_notation(Lexer::new("2 * (x + 1)")),
            Ok(vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string().into()),
                MathToken::Constant(dec!(1)),
                MathToken::Operation(OperationToken::Add),
                MathToken::Operation(OperationToken::Multiply),
            ])
        );
    }

    #[test]
    fn rpn_precedence_double_parentheses() {
        let txt = "2 * (4 + (x + 1))";
        let lexer = Lexer::new(txt);
        // 2 * (x + 5)
        assert_eq!(
            MathTree::reverse_polish_notation(lexer.clone()),
            Ok(vec![
                MathToken::Constant(dec!(2)),
                MathToken::Constant(dec!(4)),
                MathToken::Variable("x".to_string().into()),
                MathToken::Constant(dec!(1)),
                MathToken::Operation(OperationToken::Add),
                MathToken::Operation(OperationToken::Add),
                MathToken::Operation(OperationToken::Multiply),
            ])
        );

        assert_eq!(
            MathTree::parse(txt).unwrap().root,
            TreeNodeRef::new_vals(
                MathToken::Operation(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::Operation(OperationToken::Add),
                        vec![
                            TreeNodeRef::new_val(MathToken::Constant(dec!(4))),
                            TreeNodeRef::new_val(MathToken::Variable("x".to_string().into())),
                            TreeNodeRef::new_val(MathToken::Constant(dec!(1))),
                        ]
                    )
                ],
            )
        );

        let txt = "2 * (x + 1 + (2 + 3))";
        let lexer = Lexer::new(&txt);

        assert_eq!(
            MathTree::reverse_polish_notation(lexer.clone()),
            Ok(vec![
                MathToken::Constant(dec!(2)),
                MathToken::Variable("x".to_string().into()),
                MathToken::Constant(dec!(1)),
                MathToken::Operation(OperationToken::Add),
                MathToken::Constant(dec!(2)),
                MathToken::Constant(dec!(3)),
                MathToken::Operation(OperationToken::Add),
                MathToken::Operation(OperationToken::Add),
                MathToken::Operation(OperationToken::Multiply),
            ])
        );

        assert_eq!(
            MathTree::parse(txt).unwrap().root,
            TreeNodeRef::new_vals(
                MathToken::Operation(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::Operation(OperationToken::Add),
                        vec![
                            TreeNodeRef::new_val(MathToken::Variable("x".to_string().into())),
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
            MathTree::parse(lexer).unwrap().root,
            TreeNodeRef::new_vals(
                MathToken::Operation(OperationToken::Add),
                vec![
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_val(MathToken::Constant(dec!(3))),
                    TreeNodeRef::new_vals(
                        MathToken::Operation(OperationToken::Multiply),
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
            MathTree::parse(lexer).unwrap().root,
            TreeNodeRef::new_vals(
                MathToken::Operation(OperationToken::Add),
                vec![
                    // TreeNodeRef::new_val(MathToken::Constant(dec!(0))),
                    TreeNodeRef::new_val(MathToken::Constant(dec!(2))),
                    TreeNodeRef::new_vals(
                        MathToken::Operation(OperationToken::Pow),
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