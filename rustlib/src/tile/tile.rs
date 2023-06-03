use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Tile {
    pub val: i16,
    pub name: TileType,
    pub location: Point,
    pub char: char,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Ord, PartialOrd, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(u8)]
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum TileType {
    Floor = 0,
    Slope = 1,
    Wall = 2,
}
