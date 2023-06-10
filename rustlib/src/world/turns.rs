use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    entity::{entity::Entity, moves::Moves},
    tile::tile::Point,
    World,
};

#[wasm_bindgen]
impl World {
    pub fn take_player_turn(&mut self, action: u8) -> () {
        let targ: Point = Self::get_target_tile(self, self.creatures.player, action);
        if action != 4 {
            let res: Option<Entity> = Self::check_tile_for_entities(self, targ);
            match res {
                Some(_e) => log::debug!("ouch!!!!"),
                None => {
                    return match action {
                        0 => self.creatures.player.move_up(),
                        1 => self.creatures.player.move_down(),
                        2 => self.creatures.player.move_left(),
                        3 => self.creatures.player.move_right(),
                        _ => log::debug!("key not bound")
                    }
                }
            }
        } else {
            self.creatures.player.stay_still()
        }   
    }

    pub fn take_entity_turn(&mut self, i: usize, action: u8) -> () {
        let targ: Point = Self::get_target_tile(self, self.creatures.entities[i], action);
        let res: Option<Entity> = Self::check_tile_for_entities(self, targ);
        match res {
            Some(_e) => log::debug!("ouch!!!!"),
            None => {
                return match action {
                    0 => self.creatures.entities[i].move_up(),
                    1 => self.creatures.entities[i].move_down(),
                    2 => self.creatures.entities[i].move_left(),
                    3 => self.creatures.entities[i].move_right(),
                    4 => self.creatures.entities[i].stay_still(),
                    _ => log::debug!("key not bound")
                }
            }
        }
    }

    pub fn take_turn_and_return(&mut self, action: u8) -> JsValue {
        Self::take_player_turn(self, action);
        for i in 0..self.creatures.entities.len() {
            let rand = (js_sys::Math::random() * 5.0) as u8;
            Self::take_entity_turn(self, i, rand);
        }
        Self::get_all_creatures(&self)
    }
}
