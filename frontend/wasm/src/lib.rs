mod maze;

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
    maze::draw_rect::run();
}