// data structure for storing operator operand children by order of type, variables then constants,
// valuable for optimizing simplification process and ordering arguments for readability

use itertools::Itertools;
use rust_decimal::Decimal;

use crate::{ast::TreeNodeRef, MathToken, OperationToken};

#[derive(Clone, Debug, PartialEq)]
pub struct Operands {
    variables: Vec<TreeNodeRef>,
    // represent where each section of the vector starts
    operators: Vec<TreeNodeRef>,
    constants: Vec<TreeNodeRef>,
}

impl FromIterator<TreeNodeRef> for Operands {
    fn from_iter<T: IntoIterator<Item = TreeNodeRef>>(iter: T) -> Self {
        let mut operands = Operands::new();
        for i in iter {
            operands.add(i)
        }

        operands
    }
}

// impl std::fmt::Debug for Operands {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.childs.fmt(f)
//     }
// }

// pub type Operands = BTreeSet<TreeNodeRef>;
impl Operands {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            operators: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn add(&mut self, node: TreeNodeRef) {
        match node.val() {
            MathToken::Constant(_) => {
                self.constants.push(node);
            }
            MathToken::Variable(_) => {
                self.variables.push(node);
            }
            MathToken::Op(_) => {
                self.operators.push(node);
            }
        }
    }

    pub fn extend(&mut self, other: &Self) {
        for node in other.iter() {
            self.add(node.clone());
        }
    }

    pub fn iter(
        &self,
    ) -> std::iter::Chain<
        std::iter::Chain<std::slice::Iter<TreeNodeRef>, std::slice::Iter<TreeNodeRef>>,
        std::slice::Iter<TreeNodeRef>,
    > {
        self.operators
            .iter()
            .chain(self.variables.iter())
            .chain(self.constants.iter())
    }

    pub fn iter_mul(
        &self,
    ) -> std::iter::Chain<
        std::iter::Chain<std::slice::Iter<TreeNodeRef>, std::slice::Iter<TreeNodeRef>>,
        std::slice::Iter<TreeNodeRef>,
    > {
        self.constants
            .iter()
            .chain(self.variables.iter())
            .chain(self.operators.iter())
    }

    pub fn len(&self) -> usize {
        self.constants.len() + self.operators.len() + self.variables.len()  
    }    

    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    pub fn variables(&self) -> Vec<String> {
        self.variables
            .iter()
            .map(|n| {
                if let MathToken::Variable(d) = n.val() {
                    d
                } else {
                    unreachable!()
                }
            })
            .collect_vec()
    }


    pub fn constants(&self) -> Vec<Decimal> {
        self.constants
            .iter()
            .map(|n| {
                if let MathToken::Constant(d) = n.val() {
                    d
                } else {
                    unreachable!()
                }
            })
            .collect_vec()
    }

    pub fn remove_constants(&mut self) -> Vec<Decimal> {
        // there cant be constants when removing operators
        // assert!(self.constants <= self.operators);
        // update index
        self.constants
            .drain(..)
            .map(|n| {
                if let MathToken::Constant(d) = n.val() {
                    d
                } else {
                    unreachable!()
                }
            })
            .collect_vec()
    }

    pub fn remove_operators(&mut self) -> Vec<TreeNodeRef> {
        self.operators
            .drain(..)
            .map(|n| {
                if let MathToken::Op(_) = n.val() {
                    n
                } else {
                    unreachable!()
                }
            })
            .collect_vec()
    }

    // fn power_of(node: &TreeNodeRef) -> i8 {
    //     // first operators because we don't know their "power"
    //     match node.val() {
    //         MathToken::Constant(_) => 0,
    //         MathToken::Variable(_) => 1,
    //         MathToken::Op(_) => 2,
    //     }
    // }
}
