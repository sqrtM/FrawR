mod entity;
mod tile;

use std::cmp::Ordering;

use entity::entity::Entity;
use log::Log;
use noise::{self, NoiseFn};
use serde::Serialize;
use struct_iterable::Iterable;
use tile::tile::{Point, Tile};
use wasm_bindgen::prelude::*;

use crate::{
    entity::entity::{EntityType, Moves},
    tile::tile::TileType,
};

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Iterable)]
pub struct Creatures {
    entities: Vec<Entity>,
    player: Entity,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct World {
    tiles: Vec<Tile>,
    creatures: Creatures,
    width: i32,
    height: i32,
}

#[wasm_bindgen]
impl World {
    pub fn build_map(&mut self, rand: u32) {
        let simp: noise::OpenSimplex = noise::OpenSimplex::new(rand);

        self.tiles = (0..self.width * self.height)
            .map(|i: i32| {
                let row: f64 = (i % self.width) as f64;
                let col: f64 = (i as f64 / self.height as f64).floor();
                let val: i16 = (simp.get([row, col]) * 100.) as i16;
                let tile_type: TileType = match val {
                    -30..=-10 => TileType::Wall,
                    -9..=15 => TileType::Slope,
                    16..=40 => TileType::Floor,
                    _ => TileType::Floor,
                };
                Tile {
                    val,
                    name: tile_type,
                    char: tile_type.get_char(),
                    location: Point {
                        x: i % self.width,
                        y: i / self.width,
                    },
                }
            })
            .collect::<Vec<Tile>>();
    }

    pub fn set_entities(&mut self) {
        self.creatures.entities = vec![];

        for i in 1..200 {
            self.creatures
                .entities
                .push(EntityType::Enemy.get(Point { x: i, y: 5 }, i.try_into().unwrap()))
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Self {
        let p: Entity = EntityType::Player.get(Point { x: 0, y: 0 }, 0);
        let e: Vec<Entity> = { vec![] };
        let mut w: World = World {
            tiles: { vec![] },
            creatures: Creatures {
                player: p,
                entities: { e },
            },
            width,
            height,
        };
        w.build_map(389238);
        w.set_entities();
        w
    }

    pub fn get_entities(&mut self) -> JsValue {
        Self::sort_entities(self);
        serde_wasm_bindgen::to_value(&self.creatures.entities).unwrap()
    }

    pub fn get_tiles(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.tiles).unwrap()
    }

    pub fn get_player(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.creatures.player).unwrap()
    }

    pub fn get_all_creatures(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.creatures).unwrap()
    }

    /// sorts by row, then by col
    pub fn sort_entities(&mut self) {
        let _ = &self
            .creatures
            .entities
            .sort_unstable_by(|a: &Entity, b: &Entity| {
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
                Self::move_player(self, action);
                Self::move_entities(self);
            }
        }
        serde_wasm_bindgen::to_value(&self.creatures.entities).unwrap()
    }

    fn move_player(&mut self, action: u8) {
        match action {
            0 => {
                let res = Self::check_tile_for_entities(
                    self,
                    Point {
                        x: self.creatures.player.location.x,
                        y: self.creatures.player.location.y - 1,
                    },
                );
                if let None = res {
                    self.creatures.player.move_up()
                } else {
                    log::debug!("there's something there!!!!")
                }
            }
            1 => {
                let res = Self::check_tile_for_entities(
                    self,
                    Point {
                        x: self.creatures.player.location.x,
                        y: self.creatures.player.location.y - 1,
                    },
                );
                if let None = res {
                    self.creatures.player.move_down()
                } else {
                    log::debug!("there's something there!!!!")
                }
            },
            2 => {
                let res = Self::check_tile_for_entities(
                    self,
                    Point {
                        x: self.creatures.player.location.x - 1,
                        y: self.creatures.player.location.y,
                    },
                );
                if let None = res {
                    self.creatures.player.move_left()
                } else {
                    log::debug!("there's something there!!!!")
                }
            },
            3 => {
                let res = Self::check_tile_for_entities(
                    self,
                    Point {
                        x: self.creatures.player.location.x + 1,
                        y: self.creatures.player.location.y,
                    },
                );
                if let None = res {
                    self.creatures.player.move_right()
                } else {
                    log::debug!("there's something there!!!!")
                }
            },
            _ => self.creatures.player.stay_still(),
        }
    }

    /// NOT EFFICENT!!! VERY STINKY!!!!
    fn move_entities(&mut self) {
        
        for i in &mut self.creatures.entities {
            // find a way to just check if it implements the @moves trait
            let rand: u8 = (js_sys::Math::random() * 5.0) as u8;
            match rand {
                0 => i.move_up(),
                1 => i.move_down(),
                2 => i.move_left(),
                3 => i.move_right(),
                _ => i.stay_still(),
            }
            if i.hunger > 0 {
                i.hunger -= 1
            } else {
                if !i.status_effects.hungry {
                    i.status_effects.hungry = true;
                }
            }
        }
        Self::sort_entities(self)
    }

    pub fn take_turn_and_return(&mut self, action: u8) -> JsValue {
        Self::take_turn(self, action);
        Self::get_all_creatures(&self)
    }
}

impl World {
    pub fn check_tile_for_entities(&mut self, p: Point) -> Option<Entity> {
        self.sort_entities();
        let m: Result<usize, usize> = self
            .creatures
            .entities
            .binary_search_by(|i| i.location.partial_cmp(&p).expect("weird"));
        match m {
            Ok(i) => {
                log::info!("found!!!");
                Some(self.creatures.entities[i])
            }
            Err(_) => {
                log::info!("checked. nothing");
                None
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    wasm_logger::init(wasm_logger::Config::default());
}
