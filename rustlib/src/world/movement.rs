use std::cmp::Ordering;

use crate::{entity::entity::Entity, tile::tile::Point, World};

impl World {
    pub fn get_target_tile(&mut self, entity: Entity, action: u8) -> Point {
        return match action {
            0 => Point {
                x: entity.location.x,
                y: entity.location.y - 1,
            },
            1 => Point {
                x: entity.location.x,
                y: entity.location.y + 1,
            },
            2 => Point {
                x: entity.location.x - 1,
                y: entity.location.y,
            },
            3 => Point {
                x: entity.location.x + 1,
                y: entity.location.y,
            },
            _ => Point {
                x: entity.location.x,
                y: entity.location.y,
            },
        };
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
            Err(_) => if self.creatures.player.location.cmp(&p) == Ordering::Equal {
                Some(self.creatures.player)
            } else {
                None
            }
        }
        // match self.tiles.get(&(p.x, p.y)) {
        //     Some(t) => Some(t),
        //     None => None,
        // }
    }   
}
