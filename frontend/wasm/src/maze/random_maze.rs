use web_sys::CanvasRenderingContext2d;

use crate::{
    algo::{grid, kruskal},
    maze::draw_shape::set_grid_boundary,
};

pub fn validate(row: usize, col: usize, space: f64) -> bool {
    return !(row == 0 || col == 0 || !space.is_finite() || space <= 0.0);
}

pub fn draw_maze(ctx: &CanvasRenderingContext2d, width: usize, height: usize, space: f64) {
    let unused_vertex = kruskal::extract_unused_maze_edges_by_kruskal(width, height, 1);

    for (node_left, node_right) in unused_vertex {
        let from = grid::index_1d_to_2d(node_left, width);
        let to = grid::index_1d_to_2d(node_right, width);
        set_grid_boundary(&ctx, from, to, space);
    }
}
