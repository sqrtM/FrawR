use serde::{Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tile::tile::Point;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Entity {
    pub location: Point,
    pub char: char,
    pub id: u32
}

enum EntityType {
    Player,
    Enemy
}

impl EntityType {
    fn get(&self, point: Point, id: u32) -> Entity {
        match self {
            EntityType::Player => Entity { location: point, char: '@', id },
            EntityType::Enemy => Entity { location: point, char: 'W', id },
        }
    }
}
