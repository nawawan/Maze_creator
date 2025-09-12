mod algo;
mod dom;
mod maze;
use console_error_panic_hook as _;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::maze::{
    draw_shape::{set_single_stroke_maze, set_wall_edges},
    shape::Point,
};

#[wasm_bindgen(start)]
pub fn start() {
    // Setup better panic messages in the console instead of `unreachable`.
    console_error_panic_hook::set_once();
}
fn log_str(s: &str) {
    console::log_1(&JsValue::from_str(s));
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

    set_wall_edges(&ctx, col, row, space);

    ctx.stroke();
}

#[wasm_bindgen]
pub fn draw_single_stroke_maze(from_x: f64, from_y: f64, row: usize, col: usize, space: f64) {
    log_str(&format!(
        "from_x: {}, from_y: {}, row: {}, col: {}, space: {}",
        from_x, from_y, row, col, space
    ));
    if row == 0 || col == 0 || !space.is_finite() || space <= 0.0 {
        return;
    }
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

    set_single_stroke_maze(&ctx, col, row, space);

    ctx.stroke();
}
