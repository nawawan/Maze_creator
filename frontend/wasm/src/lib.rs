mod algo;
mod dom;
mod maze;

use wasm_bindgen::prelude::*;

use crate::maze::{
    draw_shape::{set_single_stroke_maze, set_wall_edges},
    shape::Point,
};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn draw_maze(from_x: f64, from_y: f64, row: usize, col: usize, space: f64) {
    let ctx = dom::fetch_2d_context("canvas");

    let from = Point::new(from_x, from_y);
    let width = space * col as f64;
    let height = space * row as f64;

    ctx.clear_rect(from.x, from.y, width, height);

    ctx.begin_path();

    // 外枠を描画
    ctx.rect(from.x, from.y, width, height);

    set_wall_edges(&ctx, row, col, space);

    ctx.stroke();
}

#[wasm_bindgen]
pub fn draw_single_stroke_maze(from_x: f64, from_y: f64, row: usize, col: usize, space: f64) {
    if row % 2 == 1 && col % 2 == 1 {
        return;
    }
    if row <= 2 && col <= 2 {
        return;
    }
    let ctx = dom::fetch_2d_context("canvas");

    let from = Point::new(from_x, from_y);
    let width = space * col as f64;
    let height = space * row as f64;

    ctx.clear_rect(from.x, from.y, width, height);

    ctx.begin_path();

    set_single_stroke_maze(&ctx, row, col, space);

    ctx.stroke();
}
