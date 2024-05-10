#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    lexer::OPERATOR_MAP,
    math_tree::{MathTree, TreeNodeRef},
    MathTokenType,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MathTree {
    pub fn to_latex(&self) -> String {
        let mut res = String::new();

        Self::to_latex_node(self.root.clone(), &mut res);
        res
    }
}

impl TreeNodeRef {
    pub fn to_latex(&self) -> String {
        let mut res = String::new();

        MathTree::to_latex_node(self.clone(), &mut res);
        res
    }
}

impl MathTree {
    pub fn to_latex_node(node: TreeNodeRef, res: &mut String) {
        let borrow = node.borrow();

        let mut childs = borrow.display_iter();
        // Self::token_to_latex(childs.next().unwrap().1, res);
        if !node.val().is_operator() {
            Self::token_to_latex(&node, res);
            return;
        };

        // first child without operator
        if let Some((_, child)) = childs.next() {
            Self::token_to_latex(child, res);
        }

        for (_, child) in childs {
            // res +=
            res.push(OPERATOR_MAP.get_by_right(&node.val()).unwrap().clone());

            Self::token_to_latex(child, res);
        }
    }

    fn token_to_latex(child: &TreeNodeRef, res: &mut String) {
        let val = child.val();
        match val.kind {
            MathTokenType::Constant => res.push_str(&val.constant.unwrap().to_string()),
            MathTokenType::Variable => res.push_str(&val.variable.unwrap()),
            MathTokenType::Operator => {
                res.push('(');
                Self::to_latex_node(child.clone(), res);
                res.push(')');
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

        assert_eq!(MathTree::parse("2 * x").unwrap().to_latex(), "2*x");

        assert_eq!(MathTree::parse("2 * (x + 1)").unwrap().to_latex(), "2*(x+1)");

        assert_eq!(
            MathTree::parse("2 * (x + 1 + (2 + 3))").unwrap().to_latex(),
            "2*(x+1+2+3)"
        );

        assert_eq!(
            MathTree::parse("2 * ((x) + (1) + (2 + 3))").unwrap().to_latex(),
            "2*(x+1+2+3)"
        );

        assert_eq!(
            MathTree::parse("1 + 5 + 2 * 5 + 3 + 1").unwrap().to_latex(),
            "(2*5)+1+5+3+1"
        );

        assert_eq!(
            MathTree::parse("2 * 5 * 3 + 1 * 2 + 3").unwrap().to_latex(),
            "(2*5*3)+(1*2)+3"
        );
    
        assert_eq!(
            MathTree::parse("e^(x^2)").unwrap().to_latex(),
            "e^(x^2)"
        );
    }
}
