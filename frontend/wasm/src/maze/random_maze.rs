use web_sys::CanvasRenderingContext2d;

use crate::{
    algo::kruskal,
    maze::draw_shape::{draw_lines, extract_grid_boundary}, WallExistence,
};

pub fn validate(row: usize, col: usize, space: f64) -> bool {
    return !(row == 0 || col == 0 || !space.is_finite() || space <= 0.0);
}

pub fn draw_maze(ctx: &CanvasRenderingContext2d, width: usize, height: usize, space: f64, wall: WallExistence) {
    log::info!(
        "create maze in width: {}, height: {}, space: {}",
        width,
        height,
        space
    );
    let unused_vertex = kruskal::extract_maze_edges_by_kruskal(
        width,
        height,
        1,
        kruskal::KruskalResultEdge::Unused,
    );

    let edges = extract_grid_boundary(&unused_vertex);

    // let edges_in_wide_wall = expand_walls(edges);
    draw_lines(ctx, edges, space);
}
