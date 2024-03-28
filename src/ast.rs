use std::{cell::RefCell, rc::Rc};

use crate::{lexer::Lexer, MathToken, Operation};

#[derive(Debug, Clone)]
pub struct TreeNode {
  val: MathToken,
  left: Option<TreeNodeRef>,
  right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

struct AST {
    node: TreeNode
}

impl AST {
    pub fn parse(lexer: Lexer) -> Self{
        // let mut node = TreeNode {}
        let mut last_token = None;
        for token in lexer.tokens {
            match token {
                MathToken::Constant(_) | MathToken::Variable(_) => break,
                MathToken::Op(op) => match op {
                    Operation::Minus => todo!(),
                    Operation::Plus => todo!(),
                    Operation::Multiply => todo!(),
                    Operation::Divide => todo!(),
                    Operation::FractionDivide => todo!(),
                    Operation::Pow => todo!(),
                    Operation::Root => todo!(),
                },
            }

            last_token = Some(token);
        }

        AST {
         node   
        }
    }
}