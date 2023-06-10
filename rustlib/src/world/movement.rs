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
    /// this is stinky but i think it works consistently
    pub fn check_tile_for_entities(&mut self, p: Point) -> Option<Entity> {
        let m = self
            .creatures
            .entities
            .clone()
            .into_iter()
            .find(|i| i.location.x == p.x && i.location.y == p.y);
        match m {
            Some(i) => Some(i),
            None => {
                if self.creatures.player.location.x == p.x
                    && self.creatures.player.location.y == p.y
                {
                    Some(self.creatures.player)
                } else {
                    None
                }
            }
        }
    }
}
