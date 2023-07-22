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
    /// recent refactor to use vec access of addresses rather than arrays.
    /// i would imagine this is faster/safer on intuition, but this is untested
    pub fn check_point_for_entities(&mut self, p: Point) -> Option<&mut Entity> {
        match self
            .creatures
            .entities
            .clone()
            .into_iter()
            .position(|i: Entity| i.location.x == p.x && i.location.y == p.y)
        {
            Some(i) => Some(&mut self.creatures.entities[i]),
            None => {
                if self.creatures.player.location.x == p.x
                    && self.creatures.player.location.y == p.y
                {
                    Some(&mut self.creatures.player)
                } else {
                    None
                }
            }
        }
    }
}
