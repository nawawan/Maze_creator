use web_sys::{CanvasRenderingContext2d};

use crate::algo::{grid, kruskal, single_stroke};
use crate::maze::shape::Point;


pub fn set_line_between_grid(ctx: &CanvasRenderingContext2d, from: (usize, usize), to: (usize, usize), space: f64)  {
    let from = Point::new(from.0 as f64 * space, from.1 as f64 * space);
    let to = Point::new(to.0 as f64 * space, to.1 as f64 * space);
    ctx.move_to(from.x, from.y);
    ctx.line_to(to.x, to.y);
}

pub fn set_wall_edges(ctx: &CanvasRenderingContext2d, width: usize, height: usize, space: f64) {
    let unused_vertex = kruskal::extract_unused_maze_edges_by_kruskal(width, height, 1);

    for (node_left, node_right) in unused_vertex {
        let from = grid::index_1d_to_2d(node_left, width);
        let to = grid::index_1d_to_2d(node_right, width);
        set_grid_boundary(&ctx, from, to, space);
    }
}

pub fn set_grid_boundary(ctx: &CanvasRenderingContext2d, from: (usize, usize), to: (usize, usize), space: f64) {
    if from.0 == to.0 {
        set_line_between_grid(ctx, (from.0, to.1), (to.0 + 1, to.1), space);
    }
    if from.1 == to.1 {
        set_line_between_grid(ctx, (to.0, from.1), (to.0, to.1 + 1), space);
    }
}

pub fn set_single_stroke_maze(ctx: &CanvasRenderingContext2d, width: usize, height: usize, space: f64) {
    let edges = single_stroke::single_stroke_maze(width, height);
    draw_lines(ctx, edges, width, space);
}

fn draw_lines(ctx: &CanvasRenderingContext2d, edges: Vec<(usize, usize)>, width: usize, space: f64) {
    for (from, to) in edges {
        let start = (from / width, from % width);
        let end = (to / width, to % width);
        set_line_between_grid(ctx, start, end, space);
    }
}