// data structure for storing operator operand children by order of type, variables then constants,
// valuable for optimizing simplification process and ordering arguments for readability

use std::{
    iter::{Chain, Enumerate, Map},
    ops::Index,
    slice::Iter,
};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{math_tree::TreeNodeRef, MathTokenType};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operands {
    variables: Vec<TreeNodeRef>,
    operators: Vec<TreeNodeRef>,
    constants: Vec<TreeNodeRef>,
    insert_order: Option<Vec<OperandPos>>,
}
// TODO: think about ordered operands

type OperandIt<'a> = Map<
    Enumerate<Iter<'a, TreeNodeRef>>,
    fn((usize, &'a TreeNodeRef)) -> (OperandPos, &TreeNodeRef),
>;
pub type OperandsIt<'a> = Chain<Chain<OperandIt<'a>, OperandIt<'a>>, OperandIt<'a>>;

// impl std::fmt::Debug for Operands {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.childs.fmt(f)
//     }
// }
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperandPos(MathTokenType, usize);

impl Index<OperandPos> for Operands {
    type Output = TreeNodeRef;

    fn index(&self, index: OperandPos) -> &Self::Output {
        match index.0 {
            MathTokenType::Constant => &self.constants[index.1],
            MathTokenType::Operator => &self.operators[index.1],
            MathTokenType::Variable => &self.variables[index.1],
        }
    }
}

// pub type Operands = BTreeSet<TreeNodeRef>;
impl Operands {
    pub fn new(orderless: bool) -> Self {
        Self {
            constants: Vec::new(),
            operators: Vec::new(),
            variables: Vec::new(),
            insert_order: if orderless { None } else { Some(Vec::new()) },
        }
    }

    pub fn add(&mut self, node: TreeNodeRef) {
        let vec = match node.val().kind {
            MathTokenType::Constant => &mut self.constants,
            MathTokenType::Variable => &mut self.variables,
            MathTokenType::Operator => &mut self.operators,
        };

        if let Some(order) = &mut self.insert_order {
            order.push(OperandPos(node.val().kind, vec.len()));
        }
        vec.push(node);
    }

    pub fn extend(&mut self, other: &Self) {
        for (_, node) in other.iter() {
            self.add(node.clone());
        }
    }

    pub fn pop_front(&mut self) -> Option<TreeNodeRef> {
        // constants first
        Some(self.remove(self.iter_mul().next()?.0))
    }

    pub fn remove(&mut self, pos: OperandPos) -> TreeNodeRef {
        match pos.0 {
            MathTokenType::Constant => self.constants.remove(pos.1),
            MathTokenType::Operator => self.operators.remove(pos.1),
            MathTokenType::Variable => self.variables.remove(pos.1),
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

    pub fn iter(&self) -> OperandsIt {
        self.operators()
            .chain(self.variables())
            .chain(self.constants())
    }

    pub fn iter_mul(&self) -> OperandsIt {
        self.constants()
            .chain(self.variables())
            .chain(self.operators())
    }

    pub fn iter_order<'a>(
        &'a self,
    ) -> Option<Map<std::ops::Range<usize>, impl FnMut(usize) -> &'a TreeNodeRef>> {
        let order = self.insert_order.as_ref()?;

        Some((0..self.len()).map(|i| &self[order[i].clone()]))
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
            .map(|(i, c)| (OperandPos(MathTokenType::Constant, i), c))
    }

    pub fn operators(&self) -> OperandIt {
        self.operators
            .iter()
            .enumerate()
            .map(|(i, c)| (OperandPos(MathTokenType::Operator, i), c))
    }

    pub fn variables(&self) -> OperandIt {
        self.variables
            .iter()
            .enumerate()
            .map(|(i, c)| (OperandPos(MathTokenType::Variable, i), c))
    }

    pub fn remove_operators(&mut self) -> Vec<TreeNodeRef> {
        self.operators
            .drain(..)
            .map(|n| {
                // if let MathToken::Op(_) = n.val() {
                n
                // } else {
                //     unreachable!()
                // }
            })
            .collect_vec()
    }
}
