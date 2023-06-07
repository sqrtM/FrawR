use serde::Serialize;
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
    Mountain = 3,
    Shore = 4,
    Water = 5,
    DeepWater = 6,
    Default = 7,
}

impl TileType {
    pub fn get_char(&self) -> char {
        match self {
            TileType::Floor => '.',
            TileType::Slope => '/',
            TileType::Wall => '#',
            TileType::Mountain => '^',
            TileType::Shore => '%',
            TileType::Water => '~',
            TileType::DeepWater => '*',
            TileType::Default => 'D',
        }
    }
}
