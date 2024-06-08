use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{MathToken};
use crate::{math_tree::TreeNodeRef, OperationToken, operands::OperandPos, math_tree::MathTree};
use crate::math_tree::TreeNode;

impl TreeNodeRef {
    // x^2 + x + 3^2
    // x^x + x^2
    // TOKEN(OP(POW))
    //  TOKEN(VAR(X)), TOKEN(CONST(2))
    pub fn ordering_exponent(&self) -> Decimal {
        match &self.val() {
            MathToken::Constant(_) => dec!(0),
            MathToken::Variable(_) => dec!(1),
            MathToken::Operation(op) => {
                if op == &OperationToken::Pow {
                    let borrow = self.borrow();
                    let mut iter = borrow.calculate_iter();

                    let (_, base) = iter.next().unwrap(); //x
                    let (_, exponent) = iter.next().unwrap(); //2
                    if let MathToken::Constant(exp) = exponent.val() {
                        exp
                    }else {
                        unreachable!()
                    }
                }
                else
                {
                    dec!(0)
                } 
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude_x_squared() {
        let tree = MathTree::parse("x^2").unwrap();
        let root_magnitude = tree.root.ordering_exponent();
        
        assert_eq!(root_magnitude, dec!(2)); // The exponent of x^2 is 2
    }

    #[test]
    fn magnitude_x() {
        let tree = MathTree::parse("x").unwrap();
        let root_magnitude = tree.root.ordering_exponent();
        
        assert_eq!(root_magnitude, dec!(1)); // The exponent of x is 1
    }

    #[test]
    fn magnitude_const() {
        let tree = MathTree::parse("1+4").unwrap();
        let root_magnitude = tree.root.ordering_exponent();
        
        assert_eq!(root_magnitude, dec!(0)); // The exponent of x is 1
    }
}

