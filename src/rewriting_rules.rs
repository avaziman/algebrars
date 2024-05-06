// use crate::{
//     bounds::Bound,
//     math_tree::{MathTree, ParseError, TreeNodeRef},
// };

// pub struct Rule {
//     pattern: MathTree,
//     becomes: MathTree,
//     // bound: Bound,
// }

// // inspired by mathjs
// pub fn build_rules() -> Result<Vec<Rule>, ParseError> {
//     Ok(vec![Rule::new("x/x", "1")?])
// }
// // x^n / x^m = x^n-m (x^m != 0)
// // pythagoras theorem
// // 3 4 5, (5, 12, 13) Pythagorean triple

// impl Rule {
//     pub fn new(pattern: &tr, becomes: &str) -> Result<Rule, ParseError> {
//         Ok(Self {
//             pattern: MathTree::parse(pattern)?,
//             becomes: MathTree::parse(becomes)?,
//         })
//     }
// }
