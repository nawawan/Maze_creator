mod maze;
mod dom;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn draw_maze() {
    let context = dom::fetch_2d_context("canvas");
    let _ = maze::draw_rect::run(&context);
}