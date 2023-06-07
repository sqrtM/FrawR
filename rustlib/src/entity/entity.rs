use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tile::tile::Point;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Entity {
    pub location: Point,
    pub char: char,
    pub npc: bool,
    pub id: u32,
    pub health: u16,
    pub hunger: u8,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize)]
enum Status {
    Hungry = 0,
    Thirsty = 1,    
}

pub trait Moves {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn stay_still(&mut self);
}

impl Moves for Entity {
    fn move_up(&mut self) {
        self.location.y -= 1
    }

    fn move_down(&mut self) {
        self.location.y += 1
    }

    fn move_left(&mut self) {
        self.location.x -= 1
    }

    fn move_right(&mut self) {
        self.location.x += 1
    }

    fn stay_still(&mut self) {
        ()
    }
}

pub enum EntityType {
    //Player,
    Enemy
}

impl EntityType {
    pub fn get(&self, point: Point, id: u32) -> Entity {
        match self {
            //EntityType::Player => Entity { location: point, char: '@', id, moves: true },
            EntityType::Enemy => Entity { location: point, char: 'W', id, npc: true, health: 100, hunger: 10 },
        }
    }
}
