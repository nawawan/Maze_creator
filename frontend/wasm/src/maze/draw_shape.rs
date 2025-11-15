use web_sys::CanvasRenderingContext2d;
use std::{collections::HashSet};

use crate::algo::shape::Point;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn set_line_between_grid(
    ctx: &CanvasRenderingContext2d,
    from: Point<usize>,
    to: Point<usize>,
    space: f64,
) {
    let from = Point::new(from.x as f64 * space, from.y as f64 * space);
    let to = Point::new(to.x as f64 * space, to.y as f64 * space);
    ctx.move_to(from.y, from.x);
    ctx.line_to(to.y, to.x);
}

pub fn extract_grid_boundary(
    vertexes: &Vec<(Point<usize>, Point<usize>)>
) -> Vec<(Point<usize>, Point<usize>)>{
    let mut lines: Vec<(Point<usize>, Point<usize>)> = Vec::new();
    for (from, to) in vertexes {
        lines.push(grid_to_edge(from, to));
    }
    lines
}

pub fn draw_lines(
    ctx: &CanvasRenderingContext2d,
    edges: Vec<(Point<usize>, Point<usize>)>,
    space: f64,
) {
    for (from, to) in edges {
        set_line_between_grid(ctx, from, to, space);
    }
}

pub fn expand_walls(edges: &Vec<(Point<usize>, Point<usize>)>) {
    let (vertical_index, horizontal_index) = fetch_vertical_and_horizontal_edge_index(edges);
    
}

fn grid_to_edge(from: &Point<usize>, to: &Point<usize>) -> (Point<usize>, Point<usize>){
    if from.x == to.x {
        return (Point::new(from.x, to.y), Point::new(to.x + 1, to.y))
    }
    if from.y == to.y {
        return (
            Point::new(to.x, from.y),
            Point::new(to.x, to.y + 1));
    }
    return (*from, *to);
}

fn fetch_vertical_and_horizontal_edge_index(
    edges: &Vec<(Point<usize>, Point<usize>)>
) -> (Vec<usize>, Vec<usize>){
    let mut vertical_index: HashSet<usize> = HashSet::new();
    let mut horizontal_index: HashSet<usize> = HashSet::new();

    for (from, to) in edges {
        if from.x == to.x {
            vertical_index.insert(from.x);
        }
        if from.y == to.y {
            horizontal_index.insert(from.y);
        }
    }

    let mut vertical_index_vec: Vec<usize> = vertical_index.into_iter().collect();
    let mut horizontal_index_vec: Vec<usize> = horizontal_index.into_iter().collect();
    vertical_index_vec.sort();
    horizontal_index_vec.sort();

    (vertical_index_vec, horizontal_index_vec)
}