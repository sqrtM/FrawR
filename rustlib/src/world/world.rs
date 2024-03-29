use crate::entity::entity::Attributes;
use crate::{entity::entity::Entity, tile::tile::Point, Creatures, Tile, World};
use crate::{entity::entity::EntityType, tile::tile::TileType};

use std::collections::BTreeMap;
use std::cmp::Ordering;

use wasm_bindgen::prelude::*;

use noise::{self, NoiseFn};


#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Self {
        let p: Entity = EntityType::Player.create(Point { x: 0, y: 0 }, Attributes {constitution: 10, strength: 10, madness: 10, intelligence: 10});
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
        w.build_map(43);
        w.set_entities();
        w
    }

    pub fn build_map(&mut self, rand: u32) {
        let simp: noise::OpenSimplex = noise::OpenSimplex::new(rand);
        let mut btree: BTreeMap<Point, Tile> = BTreeMap::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let pt = Point { x: j, y: i };
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
            self.tiles = btree.clone();
        }
    }

    pub fn set_entities(&mut self) {
        self.creatures.entities = vec![];
        for i in 1..self.height {
            self.creatures
                .entities
                .push(EntityType::Enemy.create(Point { x: i, y: i }, Attributes {constitution: 10, strength: 10, madness: 10, intelligence: 10}))
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

impl World {
    pub fn is_tile_traversable(&self, p: Point) -> bool {
        let mut val: bool = false;
        if let Some(tile) = self.tiles.get(&p) {
            val = tile.traversable;
        }
        val
    }
}
