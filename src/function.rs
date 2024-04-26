
use itertools::Itertools;

use crate::{
    math_tree::{MathTree, TreeNodeRef, TreePos}, operands::OperandPos, stepper::Steps
};

pub struct Function {
    expression: MathTree,
    variable: Option<Vec<(TreeNodeRef, OperandPos)>>,
}

impl Function {
    pub fn from(tree: MathTree) -> Self {
        // O(n) scans whole tree

        let mut variables = Vec::new();
        Self::scan_variables_node(&tree.root, &mut variables);

        let variable = if variables.is_empty() {
            None
        } else {
            // assert!(variables
            //     .iter()
            //     .tuple_windows()
            //     .all(|(a, b)| a.val() == b.val()));
            Some(variables)
        };

        Self {
            expression: tree,
            variable,
        }
    }

    pub fn scan_variables_node(node: &TreeNodeRef, variables: &mut Vec<(TreeNodeRef, OperandPos)>) {
        let borrow = node.0.borrow();

        for (_, opr) in borrow.operands.iter() {
            Self::scan_variables_node(opr, variables);
        }

        let b = borrow
            .operands
            .variables()
            .map(|(pos, _)| (node.clone(), pos))
            .collect_vec();

        variables.extend(b);
    }

    pub fn evaluate(&mut self, val: TreeNodeRef) -> Option<TreeNodeRef> {
        let Some(variables) = &mut self.variable else {
            return Some(self.expression.root.clone());
        };

        for (parent, pos) in variables {
            parent.0.borrow_mut().operands.remove(pos.clone());
            parent.add_operand(val.clone());
        }

        let mut steps = Steps::new();
        self.expression.simplify(&mut steps);

        Some(self.expression.root.clone())
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::math_tree::{MathTree, TreeNodeRef};

    use super::Function;

    #[test]
    fn evaluate() {
        let mut fx = Function::from(MathTree::parse("x^2"));

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(4))),
            // fx.evaluate(TreeNodeRef::constant(dec!(-4))),
            Some(TreeNodeRef::constant(dec!(16)))
        );
    }
}
