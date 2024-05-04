use serde::{Deserialize, Serialize};

use crate::function::function::Function;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NumberType {
    Natural,    // N positive whole numbers
    Integer,    // Z whole numbers
    Rational,   // Q fractions
    Real,       // R Rational + Irrational
    Irrational, // R / Q real numbers that are not rational (roots constants etc)
    Imaginary,  //
    Complex,    //
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoundType {
    // belongs to set of numbers
    BelongsToNumberType,
    // belongs to function
    BelongsToFunction,
    // Between,
    // smaller bigger
    Ordering,
}

// TODO: clone not efficient
#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen::prelude::wasm_bindgen
)]
pub struct Bound {
    pub kind: BoundType,
    pub function_id: Option<usize>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl Bound {
    pub fn belongs_to_fn(function_id: usize) -> Self {
        Self {
            function_id: Some(function_id),
            kind: BoundType::BelongsToFunction,
        }
    }

    // pub fn to_string(&self) -> String {
    //     match self.kind {
    //         BoundType::BelongsToNumberType => todo!(),
    //         BoundType::BelongsToFunction => format!("∈ "),
    //         BoundType::Ordering => todo!(),
    //     }
    // }
}
