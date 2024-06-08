use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    function::function::Function,
    math_tree::{MathTree, ParseError, TreeNodeRef, TreePos},
    stepper::Steps,
    MathToken, OperationToken,
};

pub struct Equation {
    left: MathTree,
    right: MathTree,
}

// Quadratic formula ax^2 + bx + c = 0; 0 = (-b ± sqrt(b^2 - 4ac))/2a
// // discriminant: Δ = (-b ± sqrt(b^2 - 4ac))
// x^2 = 0; x = ±sqrt(x)

// enum SolvingPlan {
//     // goal is to isolate the variable
//     IsolateVariable,
//     QuadraticFormula,
// }

struct EquationStep(OperationToken, TreeNodeRef);
// perform a math operation on both sides (restrictions need to apply)
// Op,
// MoveRight,
// MoveLeft
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EquationSolution {
    // ∅ - null sign (empty set)
    NoSolution,
    SolutionsFor(TreeNodeRef, Vec<TreeNodeRef>),
}

impl Equation {
    pub fn new(left: MathTree, right: MathTree) -> Self {
        Self { left, right }
    }

    pub fn parse(equation: &str) -> Result<Equation, ParseError> {
        let Some((left, right)) = equation.split("=").collect_tuple() else {
            return Err(ParseError::MissingOperand);
        };

        Ok(Self::new(MathTree::parse(left)?, MathTree::parse(right)?))
    }

    pub fn solve(&mut self) -> EquationSolution {
        // concentrate the variable on the left side

        // if right_variables.len() > right_variables.len() {
        //     self.flip_sides();
        // }

        let mut steps = Steps::new();
        self.right.simplify(&mut steps).unwrap();
        self.left.simplify(&mut steps).unwrap();
        let left_variables = Function::scan_variables(&self.left.root);
        let right_variables = Function::scan_variables(&self.right.root);

        let var = MathToken::Variable(String::from("x").into());

        // move all non variables from left to right
        // for _ in 0..2 {
        loop {
            let mut to_eliminate = Vec::new();
            if let MathToken::Operation(op) = self.left.root.val() {
                let borrow = self.left.root.borrow();

                to_eliminate.push((
                    op,
                    borrow
                        .calculate_iter()
                        .filter_map(|(_, x)| match MathTree::find_node(x, &var) {
                            Some(_) => None,
                            None => Some(x.clone()),
                        })
                        .collect_vec(),
                ));

                // println!("SIMPLIFIED {:#?}", self.left.root);
            }

            // move all variables from right to left
            if let MathToken::Operation(op) = self.right.root.val() {
                self.right.simplify(&mut steps).unwrap();
                let borrow = self.right.root.borrow();

                to_eliminate.push((
                    op,
                    borrow
                        .calculate_iter()
                        .filter_map(|(_, x)| match MathTree::find_node(x, &var) {
                            Some(_) => Some(x.clone()),
                            None => None,
                        })
                        .collect_vec(),
                ));

                // println!("{:#?}", self.right.root);
            }

            if to_eliminate.is_empty() {
                break;
            }

            println!("ELIMINATING {:#?}", to_eliminate);
            for (op, elim) in to_eliminate {
                for e in elim {
                    self.add_op(op.opposite(), e);
                }
            }


            self.left.simplify(&mut steps);
            self.right.simplify(&mut steps);
            println!("SIMPLIFIED L {:#?} R {:#?}", self.left.to_latex(), self.right.to_latex());
        }

        // we have isolated the variable
        // if right_variables.is_empty() {
            // TODO: handle err

            self.left.simplify(&mut steps).unwrap();
            self.right.simplify(&mut steps).unwrap();

            return EquationSolution::SolutionsFor(
                self.left.root.clone(),
                vec![self.right.root.clone()],
            );
    
        // }

        // eliminate variables from right

        EquationSolution::NoSolution
    }

    // pub fn opposite_operator(op: OperationToken) -> OperationToken {
    //     OPPOSITE_OPERATOR[op as usize]
    // }

    fn add_op(&mut self, op_token: OperationToken, node: TreeNodeRef) {
        // apply the operation to both sides
        // TODO: multiplying both sides by zero illegal!

        self.right.add_op(op_token, node.clone());
        self.left.add_op(op_token, node);
    }

    // fn move_variable_left(&mut self) {}

    // fn move_variable_left_node() {}

    // fn move_left(_tree_pos: TreePos) {}

    pub fn flip_sides(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::{
        equations::Equation,
        math_tree::{ParseError, TreeNodeRef},
        MathToken,
    };
    use pretty_assertions::assert_eq;

    use super::EquationSolution;

    fn equation_test(equation: &str, res: EquationSolution) {
        let mut equation = Equation::parse(equation).unwrap();

        assert_eq!(equation.solve(), res);
    }

    fn equation_test_single_x(equation: &str, res: TreeNodeRef) {
        equation_test(
            equation,
            EquationSolution::SolutionsFor(
                TreeNodeRef::new_val(MathToken::Variable("x".to_string().into())),
                vec![res],
            ),
        );
    }

    #[test]
    pub fn simple_equation() -> Result<(), ParseError> {
        // assert_eq!(Equation)

        // equation_test_single_x("x = 2+5", TreeNodeRef::constant(dec!(7)));

        // equation_test_single_x("2*x = 2", TreeNodeRef::constant(dec!(1)));

        // equation_test_single_x("x + 5 = 8", TreeNodeRef::constant(dec!(3)));

        // equation_test_single_x("2 * x + 5 = 8", TreeNodeRef::constant(dec!(1.5)));

        // equation_test_single_x("2 * x + 5 = x + 4", TreeNodeRef::constant(dec!(-1)));
        
        equation_test_single_x("2 * x + 4 = x + 4", TreeNodeRef::constant(dec!(0)));
        Ok(())
    }
}
