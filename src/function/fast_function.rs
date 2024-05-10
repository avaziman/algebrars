use std::collections::HashMap;

use crate::{
    arithmatic::arithmatic::{perform_op_constant, OperationError},
    constants::CONSTANTS_MAP,
    lexer::Lexer,
    math_tree::{MathTree, ParseError, TreeNodeRef},
    MathTokenType, OperationToken,
};
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use super::function::Function;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FastFunctionMathToken {
    pub val: Option<f64>,
    pub op: Option<OperationToken>,
}

impl FastFunctionMathToken {
    pub fn val(v: f64) -> Self {
        Self {
            val: Some(v),
            op: None,
        }
    }

    pub fn op(op: OperationToken) -> Self {
        Self {
            val: None,
            op: Some(op),
        }
    }
}

type FastRpn = SmallVec<[FastFunctionMathToken; 32]>;
// inspired by https://github.com/bertiqwerty/exmex/
// an efficient interface for calculating bulk function points uses, f64 instead of decimal and requires all variables to be given values resulting in numbers only (not expressions)
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct FastFunction {
    rpn: FastRpn,
    // positions to replace with var name
    replace: HashMap<String, Vec<usize>>,
}

impl MathTree {
    pub fn to_fast_rpn(&self) -> (FastRpn, HashMap<String, Vec<usize>>) {
        let mut rpn = FastRpn::new();
        let mut variables = HashMap::new();

        Self::to_fast_rpn_node(&self.root, &mut rpn, &mut variables);

        (rpn, variables)
    }
    fn to_fast_rpn_node(
        node: &TreeNodeRef,
        rpn: &mut FastRpn,
        variables: &mut HashMap<String, Vec<usize>>,
    ) {
        for (_, operand) in node.borrow().calculate_iter() {
            Self::to_fast_rpn_node(operand, rpn, variables);
        }

        let val = node.val();
        match val.kind {
            MathTokenType::Constant => rpn.push(FastFunctionMathToken::val(
                val.constant.unwrap().to_f64().unwrap(),
            )),

            MathTokenType::Variable => {
                let var = val.variable.unwrap();
                let entry = variables.entry(var).or_insert(vec![]);
                entry.push(rpn.len());

                rpn.push(FastFunctionMathToken::val(f64::MAX))
            }
            MathTokenType::Operator => rpn.push(FastFunctionMathToken::op(val.operation.unwrap())),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl FastFunction {
    // no math tree needed
    pub fn from(f: &Function) -> Result<FastFunction, ParseError> {
        // already simplified
        // let mut instructions = Instructions::new();
        let (rpn, replace) = f.simplified.to_fast_rpn();

        Ok(Self { rpn, replace })
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
            let val = match values.get(var) {
                Some(v) => *v,
                None => CONSTANTS_MAP.get(var.as_str()).unwrap().to_f64().unwrap(),
            };

            let val = FastFunctionMathToken::val(val);
            for index in indexes {
                self.rpn[*index] = val;
            }
        }
        let mut calculations_stack = SmallVec::<[f64; 32]>::new();

        for token in &self.rpn {
            if let Some(operand) = token.val {
                calculations_stack.push(operand);
            } else {
                let b = calculations_stack.pop().unwrap();
                let a = calculations_stack.pop().unwrap();

                calculations_stack.push(perform_op_constant(a, b, token.op.unwrap()));
            }
        }

        Ok(Some(calculations_stack.pop().unwrap()))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        function::{fast_function::VariableVal, function::Function},
        math_tree::MathTree,
    };

    use super::FastFunction;

    #[test]
    fn fast_func_xp2() {
        // let mut fx = FastFunction::from(Function::from(.unwrap();
        let mut fx =
            FastFunction::from(&Function::from(MathTree::parse("x^2").unwrap()).unwrap()).unwrap();

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
        let mut fx =
            FastFunction::from(&Function::from(MathTree::parse("x^x").unwrap()).unwrap()).unwrap();

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
        let mut fx =
            FastFunction::from(&Function::from(MathTree::parse("e^(x^2)").unwrap()).unwrap())
                .unwrap();

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), 0.0)]),
            Ok(Some(1.0))
        );

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), 1.0)]),
            Ok(Some(std::f64::consts::E))
        );
    }

    #[test]
    fn fast_func_xp2d2() {
        let mut fx =
            FastFunction::from(&Function::from(MathTree::parse("(x^2)/x").unwrap()).unwrap())
                .unwrap();

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), 2.0)]),
            Ok(Some(2.0))
        );

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), -5.0)]),
            Ok(Some(-5.0))
        );
    }
}
