use std::collections::{BTreeMap, BTreeSet, HashMap};

use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    arithmatic::power,
    math_tree::{MathTree, TreeNode, TreeNodeRef},
    MathToken, OperationToken,
};

impl MathTree {
    pub fn factorize_node(node: TreeNodeRef) -> Option<TreeNodeRef> {
        let Some(factor) = Self::find_common_factor(node.clone()) else {
            return None;
        };

        let childs = node
            .borrow()
            .calculate_iter()
            .map(|(_, node)| node.divide(factor.clone()))
            .collect_vec();

        let factored = TreeNodeRef::new_vals(MathToken::Operation(OperationToken::Add), childs);

        Some(factored.multiply(factor))
    }

    // greatest common factor
    pub fn find_common_factor(node: TreeNodeRef) -> Option<TreeNodeRef> {
        // grouping terms is done only after arithmatics,
        // so we will have at most one constant operand per multiplication

        // if node.val().operation != Some(OperationToken::Multiply) {
        //     return None;
        // }

        let mut multipliers =  node.borrow().calculate_iter().map(|x| x.1.clone()).collect_vec();

        Self::find_common_variable(multipliers)
    }

    fn get_constant_multiplier(node: TreeNodeRef) -> (Decimal, TreeNodeRef) {
        if node.val() == MathToken::Operation(OperationToken::Multiply) {
            let borrow = node.borrow();
            let mut iter = borrow.calculate_iter().map(|x| x.1.clone());

            let constant = iter.next().unwrap().val().constant.unwrap();

            // remaining variables
            let mut childs = iter.collect_vec();
            let multiplier = if childs.len() == 1 {
                childs.pop().unwrap()
            } else {
                TreeNodeRef::new_vals(MathToken::Operation(OperationToken::Multiply), childs)
            };

            (constant, multiplier)
        } else {
            (Decimal::ONE, node)
        }
    }
    // (x, x^2) common: x
    // (sqrt(x), x): common sqrt(x) = x^(1/2)
    // (x^x, x^(x/2)): common x^(x/2)
    // (x^x, x^(x/2), x^(x/3))
    //   => common x^(find_common(x, x/2, x/3))
    //   => common x^(x*find_common(1, 1/2, 1/3)) =
    // ((x+3)^2 + (x+3)^3): common (x+3)^2
    // TODO: option to only factor N (one) item at a time
    pub(crate) fn find_common_variable(variables: Vec<TreeNodeRef>) -> Option<TreeNodeRef> {
        // let mut sorted: HashMap<TreeNodeRef, BTreeMap<TreeNodeRef, TreeNodeRef>> = HashMap::new();
        let mut constants = BTreeSet::new();
        let variables = variables
            .into_iter()
            .map(|x| {
                let (constant_multiplier, node) = Self::get_constant_multiplier(x);
                constants.insert(constant_multiplier);

                power::get_node_as_power(node)
            })
            .collect_vec();

        let mut bases = variables.iter().map(|(base, _)| base);
        let base = bases
            .clone()
            .min_by_key(|a| a.borrow().operands().len())
            .unwrap()
            .clone();

        let is_multiple_of = |node: &TreeNodeRef, of: &TreeNodeRef| {
            let val = node.val();
            node == of
                || match val {
                    MathToken::Constant(c) => {
                        c * dec!(-1) == of.val().constant.unwrap()
                    }
                    MathToken::Variable => false,
                    MathToken::Operator => {
                        val == MathToken::Operation(OperationToken::Multiply)
                            && node.borrow().calculate_iter().any(|(_, n)| n == of)
                    }
                }
        };

        let constant_multiplier =
            Self::find_common_factor_constant(constants).map(|d| TreeNodeRef::constant(d));

        // common bases
        // a base is common if all operands have it
        if bases.all(|n| is_multiple_of(&n, &base)) {
            if let Some(constant) = constant_multiplier {
                return Some(constant.multiply(base.clone()));
            }

            return Some(base.clone());
        }
        // for (base, power) in variables {}

        constant_multiplier
    }

    fn find_common_factor_constant(mut constants: BTreeSet<Decimal>) -> Option<Decimal> {
        // sorted for efficiency

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

    fn find_common_denominator_constant(mut constants: BTreeSet<Decimal>) -> Decimal {
        let biggest = constants.pop_last().unwrap();
        let mut counter = biggest;
        loop {
            if constants
                .iter()
                .all(|c| c.checked_rem(biggest) == Some(Decimal::ZERO))
            {
                return counter;
            }

            counter += biggest;
        }
    }
}
#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::{
        math_tree::{MathTree, TreeNodeRef},
        simplify::simplify,
        MathToken,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn common_multiplier_constant() {
        assert_eq!(
            MathTree::find_common_factor_constant([dec!(2)].into()),
            None
        );

        assert_eq!(
            MathTree::find_common_factor_constant([dec!(2), dec!(3)].into()),
            None
        );

        assert_eq!(
            MathTree::find_common_factor_constant([dec!(2), dec!(4)].into()),
            Some(dec!(2))
        );

        assert_eq!(
            MathTree::find_common_factor_constant([dec!(2), dec!(-2)].into()),
            Some(dec!(-2))
        );

        assert_eq!(
            MathTree::find_common_factor_constant([dec!(4), dec!(6)].into()),
            Some(dec!(2))
        );

        assert_eq!(
            MathTree::find_common_factor_constant([dec!(4), dec!(8), dec!(12)].into()),
            Some(dec!(4))
        );

        assert_eq!(
            MathTree::find_common_factor_constant([dec!(4), dec!(8), dec!(0.5)].into()),
            Some(dec!(0.5))
        );
    }

    #[test]
    fn find_common_factor() {
        assert_eq!(
            MathTree::find_common_variable(vec![
                TreeNodeRef::parse("2*x"),
                TreeNodeRef::parse("4*x"),
            ]),
            Some(TreeNodeRef::parse("2*x")),
        );

        assert_eq!(
            MathTree::find_common_variable(vec![
                TreeNodeRef::parse("-2*x"),
                TreeNodeRef::parse("2*x"),
            ]),
            Some(TreeNodeRef::parse("2*x")),
        );
    }

    #[test]
    fn factorize_x() {
        // simplify::tests::simplify_test_latex("x + 2 * x", "3x");
        simplify::tests::simplify_test_latex("(-2*x)+(2*x)", "0");
    }
}
