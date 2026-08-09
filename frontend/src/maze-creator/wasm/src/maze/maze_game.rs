use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::dom;

use super::wall_grid::{self, DOWN, LEFT, RIGHT, UP};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
pub struct MazeGame {
    canvas_id: String,
    walls: Vec<u8>,
    rows: usize,
    cols: usize,
    space: f64,
    player_row: usize,
    player_col: usize,
    goal_row: usize,
    goal_col: usize,
}

#[wasm_bindgen]
impl MazeGame {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas_id: String,
        walls: Vec<u8>,
        rows: usize,
        cols: usize,
        space: f64,
    ) -> MazeGame {
        let game = MazeGame {
            canvas_id,
            walls,
            rows,
            cols,
            space,
            player_row: 0,
            player_col: 0,
            goal_row: rows.saturating_sub(1),
            goal_col: cols.saturating_sub(1),
        };
        game.render();
        game
    }

    // 移動できた場合はtrueを返し再描画する。壁に阻まれた場合はfalseを返し何もしない。
    pub fn try_move(&mut self, direction: Direction) -> bool {
        if self.rows == 0 || self.cols == 0 {
            return false;
        }

        let idx = self.player_row * self.cols + self.player_col;
        let cell = self.walls[idx];
        let (bit, row_delta, col_delta): (u8, i64, i64) = match direction {
            Direction::Up => (UP, -1, 0),
            Direction::Right => (RIGHT, 0, 1),
            Direction::Down => (DOWN, 1, 0),
            Direction::Left => (LEFT, 0, -1),
        };

        if cell & bit == 0 {
            return false;
        }

        let new_row = self.player_row as i64 + row_delta;
        let new_col = self.player_col as i64 + col_delta;
        if new_row < 0
            || new_col < 0
            || new_row as usize >= self.rows
            || new_col as usize >= self.cols
        {
            return false;
        }

        self.player_row = new_row as usize;
        self.player_col = new_col as usize;
        self.render();
        true
    }

    pub fn is_cleared(&self) -> bool {
        self.player_row == self.goal_row && self.player_col == self.goal_col
    }

    pub fn player_row(&self) -> usize {
        self.player_row
    }

    pub fn player_col(&self) -> usize {
        self.player_col
    }

    fn render(&self) {
        let ctx = dom::fetch_2d_context(&self.canvas_id);
        let width = self.space * self.cols as f64;
        let height = self.space * self.rows as f64;

        ctx.clear_rect(0.0, 0.0, width, height);

        ctx.begin_path();
        ctx.rect(0.0, 0.0, width, height);
        wall_grid::draw_walls(&ctx, &self.walls, self.rows, self.cols, self.space);
        ctx.stroke();

        draw_marker(&ctx, self.goal_row, self.goal_col, self.space, "#e53935");
        draw_marker(
            &ctx,
            self.player_row,
            self.player_col,
            self.space,
            "#1e88e5",
        );
    }
}

fn draw_marker(ctx: &CanvasRenderingContext2d, row: usize, col: usize, space: f64, color: &str) {
    let center_x = col as f64 * space + space / 2.0;
    let center_y = row as f64 * space + space / 2.0;
    let radius = space * 0.3;

    ctx.save();
    ctx.set_fill_style_str(color);
    ctx.begin_path();
    let _ = ctx.arc(center_x, center_y, radius, 0.0, std::f64::consts::PI * 2.0);
    ctx.fill();
    ctx.restore();
}
