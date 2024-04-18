use std::collections::HashMap;

use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::ast::{TreeNodeRef, AST};

impl AST {
    pub(crate) fn find_common_multiplier(multipliers: Vec<TreeNodeRef>) {
        // grouping terms is done only after arithmatics, so we will have at most one constant operand per multiplication
        let mut constants = multipliers
            .iter()
            .map(|n| {
                let mut constants = n.0.borrow().operands.constants();
                debug_assert!(constants.len() <= 1);
                constants.pop().unwrap()
            })
            .collect_vec();

        let constant_multiplier = Self::find_common_constant(constants);

        let variables = multipliers
            .iter()
            .map(|n| n.0.borrow().operands.variables())
            .flatten()
            .collect_vec();

        let common_variable = Self::find_common_variable(variables, multipliers.len() );
    }

    fn find_common_variable(variables: Vec<String>, multiplier_len: usize) -> Vec<String> {
        let mut seen_variables = HashMap::new();
        for var in variables {
            match seen_variables.get_mut(&var) {
                Some(k) => *k += 1,
                None => {
                    seen_variables.insert(var, 1);
                }
            }
        }

        let mut common = Vec::new();
        for (var, seen) in seen_variables {
            if seen == multiplier_len{
                println!("COMMON VAR: {}", var);
                common.push(var);
            }
        }

        common
    }

    fn find_common_constant(mut constants: Vec<Decimal>) -> Option<Decimal> {
        constants.sort();

        // all the numbers need to be divisiable by the common term

        if constants.len() <= 1 {
            return None;
        }

        // 1) first check smallest number
        let mut check = constants.first().unwrap().clone();
        let mut divider = dec!(1);

        'outer: loop {
            check = check / divider;
            divider += dec!(1);

            if check == Decimal::ONE {
                break;
            }

            for c in &constants {
                if c.checked_rem(check.clone()) != Some(Decimal::ZERO) {
                    // break;
                    continue 'outer;
                }
            }

            return Some(check);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::ast::AST;

    #[test]
    fn common_multiplier_constant() {
        assert_eq!(AST::find_common_constant(vec![dec!(2)]), None);

        assert_eq!(AST::find_common_constant(vec![dec!(2), dec!(3)]), None);

        assert_eq!(
            AST::find_common_constant(vec![dec!(2), dec!(4)]),
            Some(dec!(2))
        );

        assert_eq!(
            AST::find_common_constant(vec![dec!(4), dec!(6)]),
            Some(dec!(2))
        );

        assert_eq!(
            AST::find_common_constant(vec![dec!(4), dec!(8), dec!(12)]),
            Some(dec!(4))
        );

        assert_eq!(
            AST::find_common_constant(vec![dec!(4), dec!(8), dec!(0.5)]),
            Some(dec!(0.5))
        );

        // assert_eq!(
        //     AST::find_common_multiplier_constant(vec![dec!(4), dec!(8), dec!(0.3213123)]),
        //     Some(dec!(0.5))
        // );
    }
}
