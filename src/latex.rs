use crate::{
    lexer::OPERATOR_MAP,
    math_tree::{MathTree, TreeNodeRef},
    MathToken,
};

impl MathTree {
    pub fn to_latex(&self) -> String {
        let mut res = String::new();

        Self::to_latex_node(self.root.clone(), &mut res);
        res
    }

    fn to_latex_node(node: TreeNodeRef, res: &mut String) {
        let borrow = node.0.borrow();

        let mut childs = borrow.operand_iter();
        Self::token_to_latex(childs.next().unwrap().1, res);

        for (_, child) in childs {
            // res +=
            let MathToken::Op(_) = node.val() else {
                panic!()
            };
            res.push(OPERATOR_MAP.get_by_right(&node.val()).unwrap().clone());

            Self::token_to_latex(child, res);
        }
    }

    fn token_to_latex(child: &TreeNodeRef, res: &mut String) {
        match child.val() {
            MathToken::Constant(c) => res.push_str(&c.to_string()),
            MathToken::Variable(var) => res.push_str(&var),
            MathToken::Op(_) => {
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
        assert_eq!(MathTree::parse("2 * x").to_latex(), "2*x");

        assert_eq!(MathTree::parse("2 * (x + 1)").to_latex(), "2*(x+1)");

        assert_eq!(
            MathTree::parse("2 * (x + 1 + (2 + 3))").to_latex(),
            "2*(x+1+2+3)"
        );

        assert_eq!(
            MathTree::parse("2 * ((x) + (1) + (2 + 3))").to_latex(),
            "2*(x+1+2+3)"
        );

        assert_eq!(
            MathTree::parse("1 + 5 + 2 * 5 + 3 + 1").to_latex(),
            "(2*5)+1+5+3+1"
        );

        assert_eq!(
            MathTree::parse("2 * 5 * 3 + 1 * 2 + 3").to_latex(),
            "(2*5*3)+(1*2)+3"
        );
    }
}
