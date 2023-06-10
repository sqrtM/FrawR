use crate::{entity::entity::Entity, tile::tile::Point, Creatures, Tile, World};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

use std::cmp::Ordering;

use noise::{self, NoiseFn};

use crate::{entity::entity::EntityType, tile::tile::TileType};

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Self {
        let p: Entity = EntityType::Player.get(Point { x: 0, y: 0 });
        let e: Vec<Entity> = { vec![] };
        let mut w: World = World {
            tiles: { BTreeMap::new() },
            creatures: Creatures {
                player: p,
                entities: { e },
            },
            width,
            height,
        };
        w.build_map(389238);
        w.set_entities();
        w
    }

    pub fn build_map(&mut self, rand: u32) {
        let simp: noise::OpenSimplex = noise::OpenSimplex::new(rand);

        let mut btree: BTreeMap<Point, Tile> = BTreeMap::new();
        for i in 0..self.width * self.height {
            let pt: Point = Point {
                x: i % self.width,
                y: i / self.height,
            };
            let val: i16 = (simp.get([pt.x as f64, pt.y as f64]) * 100.) as i16;
            let tile_type: TileType = match val {
                -30..=-10 => TileType::Wall,
                -9..=15 => TileType::Slope,
                16..=40 => TileType::Floor,
                _ => TileType::Floor,
            };
            let tile: Tile = Tile {
                val,
                name: tile_type,
                char: tile_type.get_char(),
                traversable: true,
            };
            btree.entry(pt).or_insert(tile);
        }
        self.tiles = btree;
    }

    pub fn set_entities(&mut self) {
        self.creatures.entities = vec![];

        for i in 1..self.height {
            self.creatures
                .entities
                .push(EntityType::Enemy.get(Point { x: i, y: i }))
        }
    }

    /// sorts by row, then by col
    pub fn sort_entities(&mut self) {
        let _ = &self
            .creatures
            .entities
            .sort_unstable_by(|a: &Entity, b: &Entity| {
                if a.location.y < b.location.y {
                    Ordering::Less
                } else if a.location.y == b.location.y {
                    if a.location.x < b.location.x {
                        Ordering::Less
                    } else if a.location.x == b.location.x {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Greater
                }
            });
    }
}
