mod entity;
mod tile;

use std::cmp::Ordering;

use entity::entity::Entity;
use noise::{self, NoiseFn};
use serde::Serialize;
use tile::tile::{Point, Tile};
use wasm_bindgen::prelude::*;

use crate::{entity::entity::{EntityType, Moves}, tile::tile::TileType};

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
        self.entities = Vec::new();
        for i in 1..29 {
            self.entities
                .push(EntityType::Enemy.get(Point { x: i, y: 5 }, i.try_into().unwrap()))
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tiles: { vec![] },
            entities: { vec![] },
        }
    }

    pub fn get_entities(&mut self) -> JsValue {
        Self::sort_entities(self);
        serde_wasm_bindgen::to_value(&self.entities).unwrap()
    }

    pub fn get_tiles(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.tiles).unwrap()
    }

    /// sorts by row, then by col
    pub fn sort_entities(&mut self) {
        let _ = &self.entities.sort_unstable_by(|a, b| {
            if a.location.y < b.location.y {
                Ordering::Less
            } else if a.location.y == b.location.y {
                if a.location.x < b.location.x {
                    Ordering::Less
                } else if a.location.x == b.location.x {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Greater
            }
        });
    }
}

#[wasm_bindgen]
impl World {
    pub fn take_turn(&mut self, action: u8) -> JsValue {
        match action {
            _ => {
                Self::move_entities(self);
            }
        }
        serde_wasm_bindgen::to_value(&self.entities).unwrap()
    }

    fn move_entities(&mut self) {
        for i in &mut self.entities {
            // find a way to just check if it implements the @moves trait
            if i.moves {
                let action = (js_sys::Math::random() * 5.0) as u8;
                if action == 0 {
                    i.move_up()
                }
                if action == 1 {
                    i.move_down()
                }
                if action == 2 {
                    i.move_left()
                }
                if action == 3 {
                    i.move_right()
                }
                if action == 4 {
                    i.stay_still()
                }
            }
        }
        Self::sort_entities(self)
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    wasm_logger::init(wasm_logger::Config::default());
}
