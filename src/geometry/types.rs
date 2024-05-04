use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct Point {
    pub name: String,
    pub cords: Pos,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Segment {
    a: Point,
    b: Point,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn dist(a: Pos, b: Pos) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Pos {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn copy(&self) -> Self {
        self.clone()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Point {
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
 pub fn new(name: String, cords: Pos) -> Self {
    Self {
        name,
        cords,
    }
 }
}