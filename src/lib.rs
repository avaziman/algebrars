pub mod lexer;
pub mod ast;

use rust_decimal::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Minus,
    Plus,
    Multiply,
    Divide,
    FractionDivide,
    Pow,
    Root,
}


#[derive(Debug, PartialEq, Clone)]
enum MathToken {
    Constant(Decimal),
    Variable(String),
    Op(Operation),
}


