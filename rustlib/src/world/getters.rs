use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{World, tile::tile::{Tile, Point}};

#[wasm_bindgen]
impl World {
    pub fn get_entities(&mut self) -> JsValue {
        Self::sort_entities(self);
        serde_wasm_bindgen::to_value(&self.creatures.entities).unwrap()
    }

    pub fn get_tiles(&self) -> JsValue {
        let i: Vec<(&Point, &Tile)> = Vec::from_iter(&self.tiles);
        serde_wasm_bindgen::to_value(&i).unwrap()
    }

    pub fn get_player(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.creatures.player).unwrap()
    }

    pub fn get_all_creatures(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.creatures).unwrap()
    }
}