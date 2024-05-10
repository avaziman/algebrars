use std::collections::{BTreeMap, BTreeSet, HashMap};

use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    arithmatic::power,
    math_tree::{MathTree, TreeNode, TreeNodeRef},
    MathToken, MathTokenType, OperationToken,
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

        let factored = TreeNodeRef::new_vals(MathToken::operator(OperationToken::Add), childs);

        Some(factored.multiply(factor))
    }

    // greatest common factor
    pub fn find_common_factor(node: TreeNodeRef) -> Option<TreeNodeRef> {
        // grouping terms is done only after arithmatics,
        // so we will have at most one constant operand per multiplication

        // if node.val().operation != Some(OperationToken::Multiply) {
        //     return None;
        // }

        let mut constants = BTreeSet::new();

        let mut multipliers = Vec::with_capacity(node.borrow().operands().len());

        for (_, operand) in node.borrow().calculate_iter() {
            let val = operand.val();
            multipliers.push(operand.clone());
            match operand.val().kind {
                MathTokenType::Constant => {
                    constants.insert(val.constant.unwrap());
                }
                MathTokenType::Variable | MathTokenType::Operator => {
                    // multipliers.push(operand.clone())
                }
            }
        }

        // let mut common =
        //     Self::find_common_factor_constant(constants).map(|d| TreeNodeRef::constant(d));
        let mut common: Option<TreeNodeRef> = None;

        if let Some(common_var) = Self::find_common_variable(multipliers) {
            if let Some(cmn) = common {
                common = Some(cmn.multiply(common_var));
            } else {
                common = Some(common_var);
            }
        }

        common
    }

    // (x, x^2) common: x
    // (sqrt(x), x): common sqrt(x) = x^(1/2)
    // (x^x, x^(x/2)): common x^(x/2)
    // (x^x, x^(x/2), x^(x/3))
    //   => common x^(find_common(x, x/2, x/3))
    //   => common x^(x*find_common(1, 1/2, 1/3)) =
    // ((x+3)^2 + (x+3)^3): common (x+3)^2
    // TODO: option to only factor N (one) item at a time
    fn find_common_variable(variables: Vec<TreeNodeRef>) -> Option<TreeNodeRef> {
        // let mut sorted: HashMap<TreeNodeRef, BTreeMap<TreeNodeRef, TreeNodeRef>> = HashMap::new();
        let variables = variables.into_iter().map(|x| power::get_node_as_power(x));
        let mut bases = variables.map(|(base, _)| base);
        let base = bases
            .clone()
            .min_by_key(|a| a.borrow().operands().len())
            .unwrap()
            .clone();

        let is_multiple_of = |node: &TreeNodeRef, of: &TreeNodeRef| {
            let val = node.val();
            node == of
                || match val.kind {
                    MathTokenType::Constant => {
                        val.constant.unwrap() * dec!(-1) == of.val().constant.unwrap()
                    }
                    MathTokenType::Variable => false,
                    MathTokenType::Operator => {
                        val == MathToken::operator(OperationToken::Multiply)
                            && node.borrow().calculate_iter().any(|(_, n)| n == of)
                    }
                }
        };
        // common bases
        // a base is common if all operands have it
        if bases.all(|n| is_multiple_of(&n, &base)) {
            return Some(base.clone());
        }
        // for (base, power) in variables {}

        None
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

    use crate::{math_tree::MathTree, simplify::simplify};

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
    fn factorize_x() {
        simplify::tests::simplify_test_latex("(-2*x)+(-1)+(2*x)", "-1");
    }
}
