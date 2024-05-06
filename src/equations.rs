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
        let left_variables = Function::scan_variables(&self.left.root);
        let right_variables = Function::scan_variables(&self.right.root);

        // if right_variables.len() > right_variables.len() {
        //     self.flip_sides();
        // }

        // we have isolated the variable
        if right_variables.is_empty() {
            let mut steps = Steps::new();
            // TODO: handle err
            self.right.simplify(&mut steps).unwrap();

            for (lvar, pos) in left_variables {
                // find common

                if let Some(node) = lvar.borrow().operands.iter().find(|x| Some(*x) != pos) {
                    let node =  lvar.borrow().operands[node].clone();
                    self.add_op(
                        Self::opposite_operator(lvar.val().operation.unwrap()),
                      node 
                    );

                    // self.right.simplify(&mut steps).unwrap();
                }
            }
            self.left.simplify(&mut steps).unwrap();

            return EquationSolution::SolutionsFor(
                self.left.root.clone(),
                vec![self.right.root.clone()],
            );
        }

        // eliminate variables from right

        EquationSolution::NoSolution
    }

    pub fn opposite_operator(op: OperationToken) -> OperationToken {
        OPPOSITE_OPERATOR[op as usize]
    }

    fn add_op(&mut self, op_token: OperationToken, node: TreeNodeRef) {
        // apply the operation to both sides
        // TODO: multiplying both sides by zero illegal!

        self.right.add_op(op_token, node.clone());
        self.left.add_op(op_token, node);
    }

    // pub fn add(&mut self, node: TreeNodeRef) {
    //     self.left.add(node)
    //     self.right.add(node)
    // }

    // pub fn subtract(&self, node: TreeNodeRef) {
    //     self.op(OperationToken::Subtract, node)
    // }

    // pub fn multiply(&self, node: TreeNodeRef) {
    //     self.op(OperationToken::Multiply, node)
    // }

    // pub fn divide(&self, node:TreeNodeRef) {
    //     self.op(OperationToken::Divide, node)
    // }

    // pub fn pow(&self, node: TreeNodeRef) {
    //     self.op(OperationToken::Pow, node)
    // }

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
        math_tree::{MathTree, ParseError, TreeNodeRef},
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

        // equation_test_single_x("x = 2+5", TreeNodeRef::constant(dec!(7)));

        equation_test_single_x("2*x = 2", TreeNodeRef::constant(dec!(7)));

        Ok(())
    }
}
