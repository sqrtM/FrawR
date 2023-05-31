use log;
use noise::{self, NoiseFn};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn build_map() {
    log::warn!("start");
    let mut arr = [[0.0; 1000]; 1000];
    let simp = noise::OpenSimplex::new(0);
    for i in 0..1000 {
        for j in 0..1000 {
            arr[i][j] = simp.get([i as f64, j as f64]);
        }
    }
    log::debug!("done");
}

#[wasm_bindgen(start)]
pub fn setup() {
    wasm_logger::init(wasm_logger::Config::default());
}