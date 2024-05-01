use std::collections::HashMap;

use crate::{
    arithmatic::{perform_op_constant, OperationError},
    function::Function,
    math_tree::TreeNodeRef,
    MathTokenType, OperationToken,
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

type Instructions = Vec<(OperationToken, Vec<f64>)>;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
struct FastFunction {
    instructions: Instructions,
    // positions to replace with var name
    replace: HashMap<String, Vec<(usize, usize)>>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl FastFunction {
    pub fn from(f: Function) -> Self {
        // already simplified
        let mut instructions = Instructions::new();
        let mut replace = HashMap::new();

        Self::get_instructions(&f.expression.root, &mut instructions, &mut replace);

        Self {
            instructions,
            replace,
        }
    }

    fn get_instructions(
        node: &TreeNodeRef,
        instructions: &mut Instructions,
        replace: &mut HashMap<String, Vec<(usize, usize)>>,
    ) {
        
        let val = node.val();
        if val.kind == MathTokenType::Operator {
            // instructions.push((val.operation.unwrap(), ))
            let mut operands = Vec::new();
            let replace_len = replace.len();
            for (_, op) in node.borrow().operands.iter_order() {
                let val = op.val();
                match val.kind {
                    MathTokenType::Constant => {
                        operands.push(val.constant.unwrap().to_f64().unwrap());
                    }
                    MathTokenType::Variable => {
                        let entry = replace.entry(val.variable.unwrap()).or_insert(vec![]);
                        entry.push((replace_len, operands.len()));

                        operands.push(0.0);
                    }
                    MathTokenType::Operator => {}
                }
            }
            instructions.push((val.operation.unwrap(), operands));
        }else {
            // instructions
            for (_pos, operand) in node.borrow().operand_iter() {
                Self::get_instructions(operand, instructions, replace);
            }
        }
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
            for (operator_index, operand_index) in indexes {
                self.instructions[*operator_index].1[*operand_index] = val;
            }
        }

        let first = &self.instructions[0];
        let mut res = first.1[0];

        for operand in first.1.iter().skip(1) {
            res = perform_op_constant(res, *operand, first.0);
        }

        for (operator, operands) in self.instructions.iter().skip(1) {
            for operand in operands {
                res = perform_op_constant(res, *operand, *operator);
            }
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
        // let mut fx = FastFunction::from(Function::from(MathTree::parse("x")).unwrap());
        let mut fx = FastFunction::from(Function::from(MathTree::parse("x^2")).unwrap());

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
        let mut fx = FastFunction::from(Function::from(MathTree::parse("x^x")).unwrap());

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
        let mut fx = FastFunction::from(Function::from(MathTree::parse("e^(x^2)")).unwrap());

        // assert_eq!(
        //     fx.evaluate_float(vec![VariableVal::new("x".to_string(), 6.0)]),
        //     Ok(Some(46656.0))
        // );

        assert_eq!(
            fx.evaluate_float(vec![VariableVal::new("x".to_string(), 1.0)]),
            Ok(Some(2.718281828459045))
        );
    }
}
