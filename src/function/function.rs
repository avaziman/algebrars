use itertools::Itertools;

use serde::{Deserialize, Serialize};

use crate::{
    arithmatic::arithmatic::OperationError,
    math_tree::{MathTree, TreeNodeRef},
    operands::OperandPos,
    stepper::Steps,
    MathToken,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub simplified: MathTree,
    variables: Vec<(TreeNodeRef, Option<OperandPos>)>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Function {
    pub fn from(mut tree: MathTree) -> Result<Function, OperationError> {
        // O(n) scans whole tree

        let mut steps = Steps::new();
        tree.simplify(&mut steps)?;

        let variables = Self::scan_variables(&tree.root);

        Ok(Self {
            simplified: tree,
            variables,
        })
    }

    pub fn evaluate(&mut self, val: TreeNodeRef) -> Result<Option<TreeNodeRef>, OperationError> {
        if self.variables.is_empty() {
            return Ok(Some(self.simplified.root.clone()));
        };

        // let new_tree = MathTree::copy(&self.expression.root);
        // let mut new_variables = Vec::with_capacity(variables.len());
        // TODO find more efficient way for root
        for (parent, pos) in &self.variables {
            if let Some(pos) = pos {
                parent.borrow_mut().replace_operand(*pos, val.clone());
            } else {
                // root
                parent.replace(val.clone());
            }
        }
        let mut tree = self.simplified.copy();

        let mut steps = Steps::new();
        tree.simplify(&mut steps)?;

        Ok(Some(tree.root.clone()))
    }
}

impl Function {
pub(crate) fn scan_variables(root: &TreeNodeRef) -> Vec<(TreeNodeRef, Option<OperandPos>)> {
    let mut variables = Vec::new();
    // Just X or some variable, unique case
    if let MathToken::Variable(_) = root.val() {
        variables.push((root.clone(), None));
        }
        Self::scan_variables_node(root, &mut variables);
        variables
    }

    fn scan_variables_node(
        node: &TreeNodeRef,
        variables: &mut Vec<(TreeNodeRef, Option<OperandPos>)>,
    ) {
        let borrow = node.borrow();

        for (_, opr) in borrow.calculate_iter() {
            Self::scan_variables_node(opr, variables);
        }

        let b = borrow
            .operands()
            .variables()
            .map(|pos| (node.clone(), Some(pos)))
            .collect_vec();

        variables.extend(b);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        arithmatic::arithmatic::OperationError,
        math_tree::{MathTree, TreeNodeRef},
    };
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    use super::Function;
    #[test]
    fn evaluate_x() {
        let mut fx = Function::from(MathTree::parse("x").unwrap()).unwrap();

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(0))),
            Ok(Some(TreeNodeRef::constant(dec!(0))))
        );

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(4))),
            Ok(Some(TreeNodeRef::constant(dec!(4))))
        );

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(-4))),
            Ok(Some(TreeNodeRef::constant(dec!(-4))))
        );

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(1))),
            Ok(Some(TreeNodeRef::constant(dec!(1))))
        );
    }

    #[test]
    fn evaluate_xp2() {
        let mut fx = Function::from(MathTree::parse("x^2").unwrap()).unwrap();

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(4))),
            Ok(Some(TreeNodeRef::constant(dec!(16))))
        );

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(-4))),
            Ok(Some(TreeNodeRef::constant(dec!(16))))
        );

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(1))),
            Ok(Some(TreeNodeRef::constant(dec!(1))))
        );
    }

    #[test]
    fn evaluate_xpx() {
        let mut fx = Function::from(MathTree::parse("x^x").unwrap()).unwrap();

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(3))),
            Ok(Some(TreeNodeRef::constant(dec!(27))))
        );

        // should be undefined for some reason but allow for now
        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(-2))),
            Ok(Some(TreeNodeRef::constant(dec!(0.25))))
        );

        // overflow
        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(-40))),
            Err(OperationError::Overflow)
        );
    }

    #[test]
    fn evaluate_2p2() {
        let mut fx = Function::from(MathTree::parse("2^2").unwrap()).unwrap();

        assert_eq!(
            fx.evaluate(TreeNodeRef::constant(dec!(3333))),
            Ok(Some(TreeNodeRef::constant(dec!(4))))
        );
    }
}
