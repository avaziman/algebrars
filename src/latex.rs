use itertools::Itertools;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    math_tree::{MathTree, TreeNodeRef},
    MathToken, OperationToken,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MathTree {
    pub fn to_latex(&self) -> String {
        TreeNodeRef::to_latex(&self.root)
    }
}

impl TreeNodeRef {
    pub fn to_latex(&self) -> String {
        let mut res = String::new();

        let upper_precedence = if let Some(op) = self.val().operation {
            op.info().precedence
        } else {
            0
        };

        MathTree::to_latex_node(self.clone(), &mut res, upper_precedence);
        res
    }
}

impl MathTree {
    pub fn to_latex_node(node: TreeNodeRef, res: &mut String, precedence: i8) {
        let borrow = node.borrow();

        let mut childs = borrow.display_iter();
        // Self::token_to_latex(childs.next().unwrap().1, res);
        let MathToken::Operation(operator) = node.val() else {
            Self::token_to_latex(&node, res, precedence);
            return;
        };

        let multiply = operator == OperationToken::Multiply;

        // first childs wihtout op
        let (_, child1) = childs.next().unwrap();
        Self::token_to_latex(child1, res, precedence);
        let last_child = child1;

        for (_, child1) in childs {
            // multiply anything except (constant and constant) requires no sign
            if !multiply
                || (last_child.val().kind == MathToken::Constant
                    && child1.val().kind == MathToken::Constant)
            {
                res.push(node.val().operation.unwrap().to_char());
            }

            Self::token_to_latex(child1, res, precedence);
        }
    }

    fn token_to_latex(child: &TreeNodeRef, res: &mut String, upper_precedence: i8) {
        let val = child.val();
        match val.kind {
            MathToken::Constant => {
                let cnst = &val.constant.unwrap();
                // if cnst.is_sign_negative() {
                //     res.push('(');
                    res.push_str(&cnst.to_string());
                //     res.push(')');
                // } else {
                //     res.push_str(&cnst.to_string());
                // }
            }
            MathToken::Variable => res.push_str(&val.variable.unwrap()),
            MathToken::Operator => {
                let op = val.operation.unwrap();
                let precedence = op.info().precedence;
                let parenthesize = precedence <= upper_precedence;

                if parenthesize {
                    res.push('(');
                }

                Self::to_latex_node(child.clone(), res, precedence);

                if parenthesize {
                    res.push(')');
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::math_tree::MathTree;

    #[test]
    pub fn simple_latex() {
        assert_eq!(MathTree::parse("x").unwrap().to_latex(), "x");

        assert_eq!(MathTree::parse("2 * x").unwrap().to_latex(), "2x");
        // assert_eq!(MathTree::parse("-2 * x").unwrap().to_latex(), "-2x");

        assert_eq!(MathTree::parse("2 * (x + 1)").unwrap().to_latex(), "2(x+1)");

        assert_eq!(
            MathTree::parse("2 * (x + 1 + (2 + 3))").unwrap().to_latex(),
            "2(x+1+2+3)"
        );

        assert_eq!(
            MathTree::parse("2 * ((x) + (1) + (2 + 3))")
                .unwrap()
                .to_latex(),
            "2(x+1+2+3)"
        );

        assert_eq!(
            MathTree::parse("1 + 5 + 2 * 5 + 3 + 1").unwrap().to_latex(),
            "2*5+1+5+3+1"
        );

        assert_eq!(
            MathTree::parse("2 * 5 * 3 + 1 * 2 + 3").unwrap().to_latex(),
            "2*5*3+1*2+3"
        );

        assert_eq!(MathTree::parse("e^(x^2)").unwrap().to_latex(), "e^(x^2)");
    }
}
