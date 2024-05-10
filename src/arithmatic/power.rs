use itertools::Itertools;

use crate::{math_tree::TreeNodeRef, MathTokenType, OperationToken};

pub fn get_node_as_power(node: TreeNodeRef) -> (TreeNodeRef, TreeNodeRef) {
    let val = node.val();
    if val.kind == MathTokenType::Operator {
        let op = val.operation.unwrap();

        if op == OperationToken::Pow {
            // x ^ n => (x, n)
            return node
                .borrow()
                .calculate_iter()
                .map(|x| x.1.clone())
                .collect_tuple()
                .unwrap();
        } else if op == OperationToken::Root {
            let operands: (TreeNodeRef, TreeNodeRef) = node
                .borrow()
                .calculate_iter()
                .map(|x| x.1.clone())
                .collect_tuple()
                .unwrap();

            return (operands.0, TreeNodeRef::one().divide(operands.1));
        }
    }

    (node, TreeNodeRef::one())
}
