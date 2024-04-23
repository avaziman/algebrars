use std::any::Any;

use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use crate::{
    math_tree::{MathTree, TreeNode, TreeNodeRef},
    stepper::Step,
    MathToken, OperationToken,
};

// the operands are checked against these scenarios as they usually result in a different behavior and explanation
#[derive(Debug, Clone)]
pub enum OpDescription {
    // a == b
    EqualOperand,
    // b == 0
    ByZero,
    // b == 1
    ByOne,
    // a and b are constants
    BothConstants(Decimal, Decimal),
}

pub fn get_description(a: &TreeNodeRef, b: &TreeNodeRef) -> Option<OpDescription> {
    if a == b {
        return Some(OpDescription::EqualOperand);
    } else if let MathToken::Constant(c) = b.val() {
        if c == Decimal::ZERO {
            return Some(OpDescription::ByZero);
        } else if c == Decimal::ONE {
            return Some(OpDescription::ByOne);
        }
    }

    // if a is constant then b is necessarily constant too because of operand order
    if let MathToken::Constant(c1) = a.val() {
        if let MathToken::Constant(c2) = b.val() {
            return Some(OpDescription::BothConstants(c1, c2));
        }
    }

    None
}

impl MathTree {
    pub fn perform_op(node: &mut TreeNodeRef) {
        let MathToken::Op(op) = node.val() else {
            panic!("Not operation")
        };

        let operands = &node.0.borrow().operands;
        let mut operands_iter = operands.iter();
        // for arity 2 only
        let do_op = Self::get_op(op);
        let mut a = operands_iter.next().expect("Empty operation").clone();

        for b in operands_iter {
            let desc = get_description(&a, b);
            let step = Step::PerformOp(desc.clone());
            if let Some(res) = do_op(&a, b, desc) {
                // Self::step(node, new_node, step);
                a = res;
            }

        }
    }

    fn get_op(
        op: OperationToken,
    ) ->  fn(&TreeNodeRef, &TreeNodeRef, Option<OpDescription>) -> Option<TreeNodeRef> {
        // let desc = get_description(a, b);
        match op {
            OperationToken::Add => {
                |a: &TreeNodeRef, b, desc| match desc {
                    Some(OpDescription::BothConstants(c1, c2)) => {
                        Some(TreeNodeRef::constant(c1 + c2))
                    }
                    // x + x = 2x
                    Some(OpDescription::EqualOperand) => Some(TreeNodeRef::new_vals(
                        MathToken::Op(OperationToken::Multiply),
                        vec![TreeNodeRef::two(), a.clone()],
                    )),
                    // x + 0 = x
                    Some(OpDescription::ByZero) => Some(a.clone()),
                    // x + 1 = x + 1
                    // Some(OpDescription::ByOne)
                    _ => None,
                }
            }
            OperationToken::Subtract => {
                |a: &TreeNodeRef, b, desc| match desc {
                    Some(OpDescription::BothConstants(c1, c2)) => {
                        Some(TreeNodeRef::constant(c1 - c2))
                    }
                    // x - x = 0
                    Some(OpDescription::EqualOperand) => Some(TreeNodeRef::zero()),
                    // x - 0 = x
                    Some(OpDescription::ByZero) => Some(a.clone()),
                    // x - 1 = x - 1
                    // Some(OpDescription::ByOne)
                    _ => None,
                }
            }
            OperationToken::Multiply => {
                |a: &TreeNodeRef, b: &TreeNodeRef, desc| match desc {
                    Some(OpDescription::BothConstants(c1, c2)) => {
                        Some(TreeNodeRef::constant(c1 * c2))
                    }
                    // x * x = x^2
                    Some(OpDescription::EqualOperand) => Some(TreeNodeRef::new_vals(
                        MathToken::Op(OperationToken::Pow),
                        vec![a.clone(), TreeNodeRef::two()],
                    )),
                    // x * 0 = 0
                    Some(OpDescription::ByZero) => Some(TreeNodeRef::zero()),
                    // x * 1 = x
                    Some(OpDescription::ByOne) => Some(a.clone()),
                    _ => {
                        if let MathToken::Op(OperationToken::Pow) = a.val() {
                            if let MathToken::Op(OperationToken::Pow) = b.val() {
                                let b1 = a.0.borrow();
                                let term = b1.operands.iter().next().unwrap();
                                if term == b.0.borrow().operands.iter().next().unwrap() {
                                    // x^n * x^m = x^(n+m)
                                }
                            }
                        }
                        None
                    }
                }
            }
            OperationToken::Divide => {
                |a: &TreeNodeRef, b, desc| match desc {
                    Some(OpDescription::BothConstants(c1, c2)) => {
                        Some(TreeNodeRef::constant(c1 / c2))
                    }
                    // x / x = 1
                    Some(OpDescription::EqualOperand) => Some(TreeNodeRef::one()),
                    // x / 0 = undefined
                    Some(OpDescription::ByZero) => panic!(),
                    // x / 1 = x
                    Some(OpDescription::ByOne) => Some(a.clone()),
                    _ => None,
                }
            }
            OperationToken::Pow => {
                |a: &TreeNodeRef, b, desc| match desc {
                    Some(OpDescription::BothConstants(c1, c2)) => {
                        Some(TreeNodeRef::constant(c1.powd(c2)))
                    }
                    // x ^ x = x ^ x
                    // Some(OpDescription::EqualOperand)

                    // x ^ 0 = 1
                    Some(OpDescription::ByZero) => Some(TreeNodeRef::one()),

                    // x ^ 1 = x
                    Some(OpDescription::ByOne) => Some(a.clone()),
                    _ => {
                        // (a + b)^2 = a^2 + 2ab + b^2
                        // (a + b) * (c + d) = a(c + d) + b(c + d) = ac + ad + bc + bd
                        // if let
                        None
                    }
                }
            }
            OperationToken::FractionDivide => todo!(),
            OperationToken::Root => todo!(),
            OperationToken::LParent | OperationToken::RParent => unreachable!(),
        }
    }
}
