pub mod lexer;
pub mod ast;

use rust_decimal::prelude::*;


#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
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

impl Operation {
    pub fn precedence(&self) -> u8 {
        match self {
            Operation::Subtract | Operation::Add => 1,
            Operation::Multiply | Operation::Divide => 2,
            Operation::Pow | Operation::Root => 3,
            Operation::FractionDivide => todo!(),
            Operation::LParent | Operation::RParent => todo!(),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum MathToken {
    Constant(Decimal),
    Variable(String),
    Op(Operation),
}


