// data structure for storing operator operand children by order of type, variables then constants,
// valuable for optimizing simplification process and ordering arguments for readability

use std::{
    iter::{Chain, Map},
    ops::Index,
};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use slab::Slab;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{math_tree::TreeNodeRef, MathTokenType};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Operands {
    nodes: Slab<(TreeNodeRef, usize)>,
    // points to ^^
    variables: Slab<usize>,
    operators: Slab<usize>,
    constants: Slab<usize>,
}

impl PartialEq for Operands {
    fn eq(&self, other: &Self) -> bool {
        self.nodes.iter().eq(other.nodes.iter())
    }
}

type OperandIt<'a> = Map<slab::Iter<'a, usize>, fn((usize, &usize)) -> OperandPos>;

pub type OperandsIt<'a> = Chain<Chain<OperandIt<'a>, OperandIt<'a>>, OperandIt<'a>>;

// impl std::fmt::Debug for Operands {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.nodes.iter().map(|(_, (x, _))| x).collect_vec().fmt(f)
//     }
// }
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OperandPos(usize);

impl Index<OperandPos> for Operands {
    type Output = TreeNodeRef;

    fn index(&self, index: OperandPos) -> &Self::Output {
        &self.nodes[index.0].0
    }
}


impl Operands {
    pub fn push(&mut self, node: TreeNodeRef) {
        let node_index = self.nodes.vacant_key();
        let type_index = self.push_type_index(&node, node_index);
        self.nodes.insert((node, type_index));
    }

    fn push_type_index(&mut self, node: &TreeNodeRef, node_index: usize) -> usize {
        let val = node.val();
        let index_vec = match val.kind {
            MathTokenType::Constant => &mut self.constants,
            MathTokenType::Variable => &mut self.variables,
            MathTokenType::Operator => &mut self.operators,
        };

        index_vec.insert(node_index)
    }

    pub fn extend(&mut self, other: &Self) {
        for pos in other.iter() {
            self.push(other.nodes[pos.0].0.clone());
        }
    }

    pub fn pop_front(&mut self, orderless: bool) -> Option<TreeNodeRef> {
        // constants first
        if orderless {
            Some(self.remove(self.iter_mul().next()?))
        // by order of push
        } else {
            let index = self.iter_order().next()?.0;
            // let index = self.nodes.remove(0);
            Some(self.remove(index))
        }
    }

    pub fn remove(&mut self, pos: OperandPos) -> TreeNodeRef {
        let (tree, type_pos) = self.nodes.remove(pos.0);
        self.remove_type_index(&tree, type_pos);
        tree
    }

    fn remove_type_index(&mut self, tree: &TreeNodeRef, type_pos: usize) {
        match tree.val().kind {
            MathTokenType::Constant => self.constants.remove(type_pos),
            MathTokenType::Operator => self.operators.remove(type_pos),
            MathTokenType::Variable => self.variables.remove(type_pos),
        };
    }

    // same order, different value
    pub fn replace_val(&mut self, pos: OperandPos, with: TreeNodeRef) {
        let (tree, type_pos) = self.nodes.get(pos.0).unwrap();
        let (tree, type_pos) = (tree.clone(), *type_pos);

        self.remove_type_index(&tree, type_pos);
        let new_type_pos = self.push_type_index(&with, pos.0);

        let (tree, type_pos) = self.nodes.get_mut(pos.0).unwrap();
        *type_pos = new_type_pos;
        *tree = with;
    }

    pub fn iter<'a>(&'a self) -> OperandsIt {
        self.operators()
            .chain(self.variables())
            .chain(self.constants())
    }

    pub fn iter_mul<'a>(&'a self) -> OperandsIt {
        self.constants()
            .chain(self.variables())
            .chain(self.operators())
    }

    pub fn iter_order<'a>(
        &'a self,
    ) -> Map<
        slab::Iter<(TreeNodeRef, usize)>,
        impl FnMut((usize, &'a (TreeNodeRef, usize))) -> (OperandPos, &'a TreeNodeRef),
    > {
        self.nodes.iter().map(|(i, (n, _))| (OperandPos(i), n))
    }

    pub fn len(&self) -> usize {
        // self.constants.len() + self.operators.len() + self.variables.len()
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        // self.iter().next().is_none()
        self.len() == 0
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

    // fn ref_from_pos(&self, pos: usize) -> (OperandPos, TreeNodeRef) {
    //     (OperandPos(pos), self.nodes[pos].0.clone())
    // }

    pub fn pos_iter<'a>(&'a self, it: slab::Iter<'a, usize>) -> OperandIt<'a> {
        it.map(move |(_, pos)| {
            OperandPos(*pos)
            // self.ref_from_pos(pos)
        })
    }

    fn get_kind<'a>(&'a self, kind: MathTokenType) -> OperandIt {
        let vec = match kind {
            MathTokenType::Constant => &self.constants,
            MathTokenType::Variable => &self.variables,
            MathTokenType::Operator => &self.operators,
        };

        self.pos_iter(vec.iter())
    }

    pub fn constants<'a>(&'a self) -> OperandIt {
        self.get_kind(MathTokenType::Constant)
    }

    pub fn operators<'a>(&'a self) -> OperandIt {
        self.get_kind(MathTokenType::Operator)
    }

    pub fn variables<'a>(&'a self) -> OperandIt {
        self.get_kind(MathTokenType::Variable)
    }

    // pub fn remove_operators(&mut self) -> Vec<TreeNodeRef> {
    //     self.operators
    //         .drain(..)
    //         .map(|n| {
    //             // if let MathToken::Op(_) = n.val() {
    //             n
    //             // } else {
    //             //     unreachable!()
    //             // }
    //         })
    //         .collect_vec()
    // }
}
