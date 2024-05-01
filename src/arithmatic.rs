
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use crate::{
    math_tree::{MathTree, TreeNodeRef},
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
    if let Some(c2) = b.val().constant {
        if let Some(c1) = a.val().constant {
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
        if let Some(c1) = a.val().constant {
            if c1 == Decimal::ZERO {
                return Some(OpDescription::ByZero(b.clone()));
            } else if c1 == Decimal::ONE {
                return Some(OpDescription::ByOne(b.clone()));
            }
        }
    }
    // if a is constant then b is necessarily constant too because of operand order
    // if let Some(c1) = a.val().constant {
    //     if let MathToken::Constant(c2) = b.val() {
    //         return Some(OpDescription::BothConstants(c1, c2));
    //     }
    // }

    None
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, PartialEq)]
pub enum OperationError {
    Overflow,
}

impl MathTree {
    pub fn perform_op(
        node: &mut TreeNodeRef,
        steps: &mut Steps,
    ) -> Result<Option<TreeNodeRef>, OperationError> {
        let Some(op) = node.val().operation else {
            panic!("Not operation")
        };

        let mut borrow = node.borrow_mut();
        // let operands = &mut borrow.operands;
        // let mut operands_iter = operands.iter().enumerate();
        // for arity 2 only
        // let mut a = operands.pop().unwrap();

        let do_op = Self::get_op(&op);
        let mut remaining = Vec::new();
        let orderless = op.info().orderless;
        loop {
            if borrow.operands.len() < 2 {
                break;
            }

            let a = borrow.operands.pop_front(orderless).unwrap();
            let b = borrow.operands.pop_front(orderless).unwrap();

            let desc = get_description(&a, &b, orderless);
            let step = Step::PerformOp(desc.clone());
            if let Some(res) = do_op(&a, &b, desc)? {
                steps.step((&a, &b), &res, step);
                // a = res.clone();
                // operands.add(res);
                borrow.add_operand(res);
            } else {
                remaining.push(b);
                remaining.push(a);
            }
        }

        for r in remaining {
            borrow.operands.push(r);
        }

        Ok(if borrow.operands.len() == 1 {
            let val = borrow.operands.pop_front(true).unwrap();
            std::mem::drop(borrow);
            // replacing can only be done through operands as it may change token type
            // operation is complete, this is the single result
            Some(val)
            // node.replace(val);
        } else {
            // there are still operands left
            None
        })
    }
    // Self::perform_op(&mut op);

    fn get_op(
        op: &OperationToken,
    ) -> fn(
        &TreeNodeRef,
        &TreeNodeRef,
        Option<OpDescription>,
    ) -> Result<Option<TreeNodeRef>, OperationError> {
        // let desc = get_description(a, b);
        match op {
            OperationToken::Add => {
                |a: &TreeNodeRef, _b, desc| {
                    Ok(match desc {
                        Some(OpDescription::BothConstants(c1, c2)) => {
                            Some(TreeNodeRef::constant(c1 + c2))
                        }
                        // x + x = 2x
                        Some(OpDescription::EqualOperand) => Some(TreeNodeRef::new_vals(
                            MathToken::operator(OperationToken::Multiply),
                            vec![a.clone(), TreeNodeRef::two()],
                        )),
                        // x + 0 = x
                        Some(OpDescription::ByZero(x)) => Some(x),
                        // x + 1 = x + 1
                        // Some(OpDescription::ByOne)
                        _ => None,
                    })
                }
            }
            OperationToken::Subtract => {
                |a: &TreeNodeRef, b, desc| {
                    Ok(match desc {
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
                            if MathToken::constant(Decimal::ZERO) == a.val() {
                                return Ok(Some(TreeNodeRef::new_vals(
                                    MathToken::operator(OperationToken::Multiply),
                                    vec![TreeNodeRef::constant(dec!(-1)), b.clone()],
                                )));
                            }
                            None
                        }
                    })
                }
            }
            OperationToken::Multiply => {
                |a: &TreeNodeRef, _b: &TreeNodeRef, desc| {
                    Ok(match desc {
                        Some(OpDescription::BothConstants(c1, c2)) => {
                            Some(TreeNodeRef::constant(c1 * c2))
                        }
                        // x * x = x^2
                        Some(OpDescription::EqualOperand) => Some(TreeNodeRef::new_vals(
                            MathToken::operator(OperationToken::Pow),
                            vec![a.clone(), TreeNodeRef::two()],
                        )),
                        // x * 0 = 0
                        Some(OpDescription::ByZero(_)) => Some(TreeNodeRef::zero()),
                        // x * 1 = x
                        Some(OpDescription::ByOne(x)) => Some(x),
                        _ => {
                            // if let Some(OperationToken::Pow) = a.val().operation {
                            //     if let MathToken::Op(OperationToken::Pow) = b.val() {
                            //         let b1 = a.borrow();
                            //         let term = b1.operands.iter().next().unwrap().1;
                            //         if term == b.borrow().operands.iter().next().unwrap().1 {
                            //             // x^n * x^m = x^(n+m)
                            //         }
                            //     }
                            // }
                            None
                        }
                    })
                }
            }
            OperationToken::Divide => {
                |_a: &TreeNodeRef, _b, desc| {
                    Ok(match desc {
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
                    })
                }
            }
            OperationToken::Pow => {
                |_a: &TreeNodeRef, _b, desc| {
                    Ok(match desc {
                        Some(OpDescription::BothConstants(c1, c2)) => {
                            Some(TreeNodeRef::constant(match c1.checked_powd(c2) {
                                Some(k) => k,
                                None => return Err(OperationError::Overflow),
                            }))
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
                    })
                }
            }
            OperationToken::FractionDivide => todo!(),
            OperationToken::Root => todo!(),
            OperationToken::LParent | OperationToken::RParent => unreachable!(),
        }
    }
}

pub fn perform_op_constant<
    T: std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Pow,
>(
    a: T,
    b: T,
    op: OperationToken,
) -> T {
    match op {
        OperationToken::Subtract => a - b,
        OperationToken::Add => a + b,
        OperationToken::Multiply => a * b,
        OperationToken::Divide => a / b,
        OperationToken::Pow => a.pow(b),
        OperationToken::Root => todo!(),
        _ => unreachable!(),
    }
}

impl TreeNodeRef {
    pub fn add(self, node: TreeNodeRef) -> TreeNodeRef {
        TreeNodeRef::new_vals(MathToken::operator(OperationToken::Add), vec![self, node])
    }

    pub fn subtract(self, node: TreeNodeRef) -> TreeNodeRef {
        TreeNodeRef::new_vals(
            MathToken::operator(OperationToken::Subtract),
            vec![self, node],
        )
    }
}

trait Pow {
    fn pow(&self, b: Self) -> Self;
}

impl Pow for f64 {
    fn pow(&self, b: Self) -> Self {
        self.powf(b)
    }
}

impl Pow for Decimal {
    fn pow(&self, b: Self) -> Self {
        self.powd(b)
    }
}
