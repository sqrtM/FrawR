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
    pub attributes: Attributes,
    // eventually, we will implement a "target"
    // to smooth out the wandering/hunting
    // and update it with whatever it's going
    // towards
    //pub target: Point
}

impl Entity {
    pub fn die(&mut self) {
        self.char = '_';
        self.mood = Mood::Dead;
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Mood {
    PlayerControlled,
    Hunting,
    Wandering,
    Dead,
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

/*
I imagine a somewhat inverted relation to these than most games
usually do. I like the idea of having serious tradeoffs that encourage
mixed gameplay rather than DCSS style "just sword and board it" gameplay.

for example, I think CONSTITUTION should
increase health,
decrease damage taken,
decrease rage build up,
increase hunger build up (you're big and strong),
decrease sanity loss.
So, you generally just lose offensive power in exchange for survivability.

Madness causes
increased alpha and gamma mana
decreased damage taken
increased rage build up
decreased hunger build up
decreased sanity loss (you're resistant to horrors, you're insane.)

intelligence causes
increased alpha and beta mana
increased magic damage
decreased magic damage taken
decreased rage build up (you're rational)
increased sanity loss

strength causes
increased physical damage
increased health
decreased physical damage taken
increased rage build up
*/
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Attributes {
    pub constitution: u16,
    pub strength: u16,
    pub madness: u16,
    pub intelligence: u16,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct ManaBars {
    pub alpha: StatusBar, //common to all -- uses hunger
    pub beta: StatusBar,  //magic and healing -- uses sanity (?)
    pub gamma: StatusBar, //binds and curses -- uses rage
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct StatusBar {
    pub max: u16,
    pub current: u16,
}

pub enum EntityType {
    Player,
    Enemy,
}

impl EntityType {
    pub fn create(&self, point: Point, attributes: Attributes) -> Entity {
        match self {
            EntityType::Player => Entity {
                location: point,
                char: '@',
                mood: Mood::PlayerControlled,
                status_effects: StatusEffects { hungry: false },
                status_bars: StatusBars {
                    health: StatusBar {
                        max: 100,
                        current: 20,
                    },
                    mana: ManaBars {
                        alpha: StatusBar {
                            max: 100,
                            current: 43,
                        },
                        beta: StatusBar {
                            max: 100,
                            current: 66,
                        },
                        gamma: StatusBar {
                            max: 100,
                            current: 24,
                        },
                    },
                    hunger: StatusBar {
                        max: 100,
                        current: 45,
                    },
                    sanity: StatusBar {
                        max: 100,
                        current: 34,
                    },
                    rage: StatusBar {
                        max: 100,
                        current: 40,
                    },
                },
                attributes,
            },
            EntityType::Enemy => Entity {
                location: point,
                char: 'W',
                mood: Mood::Wandering,
                status_effects: StatusEffects { hungry: false },
                status_bars: StatusBars {
                    health: StatusBar {
                        max: EntityType::calculate_total_health(attributes.constitution, attributes.strength, 1.0),
                        current: EntityType::calculate_total_health(attributes.constitution, attributes.strength, 1.0),
                    },
                    // for now, all of these will just impl CALC ATTR, but we'll get more specific algos later.
                    mana: ManaBars {
                        alpha: StatusBar {
                            max: 100,
                            current: 100,
                        },
                        beta: StatusBar {
                            max: 100,
                            current: 100,
                        },
                        gamma: StatusBar {
                            max: 100,
                            current: 100,
                        },
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
                attributes,
            },
        }
    }

    fn calculate_total_health(constitution: u16, strength: u16, modifier: f64) -> u16 {
        let c: f64 = constitution as f64;
        let s: f64 = strength as f64;

        ((c.log10() / (0.2 * c)) * (modifier * c * s.sqrt())) as u16
    }
}
