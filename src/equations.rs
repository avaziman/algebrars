use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    function::function::Function,
    math_tree::{MathTree, ParseError, TreeNodeRef, TreePos},
    stepper::Steps,
    MathToken, OperationToken, OPPOSITE_OPERATOR,
};

struct Equation {
    left: MathTree,
    right: MathTree,
}

// Quadratic formula ax^2 + bx + c = 0; 0 = (-b ± sqrt(b^2 - 4ac))/2a
// // discriminant: Δ = (-b ± sqrt(b^2 - 4ac))
// x^2 = 0; x = ±sqrt(x)

enum SolvingPlan {
    // goal is to isolate the variable
    IsolateVariable,
    QuadraticFormula,
}

struct EquationStep(OperationToken, TreeNodeRef);
// perform a math operation on both sides (restrictions need to apply)
// Op,
// MoveRight,
// MoveLeft
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum EquationSolution {
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

        // for lvar in left_variables {
        //     // lvar
        //     match right_variables {

        //     }

        // }
        // for op in self.right {

        // }
        // todo: symmetrical scan
        let mut steps = Steps::new();
        self.right.simplify(&mut steps).unwrap();
        self.left.simplify(&mut steps).unwrap();
        let left_variables = Function::scan_variables(&self.left.root);
        let right_variables = Function::scan_variables(&self.right.root);

        let var = MathToken::variable(String::from("x"));

        // move all non variables from left to right
        while let Some(op) = self.left.root.val().operation {
            let borrow = self.left.root.borrow();

            let to_eliminate = borrow
                .operand_iter()
                .filter_map(|(_, x)| match MathTree::find_node(x, &var) {
                    Some(_) => None,
                    None => Some(x.clone()),
                })
                .collect_vec();

            // if to_eliminate.is_empty() {
            //     break;
            // }

            std::mem::drop(borrow);
            for elim in to_eliminate {
                self.add_op(op.opposite(), elim);
            }
            // println!("RAW {:#?}", self.left.root);
            self.left.simplify(&mut steps).unwrap();
            // println!("SIMPLIFIED {:#?}", self.left.root);
        }

        // move all variables from right to left
        if let Some(op) = self.right.root.val().operation {
            let borrow = self.right.root.borrow();

            let to_eliminate = borrow
                .operand_iter()
                .filter_map(|(_, x)| match MathTree::find_node(x, &var) {
                    Some(_) => Some(x.clone()),
                    None => None,
                })
                .collect_vec();

            // if to_eliminate.is_empty() {
            //     break;
            // }

            std::mem::drop(borrow);
            for elim in to_eliminate {
                self.add_op(op.opposite(), elim);
            }
            self.right.simplify(&mut steps).unwrap();
            println!("{:#?}", self.right.root);
        }

        // we have isolated the variable
        if right_variables.is_empty() {
            // TODO: handle err

            self.left.simplify(&mut steps).unwrap();
            self.right.simplify(&mut steps).unwrap();

            return EquationSolution::SolutionsFor(
                self.left.root.clone(),
                vec![self.right.root.clone()],
            );
        }

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

    fn move_variable_left(&mut self) {}

    fn move_variable_left_node() {}

    fn move_left(_tree_pos: TreePos) {}

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
                TreeNodeRef::new_val(MathToken::variable("x".to_string())),
                vec![res],
            ),
        );
    }

    #[test]
    pub fn simple_equation() -> Result<(), ParseError> {
        // assert_eq!(Equation)

        equation_test_single_x("x = 2+5", TreeNodeRef::constant(dec!(7)));

        equation_test_single_x("2*x = 2", TreeNodeRef::constant(dec!(1)));

        equation_test_single_x("x + 5 = 8", TreeNodeRef::constant(dec!(3)));

        equation_test_single_x("2 * x + 5 = 8", TreeNodeRef::constant(dec!(1.5)));

        equation_test_single_x("2 * x + 5 = x + 5", TreeNodeRef::constant(dec!(0)));
        Ok(())
    }
}
