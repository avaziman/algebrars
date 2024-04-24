// data structure for storing operator operand children by order of type, variables then constants,
// valuable for optimizing simplification process and ordering arguments for readability

use std::{
    iter::{Chain, Enumerate, Map},
    ops::Index,
    slice::Iter,
};

use itertools::Itertools;
use rust_decimal::Decimal;

use crate::{math_tree::TreeNodeRef, MathToken, OperationToken};

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

type OperandIt<'a> = Map<
    Enumerate<Iter<'a, TreeNodeRef>>,
    fn((usize, &'a TreeNodeRef)) -> (OperandPos, &TreeNodeRef),
>;
// impl std::fmt::Debug for Operands {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.childs.fmt(f)
//     }
// }
pub enum OperandPos {
    Constants(usize),
    Operators(usize),
    Variables(usize),
}

impl Index<OperandPos> for Operands {
    type Output = TreeNodeRef;

    fn index(&self, index: OperandPos) -> &Self::Output {
        match index {
            OperandPos::Constants(p) => &self.constants[p],
            OperandPos::Operators(p) => &self.operators[p],
            OperandPos::Variables(p) => &self.variables[p],
        }
    }
}

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
        for (_, node) in other.iter() {
            self.add(node.clone());
        }
    }

    pub fn pop_front(&mut self) -> Option<TreeNodeRef> {
        Some(self.remove(self.iter().next()?.0))
    }

    pub fn remove(&mut self, pos: OperandPos) -> TreeNodeRef {
        match pos {
            OperandPos::Constants(p) => self.constants.remove(p),
            OperandPos::Operators(p) => self.operators.remove(p),
            OperandPos::Variables(p) => self.variables.remove(p),
        }
    }

    // pub fn replace(&mut self, index: usize, new: TreeNodeRef) {
    //     if index < self.operators.len() {
    //         self.operators.remove(index);
    //     }else if index < self.operators.len() + self.variables.len() {
    //         self.variables.remove(index - self.operators.len());
    //     }else {
    //         self.constants.remove(index - self.operators.len() - self.variables.len());
    //     }
    // }   }

    // pub fn variables(&self) -> Vec<String> {
    //     self.variables
    //         .iter()
    //         .map(|n| {
    //             if let MathToken::Variable(d) = n.val() {
    //                 d
    //             } else {
    //                 unreachable!()
    //             }
    //         })
    //         .collect_vec()
    // }
    // }

    pub fn iter(&self) -> Chain<Chain<OperandIt, OperandIt>, OperandIt> {
        self.operators()
            .chain(self.variables())
            .chain(self.constants())
    }

    pub fn iter_mul(&self) -> Chain<Chain<OperandIt, OperandIt>, OperandIt> {
        self.constants()
            .chain(self.variables())
            .chain(self.operators())
    }

    pub fn len(&self) -> usize {
        self.constants.len() + self.operators.len() + self.variables.len()
    }

    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    // pub fn variables(&self) -> Vec<String> {
    //     self.variables
    //         .iter()
    //         .map(|n| {
    //             if let MathToken::Variable(d) = n.val() {
    //                 d
    //             } else {
    //                 unreachable!()
    //             }
    //         })
    //         .collect_vec()
    // }

    pub fn constants(&self) -> OperandIt {
        self.constants
            .iter()
            .enumerate()
            .map(|(i, c)| (OperandPos::Constants(i), c))
    }

    pub fn operators(&self) -> OperandIt {
        self.operators
            .iter()
            .enumerate()
            .map(|(i, c)| (OperandPos::Operators(i), c))
    }

    pub fn variables(&self) -> OperandIt {
        self.variables
            .iter()
            .enumerate()
            .map(|(i, c)| (OperandPos::Variables(i), c))
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
}
