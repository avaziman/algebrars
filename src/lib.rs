pub mod arithmatic;
pub mod bounds;
pub mod cancel_op;
pub mod constants;
pub mod equations;
pub mod expand;
pub mod factorization;
pub mod function;
pub mod geometry;
pub mod latex;
pub mod lexer;
pub mod math_tree;
pub mod operands;
pub mod pattern;
mod rewriting_rules;
pub mod simplify;
pub mod stepper;
pub mod math_json;

use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum OperationToken {
    /* order critical */
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
    Root,
    /* order critical */
    LParent,
    RParent,
}

// i think a simple match function will evaluate to this too
// pub const OPPOSITE_OPERATOR: [OperationToken; 6] = [
//     OperationToken::Subtract,
//     OperationToken::Add,
//     OperationToken::Divide,
//     OperationToken::Multiply,
//     OperationToken::Root,
//     OperationToken::Pow,
// ];

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
            OperationToken::LParent | OperationToken::RParent => unreachable!(),
        }
    }

    pub fn is_parenthesis(&self) -> bool {
        match self {
            OperationToken::LParent | OperationToken::RParent => true,
            _ => false,
        }
    }

    pub fn is_mul_or_div(&self) -> bool {
        match self {
            OperationToken::Multiply | OperationToken::Divide => true,
            _ => false,
        }
    }

    pub fn opposite(&self) -> OperationToken {
        // OPPOSITE_OPERATOR[*self as usize]
        match self {
            OperationToken::Add => OperationToken::Subtract,
            OperationToken::Subtract => OperationToken::Add,
            OperationToken::Multiply => OperationToken::Divide,
            OperationToken::Divide => OperationToken::Multiply,
            OperationToken::Pow => OperationToken::Root,
            OperationToken::Root => OperationToken::Pow,
            _ => unreachable!(),
        }
    }

    pub fn from_char(c: char) -> Option<OperationToken> {
        Some(match c {
            '+' => OperationToken::Add,
            '-' => OperationToken::Subtract,
            '/' => OperationToken::Divide,
            '*' => OperationToken::Multiply,
            '^' => OperationToken::Pow,
            '(' => OperationToken::LParent,
            ')' => OperationToken::RParent,
            _ => return None,
        })
    }

    pub fn to_char(&self) -> char {
        match self {
            OperationToken::Add => '+',
            OperationToken::Subtract => '-',
            OperationToken::Divide => '/',
            OperationToken::Multiply => '*',
            OperationToken::Pow => '^',
            OperationToken::LParent => '(',
            OperationToken::RParent => ')',
            OperationToken::Root => todo!(),
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
    pub variable: Option<Rc<String>>,
    pub operation: Option<OperationToken>,
}

impl std::fmt::Debug for MathToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            MathTokenType::Constant => write!(f, "{}", self.constant.unwrap()),
            MathTokenType::Variable => write!(f, "{}", self.variable.as_ref().unwrap()),
            MathTokenType::Operator => write!(f, "{:?}", self.operation.unwrap()),
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

    pub fn variable(s: Rc<String>) -> Self {
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
