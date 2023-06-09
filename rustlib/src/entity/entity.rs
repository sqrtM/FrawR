use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tile::tile::Point;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Entity {
    pub location: Point,
    pub char: char,
    pub npc: bool,
    pub id: u32,
    pub health: u16,
    pub hunger: u8,
    pub status_effects: StatusEffects
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct StatusEffects {
    pub hungry: bool,
}

pub enum EntityType {
    Player,
    Enemy
}

impl EntityType {
    pub fn get(&self, point: Point, id: u32) -> Entity {
        match self {
            EntityType::Player => Entity { location: point, char: '@', id, npc: false, health: 100, hunger: 10, status_effects: StatusEffects { hungry: false } },
            EntityType::Enemy => Entity { location: point, char: 'W', id, npc: true, health: 100, hunger: 10, status_effects: StatusEffects { hungry: false, }},
        }
    }
}
