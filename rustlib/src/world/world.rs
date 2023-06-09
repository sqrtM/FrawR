use wasm_bindgen::prelude::*;
use std::collections::BTreeMap;
use crate::{ Tile, Creatures, entity::entity::Entity, tile::tile::Point, World};

use std::{cmp::Ordering,};

use noise::{self, NoiseFn};


use crate::{
    entity::{entity::EntityType},
    tile::tile::TileType,
};

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Self {
        let p: Entity = EntityType::Player.get(Point { x: 0, y: 0 }, 0);
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

        let mut btree: BTreeMap<(i32, i32), Tile> = BTreeMap::new();
        for i in 0..self.width * self.height {
            let row: i32 = i % self.width;
            let col: i32 = i / self.height;
            let val: i16 = (simp.get([row as f64, col as f64]) * 100.) as i16;
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
            };
            btree.entry((row, col)).or_insert(tile);
        }
        self.tiles = btree;
    }

    pub fn set_entities(&mut self) {
        self.creatures.entities = vec![];

        for i in 1..200 {
            self.creatures
                .entities
                .push(EntityType::Enemy.get(Point { x: i, y: 5 }, i.try_into().unwrap()))
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