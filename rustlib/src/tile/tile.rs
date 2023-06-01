use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Tile {
    pub val: f64,
    pub name: TileType,
    pub char: char
}

#[repr(u8)]
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum TileType {
    Floor = 0,
    Slope = 1,
    Wall = 2
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum TileChar {
    Floor = b'.',
    Slope = b'/',
    Wall = b'#',
}
