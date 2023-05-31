use log;
use noise::{self, NoiseFn};
use serde::Serialize;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}



#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Tile {
    val: f64,
    name: TileType
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
enum TileType {
    Floor,
    Slope,
    Wall
}

#[wasm_bindgen]
pub struct World {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl World {
    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }
}

#[wasm_bindgen]
impl World {
    pub fn build_map() -> World {
        log::warn!("start");
        let height = 1000;
        let width = 1000;
        let simp = noise::OpenSimplex::new(7894789);
        
        let tiles = (0..width * height)
            .map(|i| {
                let row = (i % width) as f64;
                let col = (i as f64 / height as f64).floor();
                let val = simp.get([row, col]);
                let name = match val {
                    -0.4..=-0.2 => TileType::Wall,
                    -0.2..=0.0 => TileType::Slope,
                    0.0..=0.3 => TileType::Floor,
                    _ => TileType::Floor
                };
                Tile { val: val, name: name }
            })
            .collect();

        World {
            width,
            height,
            tiles
        }
    }

    pub fn render(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.tiles).unwrap()
    }
}

use std::fmt;

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.tiles.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = cell.val;
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    wasm_logger::init(wasm_logger::Config::default());
}