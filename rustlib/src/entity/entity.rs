use serde::{Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tile::tile::Point;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Entity {
    pub location: Point,
    pub char: char,
    pub moves: bool,
    pub id: u32
}

pub enum EntityType {
    Player,
    Enemy
}

impl EntityType {
    pub fn get(&self, point: Point, id: u32) -> Entity {
        match self {
            EntityType::Player => Entity { location: point, char: '@', id, moves: true },
            EntityType::Enemy => Entity { location: point, char: 'W', id, moves: true },
        }
    }
}
