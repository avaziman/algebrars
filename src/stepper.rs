// stepper is responsible for describing the STEPS that led the algebrars to solve or simplify an expression or equation
// > STEPS are designed to be programmatically structured in a way that will:
//      allow software to recite its steps and also be translatable to understandable human algebra rules
// every modification of the MathTree requires a STEP to explain it

use crate::{
    arithmatic::OpDescription,
    math_tree::{MathTree, TreeNodeRef},
};

type NodePos = Vec<usize>;

#[derive(Debug, PartialEq)]
pub enum Step {
    PerformOp(Option<OpDescription>),
    FactorOut(TreeNodeRef),
}

#[derive(Debug, PartialEq)]
pub struct Steps(Vec<Step>);

impl Steps {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn step(&mut self, operands: (&TreeNodeRef, &TreeNodeRef), res: &TreeNodeRef, step: Step) {
        self.0.push(step);
    }
}

#[cfg(test)]
pub mod tests {
    use rust_decimal_macros::dec;

    use crate::{
        math_tree::{MathTree, TreeNodeRef},
        stepper::Steps,
    };

    use super::Step;

    // fn steps_from_it(it: impl Iterator<Item = Step>) -> Steps {
    //     let steps = Steps::new();
    //     for i in it {
    //         steps.step(operands, res, step)
    //     }
    // }

    fn steps_test(expr: &str, res: TreeNodeRef, steps_check: Steps) {
        let mut simplified = MathTree::parse(expr);
        let mut steps = Steps::new();
        simplified.simplify(&mut steps);

        assert_eq!(simplified.root, res);
        assert_eq!(steps, steps_check);
    }

    #[test]
    fn steps() {
        // steps_test("1 + 2", TreeNodeRef::constant(dec!(3)), Steps::new()); 
    }
}