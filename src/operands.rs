// data structure for storing operator operand children by order of type, variables then constants,
// valuable for optimizing simplification process and ordering arguments for readability

use std::{
    collections::BTreeMap,
    default,
    iter::{self, Chain, Map},
    ops::Index,
    vec,
};

use itertools::Itertools;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use slab::Slab;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{math_tree::TreeNodeRef, MathToken};

// 3 + x^2 -> x^2 + 3
// 3 / x^2 -> x^2 / 3

// left, right

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Serialize, Deserialize)]
enum Operands {
    //
    Orderless(BTreeMap<Decimal, TreeNodeRef>),
    Ordered(Vec<TreeNodeRef>),
}

impl PartialEq for Operands {

    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

type OperandIt<'a> = Map<slab::Iter<'a, usize>, fn((usize, &usize)) -> OperandPos>;

pub type OperandsIt<'a> = Chain<Chain<OperandIt<'a>, OperandIt<'a>>, OperandIt<'a>>;

// impl std::fmt::Debug for Operands {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.nodes.iter().map(|(_, (x, _))| x).collect_vec().fmt(f)
//     }
// }


//   +
// 1, *
//   2 1

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
        // let node_index = self.nodes.vacant_key();

        // let type_index = self.push_type_index(&node, node_index);
        match self {
            Operands::Orderless(map) => {
                map.insert(node.ordering_exponent(), node);
            }
            Operands::Ordered(vec) => {
                vec.push(node);
            }
        }
    }

    pub fn extend(&mut self, other: &Self) {
        for val in other.iter() {
            self.push(val.clone());
        }
    }

    pub fn remove(&mut self, pos: OperandPos) -> TreeNodeRef {
        let (tree, type_pos) = self.nodes.remove(pos.0);
        self.remove_type_index(&tree, type_pos);
        tree
    }
    
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &TreeNodeRef> + 'a> {
        match self {
            Operands::Orderless(map) => Box::new(map.iter().map(|(_, v)| v)),
            Operands::Ordered(vec) => Box::new(vec.iter()),
        }
    }

    // pub fn iter_mul<'a>(&'a self) -> OperandsIt {
    //     self.constants()
    //         .chain(self.variables())
    //         .chain(self.operators())
    // }

    pub fn len(&self) -> usize {
        match self {
            Operands::Orderless(map) => map.len(),
            Operands::Ordered(vec) => vec.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pos_iter<'a>(&'a self, it: slab::Iter<'a, usize>) -> OperandIt<'a> {
        it.map(move |(_, pos)| {
            OperandPos(*pos)
            // self.ref_from_pos(pos)
        })
    }

    // fn get_kind<'a>(&'a self, kind: MathTokenType) -> OperandIt {
    //     let vec = match kind {
    //         MathTokenType::Constant => &self.constants,
    //         MathTokenType::Variable => &self.variables,
    //         MathTokenType::Operator => &self.operators,
    //     };

    //     self.pos_iter(vec.iter())
    // }

    // pub fn constants<'a>(&'a self) -> OperandIt {
    //     self.get_kind(MathTokenType::Constant)
    // }

    // pub fn operators<'a>(&'a self) -> OperandIt {
    //     self.get_kind(MathTokenType::Operator)
    // }

    // pub fn variables<'a>(&'a self) -> OperandIt {
    //     self.get_kind(MathTokenType::Variable)
    // }
}
