pub mod lexer;
pub mod ast;
pub mod expand;
pub mod simplify;

use rust_decimal::prelude::*;


#[derive(Debug, PartialEq, Clone)]
pub enum OperationToken {
    Subtract,
    Add,
    Multiply,
    Divide,
    FractionDivide,
    Pow,
    Root,
    LParent,
    RParent,
}


pub struct OperatorInfo {
    precedence: i8,
    orderless: bool,
    // associativity_left: bool,
}



impl OperationToken {
    pub fn info(&self) -> OperatorInfo {
        match self {
            OperationToken::Add => OperatorInfo {precedence: 1, orderless: true},
            OperationToken::Subtract => OperatorInfo {precedence: 1, orderless: false},
            OperationToken::Multiply => OperatorInfo {precedence: 2, orderless: true},
            OperationToken::Divide => OperatorInfo {precedence: 2, orderless: false},
            OperationToken::Pow | OperationToken::Root => OperatorInfo {precedence: 3, orderless: false},
            OperationToken::FractionDivide => todo!(),
            OperationToken::LParent | OperationToken::RParent => unreachable!(),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum MathToken {
    Constant(Decimal),
    Variable(String),
    Op(OperationToken),
}


