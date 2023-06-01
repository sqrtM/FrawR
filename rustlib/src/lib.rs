mod tile;

use log;
use noise::{self, NoiseFn};
use tile::tile::{Tile};
use wasm_bindgen::prelude::*;

use crate::tile::tile::{TileType, TileChar};

#[wasm_bindgen]
pub struct World {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

#[wasm_bindgen]
impl World {
    pub fn build_map(height: i32, width: i32) -> World {
        log::warn!("start");
        let simp = noise::OpenSimplex::new(4673837);
        
        let tiles = (0..width * height)
            .map(|i| {
                let row = (i % width) as f64;
                let col = (i as f64 / height as f64).floor();
                let val = simp.get([row, col]);
                let name = match val {
                    -1. ..=-0.4 => TileType::Wall,
                    -0.4..=0.0 => TileType::Slope,
                    0.0..=0.3 => TileType::Floor,
                    _ => TileType::Floor
                };
                let char = match val {
                    -1. ..=-0.4 => '#',
                    -0.4..=0.0 => '/',
                    0.0..=0.3 => '.',
                    _ => '.'
                };
                Tile {
                    val,
                    name,
                    char
                }
            })
            .collect();
        
        World {
            width,
            height,
            tiles
        }
    }

    pub fn render(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.tiles).unwrap()
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    wasm_logger::init(wasm_logger::Config::default());
}
