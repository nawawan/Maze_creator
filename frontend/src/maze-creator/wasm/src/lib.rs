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

// ランダム迷路を生成して1D(線)で描画し、通路のビットマスク(セルごとにUP/RIGHT/DOWN/LEFTが
// 開いているか)を返す。このビットマスクをJS側で保持しておくことで、生成した迷路そのものを
// プレイ画面や別の描画スタイル(2Dタイル表示など)に引き継げる。
#[wasm_bindgen]
pub fn create_random_maze(canvas_id: String, row: usize, col: usize, space: f64) -> Vec<u8> {
    let walls = maze::wall_grid::generate_walls(row, col);
    draw_random_maze(canvas_id, walls.clone(), row, col, space);
    walls
}

// 既に生成済みの壁ビットマスクを、1D(線)で再描画する。迷路を再生成せずに
// 表示スタイルだけを切り替えたい場合に使う。
#[wasm_bindgen]
pub fn draw_random_maze(canvas_id: String, walls: Vec<u8>, row: usize, col: usize, space: f64) {
    if !random_maze::validate(row, col, space) {
        return;
    }

    let ctx = dom::fetch_2d_context(&canvas_id);
    let width = space * col as f64;
    let height = space * row as f64;

    ctx.clear_rect(0.0, 0.0, width, height);
    ctx.begin_path();
    ctx.rect(0.0, 0.0, width, height);
    maze::wall_grid::draw_walls(&ctx, &walls, row, col, space);
    ctx.stroke();
}

// 既に生成済みの壁ビットマスクを、幅を持った2Dタイルとして再描画する。
// 壁も床も同じ大きさのタイルとして並べたタイルマップになる。
#[wasm_bindgen]
pub fn draw_random_maze_tiles(
    canvas_id: String,
    walls: Vec<u8>,
    row: usize,
    col: usize,
    tile_size: f64,
) {
    if row == 0 || col == 0 || !tile_size.is_finite() || tile_size <= 0.0 {
        return;
    }

    let ctx = dom::fetch_2d_context(&canvas_id);
    maze::tile_grid::draw_tiles(&ctx, &walls, row, col, tile_size);
}
