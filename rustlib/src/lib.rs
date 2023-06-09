mod entity;
mod tile;
mod world;

use std::{collections::BTreeMap};

use entity::entity::Entity;
use serde::Serialize;
use struct_iterable::Iterable;
use tile::tile::{Point, Tile};
use wasm_bindgen::prelude::*;

use crate::entity::moves::Moves;


#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Iterable)]
pub struct Creatures {
    entities: Vec<Entity>,
    player: Entity,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct World {
    tiles: BTreeMap<(i32, i32), Tile>,
    creatures: Creatures,
    width: i32,
    height: i32,
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
                let res: Option<Entity> = Self::check_tile_for_entities(
                    self,
                    Point {
                        x: self.creatures.player.location.x,
                        y: self.creatures.player.location.y - 1,
                    },
                );
                if let None = res {
                    self.creatures.player.move_up()
                } else {
                    ////log::debug!("there's something there!!!!")
                }
            }
            1 => {
                let res: Option<Entity> = Self::check_tile_for_entities(
                    self,
                    Point {
                        x: self.creatures.player.location.x,
                        y: self.creatures.player.location.y + 1,
                    },
                );
                if let None = res {
                    self.creatures.player.move_down()
                } else {
                    ////log::debug!("there's something there!!!!")
                }
            }
            2 => {
                let res: Option<Entity> = Self::check_tile_for_entities(
                    self,
                    Point {
                        x: self.creatures.player.location.x - 1,
                        y: self.creatures.player.location.y,
                    },
                );
                if let None = res {
                    self.creatures.player.move_left()
                } else {
                    ////log::debug!("there's something there!!!!")
                }
            }
            3 => {
                let res: Option<Entity> = Self::check_tile_for_entities(
                    self,
                    Point {
                        x: self.creatures.player.location.x + 1,
                        y: self.creatures.player.location.y,
                    },
                );
                if let None = res {
                    self.creatures.player.move_right()
                } else {
                    //////log::debug!("there's something there!!!!")
                }
            }
            _ => self.creatures.player.stay_still(),
        }
    }

    /// NOT EFFICENT!!! VERY STINKY!!!!
    fn move_entities(&mut self) {
        Self::sort_entities(self);

        for i in 0..self.creatures.entities.len() {
            // find a way to just check if it implements the @moves trait
            let rand: u8 = (js_sys::Math::random() * 5.0) as u8;
            match rand {
                0 => {
                    let res: Option<Entity> = Self::check_tile_for_entities(
                        self,
                        Point {
                            x: self.creatures.entities[i].location.x,
                            y: self.creatures.entities[i].location.y - 1,
                        },
                    );
                    if let None = res {
                        self.creatures.entities[i].move_up()
                    } else {
                        //////log::debug!("there's something there!!!!")
                    }
                }
                1 => {
                    let res: Option<Entity> = Self::check_tile_for_entities(
                        self,
                        Point {
                            x: self.creatures.entities[i].location.x,
                            y: self.creatures.entities[i].location.y + 1,
                        },
                    );
                    if let None = res {
                        self.creatures.entities[i].move_down()
                    } else {
                        ////log::debug!("there's something there!!!!")
                    }
                }
                2 => {
                    let res: Option<Entity> = Self::check_tile_for_entities(
                        self,
                        Point {
                            x: self.creatures.entities[i].location.x - 1,
                            y: self.creatures.entities[i].location.y,
                        },
                    );
                    if let None = res {
                        self.creatures.entities[i].move_left()
                    } else {
                        ////log::debug!("there's something there!!!!")
                    }
                }
                3 => {
                    let res: Option<Entity> = Self::check_tile_for_entities(
                        self,
                        Point {
                            x: self.creatures.entities[i].location.x + 1,
                            y: self.creatures.entities[i].location.y,
                        },
                    );
                    if let None = res {
                        self.creatures.entities[i].move_right()
                    } else {
                        ////log::debug!("there's something there!!!!")
                    }
                }
                _ => self.creatures.entities[i].stay_still(),
            }
            if self.creatures.entities[i].hunger > 0 {
                self.creatures.entities[i].hunger -= 1
            } else {
                if !self.creatures.entities[i].status_effects.hungry {
                    self.creatures.entities[i].status_effects.hungry = true;
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
        let m: Result<usize, usize> = self
            .creatures
            .entities
            .binary_search_by(|i| i.location.partial_cmp(&p).expect("weird"));
        match m {
            Ok(i) => Some(self.creatures.entities[i]),
            Err(_) => {
                //log::info!("checked {:?}. nothing", p);
                None
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    wasm_logger::init(wasm_logger::Config::default());
}
