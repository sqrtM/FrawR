use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tile::tile::Point;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Entity {
    pub location: Point,
    pub char: char,
    pub mood: Mood,
    pub status_effects: StatusEffects,
    pub status_bars: StatusBars,
    // eventually, we will implement a "target"
    // to smooth out the wandering/hunting
    // and update it with whatever it's going
    // towards
    //pub target: Point
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Mood {
    PlayerControlled,
    Hunting,
    Wandering,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct StatusEffects {
    pub hungry: bool,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct StatusBars {
    pub health: StatusBar,
    pub mana: ManaBars,
    pub hunger: StatusBar,
    pub sanity: StatusBar,
    pub rage: StatusBar,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct ManaBars {
    pub alpha: StatusBar,
    pub beta: StatusBar,
    pub gamma: StatusBar,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct StatusBar {
    pub max: u8,
    pub current: u8,
}

pub enum EntityType {
    Player,
    Enemy,
}

impl EntityType {
    pub fn get(&self, point: Point) -> Entity {
        match self {
            EntityType::Player => Entity {
                location: point,
                char: '@',
                mood: Mood::PlayerControlled,
                status_effects: StatusEffects { hungry: false },
                status_bars: StatusBars {
                    health: StatusBar {
                        max: 100,
                        current: 100,
                    },
                    mana: ManaBars {
                        alpha: StatusBar { max:100, current: 100 },
                        beta: StatusBar { max: 100, current: 100 },
                        gamma: StatusBar { max: 100, current: 100 },
                    },
                    hunger: StatusBar {
                        max: 100,
                        current: 100,
                    },
                    sanity: StatusBar {
                        max: 100,
                        current: 100,
                    },
                    rage: StatusBar {
                        max: 100,
                        current: 100,
                    },
                },
            },
            EntityType::Enemy => Entity {
                location: point,
                char: 'W',
                mood: Mood::Wandering,
                status_effects: StatusEffects { hungry: false },
                status_bars: StatusBars {
                    health: StatusBar {
                        max: 100,
                        current: 100,
                    },
                    mana: ManaBars {
                        alpha: StatusBar { max:100, current: 100 },
                        beta: StatusBar { max: 100, current: 100 },
                        gamma: StatusBar { max: 100, current: 100 },
                    },
                    hunger: StatusBar {
                        max: 100,
                        current: 100,
                    },
                    sanity: StatusBar {
                        max: 100,
                        current: 100,
                    },
                    rage: StatusBar {
                        max: 100,
                        current: 100,
                    },
                },
            },
        }
    }
}
