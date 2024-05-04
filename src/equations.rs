use crate::{
    math_tree::{MathTree, TreeNodeRef, TreePos},
    OperationToken,
};

// multiplying both sides by zero is illegal
struct Equation {
    left: MathTree,
    right: MathTree,
}

struct EquationStep(OperationToken, TreeNodeRef);
// perform a math operation on both sides (restrictions need to apply)
// Op,
// MoveRight,
// MoveLeft
// }

enum EquationSolution {
    // ∅ - null sign (empty set)
    NoSolution,
    SolutionsFor(TreeNodeRef, Vec<TreeNodeRef>),
}


impl Equation {
    // goal is to isolate the variable
    pub fn solve(&self) -> EquationSolution {
        // concentrate the variable in the same side
        // self.left
        


        EquationSolution::NoSolution
    }

    fn move_variable_left(&mut self) {

    }

    fn move_variable_left_node() {

    }

    fn move_left(_tree_pos: TreePos) {}
    
    pub fn flip_sides(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

}

#[cfg(test)]
mod tests {
    use crate::equations::Equation;

    pub fn equation_test() {
        // assert_eq!(Equation)
        // 2x = 2 + x
    }
}
