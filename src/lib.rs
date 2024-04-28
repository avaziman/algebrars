pub mod arithmatic;
pub mod cancel_op;
pub mod equations;
pub mod expand;
pub mod factorization;
pub mod function;
pub mod latex;
pub mod lexer;
pub mod math_tree;
pub mod operands;
pub mod pattern;
pub mod simplify;
pub mod stepper;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
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
    // how many operands
    arity: u8,
    // determines which operation is performed first (bigger -> priority)
    precedence: i8,
    // whether the order of the operands changes the result
    orderless: bool,
    // associativity_left: bool,
}

impl OperationToken {
    pub fn info(&self) -> OperatorInfo {
        match self {
            OperationToken::Add => OperatorInfo {
                arity: 2,
                precedence: 1,
                orderless: true,
            },
            OperationToken::Subtract => OperatorInfo {
                arity: 2,
                precedence: 1,
                orderless: false,
            },
            OperationToken::Multiply => OperatorInfo {
                arity: 2,
                precedence: 2,
                orderless: true,
            },
            OperationToken::Divide => OperatorInfo {
                arity: 2,
                precedence: 2,
                orderless: false,
            },
            OperationToken::Pow | OperationToken::Root => OperatorInfo {
                arity: 2,
                precedence: 3,
                orderless: false,
            },
            OperationToken::FractionDivide => todo!(),
            OperationToken::LParent | OperationToken::RParent => unreachable!(),
        }
    }

    pub fn is_parenthesis(&self) -> bool {
        match self {
            OperationToken::LParent | OperationToken::RParent => true,
            _ => false,
        }
    }
}

// struct MathTokenType
// pub struct ShortString([char; 16]);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MathToken {
    pub kind: MathTokenType,
    pub constant: Option<Decimal>,
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub variable: Option<String>,
    pub operation: Option<OperationToken>,
}

impl std::fmt::Debug for MathToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            MathTokenType::Constant => write!(f, "{}", self.constant.unwrap()),
            MathTokenType::Variable => write!(f, "{}", self.variable.as_ref().unwrap()),
            MathTokenType::Operator => write!(f, "{:?}", self.operation),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MathTokenType {
    Constant,
    Variable,
    Operator,
}
// #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
// pub enum MathToken {
//     // #[wasm_bindgen(skip)]
//     Constant(Decimal),
//     Variable(String),
//     Op(OperationToken),
// }

// impl MathToken {
//     pub fn is_operator(&self) -> bool {
//         match self {
//             MathToken::Op(_) => true,
//             _ => false,
//         }
//     }

// }

impl MathToken {
    // pub fn kind(&self) -> MathTokenType {
    //     self.kind
    // }

    pub fn is_operator(&self) -> bool {
        match self.kind {
            MathTokenType::Operator => true,
            _ => false,
        }
    }

    pub fn variable(s: String) -> Self {
        Self {
            kind: MathTokenType::Variable,
            constant: None,
            variable: Some(s),
            operation: None,
        }
    }

    pub fn constant(d: Decimal) -> Self {
        Self {
            kind: MathTokenType::Constant,
            constant: Some(d),
            variable: None,
            operation: None,
        }
    }

    pub fn operator(o: OperationToken) -> Self {
        Self {
            kind: MathTokenType::Operator,
            constant: None,
            variable: None,
            operation: Some(o),
        }
    }
}