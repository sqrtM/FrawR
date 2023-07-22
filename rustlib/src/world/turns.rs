use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    entity::{
        entity::{Entity, Mood},
        moves::Moves,
    },
    tile::tile::Point,
    World,
};

#[wasm_bindgen]
impl World {
    pub fn take_player_turn(&mut self, action: u8) -> bool {
        let targ: Point = Self::get_target_tile(self, self.creatures.player, action);
        let mut turn: bool = self.is_tile_traversable(targ);
        if action != 4 && turn == true {
            match Self::check_point_for_entities(self, targ) {
                Some(e) => {
                    if e.mood == Mood::Dead {
                        match action {
                            0 => self.creatures.player.move_up(),
                            1 => self.creatures.player.move_down(),
                            2 => self.creatures.player.move_left(),
                            3 => self.creatures.player.move_right(),
                            _ => {
                                log::debug!("key not bound");
                                turn = false
                            }
                        }
                    } else {
                        log::debug!("{:?} HIT. SOMETHING SHOULD HAPPEN", e);
                        Self::handle_collision(e);
                        turn = false
                    }
                }
                None => match action {
                    0 => self.creatures.player.move_up(),
                    1 => self.creatures.player.move_down(),
                    2 => self.creatures.player.move_left(),
                    3 => self.creatures.player.move_right(),
                    _ => {
                        log::debug!("key not bound");
                        turn = false
                    }
                },
            }
        } else {
            self.creatures.player.stay_still()
        }
        turn
    }

    pub fn take_entity_turn(&mut self, i: usize, action: u8) -> () {
        let targ: Point = Self::get_target_tile(self, self.creatures.entities[i], action);
        if self.is_tile_traversable(targ) {
            match Self::check_point_for_entities(self, targ) {
                Some(e) => {
                    if e.mood == Mood::Dead {
                        Self::move_entity(e, action)
                    }
                }
                None => {
                    if self.creatures.entities[i].mood == Mood::Wandering {
                        Self::move_entity(&mut self.creatures.entities[i], action)
                    }
                }
            }
        }
    }

    fn move_entity(entity: &mut Entity, action: u8) {
        match action {
            0 => entity.move_up(),
            1 => entity.move_down(),
            2 => entity.move_left(),
            3 => entity.move_right(),
            4 => entity.stay_still(),
            _ => log::debug!("key not bound"),
        }
    }

    pub fn take_turn_and_return(&mut self, action: u8) -> JsValue {
        if Self::take_player_turn(self, action) {
            for i in 0..self.creatures.entities.len() {
                let rand: u8 = (js_sys::Math::random() * 5.0) as u8;
                Self::take_entity_turn(self, i, rand);
            }
        }
        Self::get_all_creatures(&self)
    }

    fn handle_collision(hit: &mut Entity) {
        // placeholder...
        if hit.status_bars.health.current >= 10 {
            hit.status_bars.health.current = hit.status_bars.health.current - 10
        } else {
            hit.die();
        }
    }
}
