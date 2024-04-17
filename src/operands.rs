// data structure for storing operator operand children by order of type, variables then constants,
// valuable for optimizing simplification process and ordering arguments for readability

use itertools::Itertools;
use rust_decimal::Decimal;

use crate::{ast::TreeNodeRef, MathToken, OperationToken};

#[derive(Clone)]
pub struct Operands {
    childs: Vec<TreeNodeRef>,
    // represent where each section of the vector starts
    operators: usize,
    constants: usize,
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

impl PartialEq for Operands {
    fn eq(&self, other: &Self) -> bool {
        self.childs == other.childs
    }
}
impl std::fmt::Debug for Operands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.childs.fmt(f)
    }
}

// pub type Operands = BTreeSet<TreeNodeRef>;
impl Operands {
    pub fn new() -> Self {
        Self {
            childs: Vec::new(),
            operators: 0,
            constants: 0,
        }
    }

    pub fn add(&mut self, node: TreeNodeRef) {
        match node.val() {
            MathToken::Constant(_) => {
                self.childs.insert(self.operators, node);
                self.operators += 1;
            }
            MathToken::Variable(_) => {
                self.childs.insert(self.constants, node);
                self.constants += 1;
                self.operators += 1;
            }
            MathToken::Op(_) => {
                self.childs.push(node);
            }
        }
    }

    pub fn extend(&mut self, other: &Self) {
        for node in &other.childs {
            self.add(node.clone());
        }
    }

    pub fn iter(&self) -> core::slice::Iter<TreeNodeRef> {
        self.childs.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.childs.is_empty()
    }
    pub fn remove_constants(&mut self) -> Vec<Decimal> {
        // there cant be constants when removing operators
        assert!(self.childs.len() <= self.operators);
        // update index
        self.operators = self.constants;

        self.childs
            .split_off(self.constants)
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

    pub fn remove_operators(&mut self) -> Vec<TreeNodeRef> {
        self.childs
            .split_off(self.operators)
            .into_iter()
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
