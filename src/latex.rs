use crate::{
    ast::{TreeNodeRef, AST},
    lexer::OPERATOR_MAP,
    MathToken, OperationToken,
};

impl AST {
    pub fn to_latex(&self) -> String {
        let mut res = String::new();

        Self::to_latex_node(self.root.clone(), &mut res);
        res
    }

    fn to_latex_node(node: TreeNodeRef, res: &mut String) {
        let borrow = node.0.borrow();

        let mut childs = borrow.childs.iter();
        Self::token_to_latex(childs.next().unwrap(), res);

        for child in childs {
            // res +=
            let MathToken::Op(op) = node.val() else {
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

    use crate::{ast::AST, lexer::Lexer};

    #[test]
    pub fn simple_latex() {
        assert_eq!(AST::parse(Lexer::new("2 * x")).to_latex().as_str(), "2*x");

        assert_eq!(AST::parse(Lexer::new("2 * (x + 1)")).to_latex().as_str(), "2*(x+1)");

        assert_eq!(AST::parse(Lexer::new("2 * (x + 1 + (2 + 3))")).to_latex().as_str(), "2*(x+1+2+3)");

    }
}
