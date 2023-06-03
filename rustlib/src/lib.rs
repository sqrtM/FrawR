mod entity;
mod tile;

use entity::entity::Entity;
use noise::{self, NoiseFn};
use serde::Serialize;
use tile::tile::{Point, Tile};
use wasm_bindgen::prelude::*;

use crate::tile::tile::TileType;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct World {
    tiles: Vec<Tile>,
    entities: Vec<Entity>,
}

#[wasm_bindgen]
impl World {
    pub fn build_map(&mut self, height: i32, width: i32, rand: u32) {
        let simp = noise::OpenSimplex::new(rand);

        self.tiles = (0..width * height)
            .map(|i| {
                let row = (i % width) as f64;
                let col = (i as f64 / height as f64).floor();
                let val = (simp.get([row, col]) * 100.) as i16;
                let name = match val {
                    -30..=-10 => TileType::Wall,
                    -9..=15 => TileType::Slope,
                    16..=40 => TileType::Floor,
                    _ => TileType::Floor,
                };
                let char = match val {
                    -500..=-30 => '#',
                    -9..=15 => '/',
                    16..=40 => '.',
                    _ => '.',
                };
                Tile {
                    val,
                    name,
                    char,
                    location: Point {
                        x: i % width,
                        y: i / width,
                    },
                }
            })
            .collect::<Vec<Tile>>();
    }

    pub fn set_entities(&mut self) {
        self.entities = vec![
            Entity {
                location: Point { x: 5, y: 5 },
            },
            Entity {
                location: Point { x: 32, y: 77 },
            },
            Entity {
                location: Point { x: 42, y: 33 },
            },
            Entity {
                location: Point { x: 35, y: 5 },
            },
            Entity {
                location: Point { x: 19, y: 19 },
            },
            Entity {
                location: Point { x: 34, y: 43 },
            },
            Entity {
                location: Point { x: 99, y: 80 },
            },
            Entity {
                location: Point { x: 77, y: 66 },
            },
            Entity {
                location: Point { x: 55, y: 54 },
            },
            Entity {
                location: Point { x: 53, y: 1 },
            },
        ]
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tiles: { vec![] },
            entities: { vec![] },
        }
    }

    pub fn get_entities(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.entities).unwrap()
    }

    pub fn get_tiles(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.tiles).unwrap()
    }

    /// returns sorted by row.
    /// one line andy
    pub fn sort_entities(&mut self) {
        let _ = &self
            .entities
            .sort_by(|a, b| a.location.y.cmp(&b.location.y));
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    wasm_logger::init(wasm_logger::Config::default());
}
