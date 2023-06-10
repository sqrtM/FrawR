mod entity;
mod tile;
mod world;

use std::collections::BTreeMap;

use entity::entity::Entity;
use serde::Serialize;
use struct_iterable::Iterable;
use tile::tile::{Tile, Point};
use wasm_bindgen::prelude::*;

/// everything that lives in the world and organically changes.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Iterable)]
pub struct Creatures {
    entities: Vec<Entity>,
    player: Entity,
}

/// everything
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct World {
    tiles: BTreeMap<Point, Tile>,
    creatures: Creatures,
    width: i32,
    height: i32,
}

#[wasm_bindgen(start)]
pub fn setup() {
    wasm_logger::init(wasm_logger::Config::default());
}
