use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use crate::{
    math_tree::{MathTree, TreeNode, TreeNodeRef},
    operands::Operands,
    stepper::{Step, Steps},
    MathToken, OperationToken,
};

// the operands are checked against these scenarios as they usually result in a different behavior and explanation
#[derive(Debug, Clone, PartialEq)]
pub enum OpDescription {
    // a == b
    EqualOperand,
    // b == 0
    ByZero(TreeNodeRef),
    // b == 1
    ByOne(TreeNodeRef),
    // a and b are constants
    BothConstants(Decimal, Decimal),
}

pub fn get_description(a: &TreeNodeRef, b: &TreeNodeRef, orderless: bool) -> Option<OpDescription> {
    if let MathToken::Constant(c2) = b.val() {
        if let MathToken::Constant(c1) = a.val() {
            return Some(OpDescription::BothConstants(c1, c2));
        }

        if c2 == Decimal::ZERO {
            return Some(OpDescription::ByZero(a.clone()));
        } else if c2 == Decimal::ONE {
            return Some(OpDescription::ByOne(a.clone()));
        }
    } else if a == b {
        return Some(OpDescription::EqualOperand);
    }

    if orderless {
        if let MathToken::Constant(c1) = a.val() {
            if c1 == Decimal::ZERO {
                return Some(OpDescription::ByZero(b.clone()));
            } else if c1 == Decimal::ONE {
                return Some(OpDescription::ByOne(b.clone()));
            }
        }
    }
    // if a is constant then b is necessarily constant too because of operand order
    // if let MathToken::Constant(c1) = a.val() {
    //     if let MathToken::Constant(c2) = b.val() {
    //         return Some(OpDescription::BothConstants(c1, c2));
    //     }
    // }

    None
}

impl MathTree {
    pub fn perform_op(node: &mut TreeNodeRef, steps: &mut Steps) {
        let MathToken::Op(op) = node.val() else {
            panic!("Not operation")
        };

        let mut borrow = node.0.borrow_mut();
        let operands = &mut borrow.operands;
        // let mut operands_iter = operands.iter().enumerate();
        // for arity 2 only
        // let mut a = operands.pop().unwrap();
        

        let do_op = Self::get_op(&op);
        let mut remaining = Vec::new();
        loop {
            if operands.len() < 2 {
                break;
            }

            let a = operands.pop_front().unwrap();
            let b = operands.pop_front().unwrap();

            let desc = get_description(&a, &b, op.info().orderless);
            let step = Step::PerformOp(desc.clone());
            if let Some(res) = do_op(&a, &b, desc) {
                steps.step((&a, &b), &res, step);
                // a = res.clone();
                operands.add(res);
            } else {
                remaining.push(a);
                remaining.push(b);
            }
        }

        for r in remaining {
            operands.add(r);
        }

        if operands.len() == 1 {
            let val = operands.pop_front().unwrap();
            std::mem::drop(borrow);

            *node = val;
        }
    }
    // Self::perform_op(&mut op);

    fn get_op(
        op: &OperationToken,
    ) -> fn(&TreeNodeRef, &TreeNodeRef, Option<OpDescription>) -> Option<TreeNodeRef> {
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
                    Some(OpDescription::ByZero(x)) => Some(x),
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
                    Some(OpDescription::ByZero(x)) => Some(x),
                    // x - 1 = x - 1
                    // Some(OpDescription::ByOne)
                    _ => {
                        // -(-x) = x
                        // 0-x = -x = -1 * x
                        if MathToken::Constant(Decimal::ZERO) == a.val() {
                            return Some(TreeNodeRef::new_vals(
                                MathToken::Op(OperationToken::Multiply),
                                vec![TreeNodeRef::constant(dec!(-1)), b.clone()],
                            ));
                        }
                        None
                    }
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
                    Some(OpDescription::ByZero(_)) => Some(TreeNodeRef::zero()),
                    // x * 1 = x
                    Some(OpDescription::ByOne(x)) => Some(x),
                    _ => {
                        if let MathToken::Op(OperationToken::Pow) = a.val() {
                            if let MathToken::Op(OperationToken::Pow) = b.val() {
                                let b1 = a.0.borrow();
                                let term = b1.operands.iter().next().unwrap().1;
                                if term == b.0.borrow().operands.iter().next().unwrap().1 {
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
                    Some(OpDescription::ByZero(_)) => panic!(),
                    // x / 1 = x
                    Some(OpDescription::ByOne(x)) => Some(x),
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
                    Some(OpDescription::ByZero(_)) => Some(TreeNodeRef::one()),

                    // x ^ 1 = x
                    Some(OpDescription::ByOne(x)) => Some(x),
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

impl TreeNodeRef {
    pub fn add(self, node: TreeNodeRef) -> TreeNodeRef {
        TreeNodeRef::new_vals(MathToken::Op(OperationToken::Add), vec![self, node])
    }

    pub fn subtract(self, node: TreeNodeRef) -> TreeNodeRef {
        TreeNodeRef::new_vals(MathToken::Op(OperationToken::Subtract), vec![self, node])
    }
}
