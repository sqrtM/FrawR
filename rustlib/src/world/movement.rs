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
            .position(|i| i.location.x == p.x && i.location.y == p.y)
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

#[cfg(test)]
mod tests {
    use crate::entity::entity::EntityType;

    use super::*;

    #[test]
    fn collision_detected() {
        let mut w = World::new(20, 20);

        let p1 = Point { x: 1, y: 2 };
        let _e1 = EntityType::Enemy.get(p1);

        let p2 = Point { x: 4893, y: 3432 };
        let _e2 = EntityType::Enemy.get(p2);

        let p3 = Point { x: 13, y: 222 };
        let _e3 = EntityType::Enemy.get(p1);

        let p4 = Point { x: 55, y: 55 };
        let _e4 = EntityType::Enemy.get(p4);

        assert_eq!(w.check_point_for_entities(p1), None);
        assert_eq!(w.check_point_for_entities(p2), None);
        assert_eq!(w.check_point_for_entities(p3), None);
        assert_eq!(w.check_point_for_entities(p4), None);
    }
}
