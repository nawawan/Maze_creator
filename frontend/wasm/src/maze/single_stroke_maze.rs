use web_sys::CanvasRenderingContext2d;

use crate::{algo::single_stroke, maze::draw_shape::draw_lines, WallExistence};

pub fn validate(row: usize, col: usize, space: f64) -> bool {
    if row == 0 || col == 0 || !space.is_finite() || space <= 0.0 {
        return false;
    }
    if row % 2 == 1 && col % 2 == 1 {
        return false;
    }
    if row <= 2 && col <= 2 {
        return false;
    }

    return true;
}

pub fn draw_maze(ctx: &CanvasRenderingContext2d, width: usize, height: usize, space: f64, wall: WallExistence) {
    log::debug!(
        "{}",
        format!(
            "create single stroke maze in width: {}, height: {}, space: {}",
            width, height, space
        )
    );
    let edges = single_stroke::single_stroke_maze(width, height);
    draw_lines(ctx, edges, space);
}
