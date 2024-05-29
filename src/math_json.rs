// parsing of cortex's MathJSON format, remind infix notation with token list

// use rust_decimal::{prelude::FromPrimitive, Decimal};
// use rust_decimal_macros::dec;
// use serde_json::Value;

// use crate::{math_tree::MathTree, MathToken};

// impl MathTree {
//     pub fn from_math_json(json_tokens: Vec<Value>) -> Self {
//         let tokens = Vec::new();
//         for jtoken in json_tokens {
//             tokens.push(match jtoken {
//                 Value::Number(c) => {
//                     MathToken::constant(Decimal::from_f64(c.as_f64().unwrap()).unwrap())
//                 }
//                 Value::String(op_or_var) => todo!(),
//                 Value::Array(_) => todo!(),
//                 Value::Object(_) | Value::Null | Value::Bool(_) => unreachable!(),
//             })
//         }

//         panic!()
//     }
// o
