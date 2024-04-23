use crate::math_tree::MathTree;

// multiplying both sides by zero is illegal
struct Equation {
    left: MathTree,
    right: MathTree,
}

enum EquationSolution {
    NoSolution,
}

impl Equation {
    // pub fn solve() -> EquationSolution { // ∅ - null sign (empty set)
    // }
}

#[cfg(test)]
mod tests {
    use crate::equations::Equation;

    pub fn equation_test() {
        // assert_eq!(Equation)
    }
}
