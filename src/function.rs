use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    math_tree::{MathTree, TreeNodeRef},
    operands::OperandPos,
    stepper::Steps,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub expression: MathTree,
    variable: Option<Vec<(TreeNodeRef, OperandPos)>>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

    
    pub fn evaluate(&mut self, val: TreeNodeRef) -> Option<TreeNodeRef> {
        let Some(variables) = &mut self.variable else {
            return Some(self.expression.root.clone());
        };

        // let new_tree = MathTree::copy(&self.expression.root);
        // let mut new_variables = Vec::with_capacity(variables.len());
        for (parent, pos) in variables {
            parent.borrow_mut().operands.replace_val(*pos, val.clone());
            // new_variables.push((parent, ));
        }
        let mut tree = self.expression.copy();
        
        let mut steps = Steps::new();
        tree.simplify(&mut steps);
        
        Some(tree.root.clone())
    }
}

impl Function {
    pub fn scan_variables_node(node: &TreeNodeRef, variables: &mut Vec<(TreeNodeRef, OperandPos)>) {
        let borrow = node.borrow();
    
        for (_, opr) in borrow.operand_iter() {
            Self::scan_variables_node(opr, variables);
        }
    
        let b = borrow
            .operands
            .variables()
            .map(|pos| (node.clone(), pos))
            .collect_vec();
    
        variables.extend(b);
    }
}
// #[cfg(target_arch = "wasm32")]
// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// pub fn export_function(expr: &str) -> JsValue {
//     serde_wasm_bindgen::to_value(&Function::from(MathTree::parse(expr))).unwrap()
// }
#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use pretty_assertions::assert_eq;
    use crate::math_tree::{MathTree, TreeNodeRef};

    use super::Function;

    #[test]
    fn evaluate() {
        let mut fx = Function::from(MathTree::parse("x^2"));

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(4))),
            Some(TreeNodeRef::constant(dec!(16)))
        );

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(-4))),
            Some(TreeNodeRef::constant(dec!(16)))
        );


        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(1))),
            Some(TreeNodeRef::constant(dec!(1)))
        );
    }
}
