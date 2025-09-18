mod algo;
mod dom;
mod maze;
use wasm_bindgen::prelude::*;

use crate::algo::shape::Point;
use crate::maze::{random_maze, single_stroke_maze};

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
}

#[wasm_bindgen]
pub enum MazeType {
    Random,
    SingleStroke,
}

#[wasm_bindgen]
pub fn draw_maze(
    left_top_x: f64,
    left_top_y: f64,
    row: usize,
    col: usize,
    space: f64,
    maze: MazeType,
) {
    let validated_input = match maze {
        MazeType::Random => random_maze::validate(row, col, space),
        MazeType::SingleStroke => single_stroke_maze::validate(row, col, space),
    };

    if !validated_input {
        return;
    }

    let ctx = dom::fetch_2d_context("canvas");

    let from = Point::new(left_top_x, left_top_y);
    let width = space * col as f64;
    let height = space * row as f64;

    ctx.clear_rect(from.x, from.y, width, height);

    ctx.begin_path();

    match maze {
        MazeType::Random => {
            ctx.rect(0.0, 0.0, width, height);
            random_maze::draw_maze(&ctx, col, row, space)
        }
        MazeType::SingleStroke => {
            single_stroke_maze::draw_maze(&ctx, col, row, space);
        }
    };
    ctx.stroke();
}
