use std::collections::HashMap;

use crate::{
    arithmatic::{perform_op_constant, OperationError},
    constants::CONSTANTS_MAP,
    function::Function,
    lexer::Lexer,
    math_tree::{MathTree, ParseError, TreeNodeRef},
    MathToken, MathTokenType, OperationToken,
};
use rust_decimal::prelude::ToPrimitive;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct VariableVal {
    pub var: String,
    pub val: f64,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl VariableVal {
    pub fn new(var: String, val: f64) -> Self {
        Self { var, val }
    }
}

// type Instructions = Vec<(OperationToken, Vec<f64>)>;

// inspired by https://github.com/bertiqwerty/exmex/
// an efficient interface for calculating bulk function points uses, f64 instead of decimal and requires all variables to be given values resulting in numbers only (not expressions)
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
struct FastFunction {
    // instructions: Instructions,
    operands: Vec<f64>,
    operators: Vec<OperationToken>,
    // positions to replace with var name
    replace: HashMap<String, Vec<usize>>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl FastFunction {
    // no math tree needed
    pub fn from(fexpr: &str) -> Result<FastFunction, ParseError> {
        // already simplified
        // let mut instructions = Instructions::new();
        let mut operators = Vec::new();
        let mut operands = Vec::new();
        let mut replace = HashMap::new();
        let rpn = MathTree::reverse_polish_notation(Lexer::new(fexpr))?;

        for token in rpn.into_iter() {
            match token.kind {
                MathTokenType::Constant => operands.push(token.constant.unwrap().to_f64().unwrap()),
                MathTokenType::Variable => {
                    let var = token.variable.unwrap();
                    if let Some(c) = CONSTANTS_MAP.get(var.as_str()) {
                        operands.push(c.to_f64().unwrap());
                    } else {
                        let entry = replace.entry(var).or_insert(vec![]);
                        entry.push(operands.len());

                        operands.push(0.0);
                    }
                }
                MathTokenType::Operator => operators.push(token.operation.unwrap()),
            }
        }

        Ok(Self {
            operands,
            operators,
            replace,
        })
    }

    // faster evaluation for bulk points, uses floating point instead of deciaml, resulting in less accuracy
    pub fn evaluate_float(
        &mut self,
        // values: HashMap<String, f64>,
        values: Vec<VariableVal>,
    ) -> Result<Option<f64>, OperationError> {
        let values = HashMap::<String, f64>::from_iter(values.into_iter().map(|x| (x.var, x.val)));

        // just replace the variables and execute the instructions
        for (var, indexes) in &self.replace {
            // TODO: handle error
            let val = *values.get(var).unwrap();
            for index in indexes {
                self.operands[*index] = val;
            }
        }
        let mut operand_iter = self.operands.iter().rev();
        let mut res = *operand_iter.next().unwrap();
        for operator in self.operators.iter().rev() {
            let b = *operand_iter.next().unwrap();
            res = perform_op_constant(b, res, *operator);
        }

        Ok(Some(res))
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::{fast_function::VariableVal, function::Function, math_tree::MathTree};

    use super::FastFunction;

    #[test]
    fn fast_func_xp2() {
        // let mut fx = FastFunction::from(.unwrap();
        let mut fx = FastFunction::from("x^2").unwrap();

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), 6.0)]),
            Ok(Some(36.0))
        );

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), -2.0)]),
            Ok(Some(4.0))
        );
    }

    #[test]
    fn fast_func_xpx() {
        let mut fx = FastFunction::from("x^x").unwrap();

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), 6.0)]),
            Ok(Some(46656.0))
        );

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), 1.0)]),
            Ok(Some(1.0))
        );
    }

    #[test]
    fn fast_func_epxp2() {
        let mut fx = FastFunction::from("e^(x^2)").unwrap();

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), 0.0)]),
            Ok(Some(1.0))
        );

        // assert_eq!(
        //     fx.evaluate_float(vec![VariableVal::new("x".to_string(), 1.0)]),
        // Ok(Some(std::f64::consts::E))
        // );
    }
}
