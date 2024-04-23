// stepper is responsible for describing the STEPS that led the algebrars to solve or simplify an expression or equation
// > STEPS are designed to be programmatically structured in a way that will:
//      allow software to recite its steps and also be translatable to understandable human algebra rules
// every modification of the MathTree requires a STEP to explain it

use crate::{arithmatic::OpDescription, math_tree::{MathTree, TreeNodeRef}};

type NodePos = Vec<usize>;

pub enum Step {
    PerformOp(Option<OpDescription>),
    FactorOut(TreeNodeRef)
}



impl MathTree {
    pub fn step(node: TreeNodeRef, new_node: TreeNodeRef, step: Step) {

        
    }
}