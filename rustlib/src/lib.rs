mod entity;
mod tile;

use std::cmp::Ordering;

use entity::entity::Entity;
use noise::{self, NoiseFn};
use serde::Serialize;
use tile::tile::{Point, Tile};
use wasm_bindgen::prelude::*;

use crate::{
    entity::entity::{EntityType, Moves},
    tile::tile::TileType,
};

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct World {
    tiles: Vec<Tile>,
    entities: Vec<Entity>,
    player: Entity,
    width: i32,
    height: i32
}

#[wasm_bindgen]
impl World {
    pub fn build_map(&mut self, rand: u32) {
        let simp = noise::OpenSimplex::new(rand);

        self.tiles = (0..self.width * self.height)
            .map(|i| {
                let row = (i % self.width) as f64;
                let col = (i as f64 / self.height as f64).floor();
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
                        x: i % self.width,
                        y: i / self.width,
                    },
                }
            })
            .collect::<Vec<Tile>>();
    }

    pub fn set_entities(&mut self) {
        let p = Entity {
            location: Point { x: 0, y: 0 },
            char: '@',
            NPC: false,
            id: 0,
            health: 100,
        };
        self.entities = vec![p];

        for i in 1..29 {
            self.entities
                .push(EntityType::Enemy.get(Point { x: i, y: 5 }, i.try_into().unwrap()))
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Self {
        let p = Entity {
            location: Point { x: 0, y: 0 },
            char: '@',
            NPC: true,
            id: 0,
            health: 100,
        };
        let e = { vec![p] };
        Self {
            tiles: { vec![] },
            player: p,
            entities: { e },
            width,
            height
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
                Self::move_entities(self, action);
            }
        }
        serde_wasm_bindgen::to_value(&self.entities).unwrap()
    }

    /// NOT EFFICENT!!! VERY STINKY!!!!
    fn move_entities(&mut self, action: u8) {
        for i in &mut self.entities {
            // find a way to just check if it implements the @moves trait
            if i.NPC {
                let rand = (js_sys::Math::random() * 5.0) as u8;
                if rand == 0 {
                    i.move_up()
                }
                if rand == 1 {
                    i.move_down()
                }
                if rand == 2 {
                    i.move_left()
                }
                if rand == 3 {
                    i.move_right()
                }
                if rand == 4 {
                    i.stay_still()
                }
            } else if !i.NPC {
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
