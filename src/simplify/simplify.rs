use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    arithmatic::arithmatic::{perform_op, OperationError},
    constants::CONSTANTS_MAP,
    math_tree::{MathTree, TreeNodeRef, VarBounds},
    stepper::Steps,
    OperationToken,
    MathToken,
};

use super::symmetry::symmetrical_scan;

// since contrary to addition, substraction is not an orderless operation,
// for simplification purposes, it is easier to represent substration as addition of a negative term
// this allows for grouping of addition and substraction

impl MathTree {
    pub fn simplify(&mut self, steps: &mut Steps) -> Result<(), OperationError> {
        println!("Simplifying: {:?}", self.to_latex());
        while let Some(complete) = Self::simplify_node(&mut self.root, steps, &mut self.bounds)? {
            self.root = complete;
        }

        Ok(())
    }

    fn simplify_node(
        node: &mut TreeNodeRef,
        steps: &mut Steps,
        bounds: &mut VarBounds,
    ) -> Result<Option<TreeNodeRef>, OperationError> {
        // let node = &mut self.root;
        let val = node.val();

        let MathToken::Operation(op_token) = val else {
            return Ok(None);
        };
        

        if val == MathToken::Operation(OperationToken::Divide) {
            symmetrical_scan(node.clone());
        } 
        // flatten
        if node.borrow().operands().len() == 1 {
            let val = node
            .borrow()
            .calculate_iter()
            .map(|x| x.1.clone())
            .next()
                .unwrap();
            
            // node.borrow_mut().replace_operand(op_pos, val);
            return Ok(Some(val));
        }
        let mut borrow = node.borrow_mut();
        
        // let mut operators = borrow.operands().operators().collect_vec();
        // let mut multipliers = Vec::new();
        let mut skip = 0;
        for op_pos in borrow.operands().operators().collect_vec() {
            let mut op = borrow[op_pos].clone();
            skip += 1;
            
            // possibly simplified to an operator and can be simplified
            if let Some(complete) = Self::simplify_node(&mut op, steps, bounds)? {
                // either there is new operator or one is gone
                if complete.val().is_operator() {
                    skip -= 1;
                }
                borrow.replace_operand(op_pos, complete);
                
                // node is operator and both operators equal
                if complete.val() == node.val() && op_token.info().orderless {
                    borrow.operands().push(complete.operands().clone());
                }
                    
                    std::mem::drop(borrow);
                return Self::simplify_node(node, steps, bounds);
            }
        }
        
        // inject constants
        // for v in borrow.operands.variables().collect_vec() {
            //     // if val.kind == MathToken::Variable {
                //     let val = borrow[v].val();

        //     let var = val.variable.as_ref().unwrap();
        //     if let Some(c) = CONSTANTS_MAP.get(var.as_str()) {
            //         borrow.operands.replace_val(v, TreeNodeRef::constant(*c));
            //     }
            //     // }
            // }
            std::mem::drop(borrow);
            if val == MathToken::Operation(OperationToken::Add) {
                if let Some(factored) = MathTree::factorize_node(node.clone()) {
                    println!("{} FACTORED TO {}", node.to_latex(), factored.to_latex());
                    return Ok(Some(factored));
                }
            }
            
 
            // println!("simplifying {:#?}", borrow);
            
            Ok(perform_op(bounds, node, steps)?)
        }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        math_tree::{MathTree, TreeNodeRef},
        stepper::Steps,
        MathToken, OperationToken,
    };
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    pub fn simplify_test(expr: &str, res: TreeNodeRef) {
        let mut simplified = MathTree::parse(expr).unwrap();
        let mut steps = Steps::new();
        if let Err(e) = simplified.simplify(&mut steps) {
            panic!("{:?}", e);
        }

        assert_eq!(simplified.root, res);
    }

    pub fn simplify_test_latex(expr: &str, res: &str) {
        let mut simplified = MathTree::parse(expr).unwrap();
        let mut steps = Steps::new();
        if let Err(e) = simplified.simplify(&mut steps) {
            panic!("{:?}", e);
        }
        assert_eq!(simplified.to_latex(), res);
    }

    #[test]
    fn simplify_constants() {
        simplify_test("1 + 2", TreeNodeRef::constant(dec!(3)));

        simplify_test("1 + 2 + 3", TreeNodeRef::constant(dec!(6)));

        simplify_test("1 + 2*2 + 3", TreeNodeRef::constant(dec!(8)));

        simplify_test("2 + 2^2", TreeNodeRef::constant(dec!(6)));

        simplify_test("2 + 2^3", TreeNodeRef::constant(dec!(10)));
    }

    #[test]
    fn simplify_x() {
        simplify_test(
            "1*x",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
        );

        simplify_test("0*x", TreeNodeRef::constant(dec!(0)));

        simplify_test(
            "0 + x",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
        );

        // TODO: !!
        // simplify_test(
        //     "x + x",
        //     TreeNodeRef::new_vals(
        //         MathToken::operator(OperationToken::Multiply),
        //         vec![
        //             TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        //             TreeNodeRef::constant(dec!(2)),
        //         ],
        //     ),
        // );

        simplify_test(
            "x + 5 - 5",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
        );

        simplify_test(
            "x - 5 + 5",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
        );

        simplify_test("(2*x)/(2*x)", TreeNodeRef::one());

        simplify_test(
            "2*x/2",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
        );

        simplify_test(
            "2*x + x",
            TreeNodeRef::new_vals(
                MathToken::Operation(OperationToken::Multiply),
                vec![
                    TreeNodeRef::constant(dec!(3)),
                    TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
                ],
            ),
        );
    }

    #[test]
    fn zero_and_double_add_subs() {
        simplify_test(
            "+x",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
        );

        simplify_test(
            "-x",
            TreeNodeRef::new_vals(
                MathToken::Operation(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
                    TreeNodeRef::constant(dec!(-1)),
                ],
            ),
        );

        simplify_test(
            "+(+x)",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
        );

        simplify_test(
            "-(-x)",
            TreeNodeRef::new_val(MathToken::Variable(String::from("x").into())),
        );

        simplify_test("-(-2)", TreeNodeRef::constant(dec!(2)));

        // // lex: 5 sub ( sub 2 )
        // // pf: 52-- sub(sub(5, 2)) = sub(3) = -3 WRONG!
        // // pf: 5-2- sub(5, sub(2)) = sub(5, -2) = 7 right => -2 needs to be parsed as a decimal not substract
        simplify_test("5-(-2)", TreeNodeRef::constant(dec!(7)));
    }
}
