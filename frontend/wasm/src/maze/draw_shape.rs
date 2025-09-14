use web_sys::CanvasRenderingContext2d;

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
    from: (usize, usize),
    to: (usize, usize),
    space: f64,
) {
    let from = Point::new(from.0 as f64 * space, from.1 as f64 * space);
    let to = Point::new(to.0 as f64 * space, to.1 as f64 * space);
    ctx.move_to(from.y, from.x);
    ctx.line_to(to.y, to.x);
}

pub fn set_grid_boundary(
    ctx: &CanvasRenderingContext2d,
    from: (usize, usize),
    to: (usize, usize),
    space: f64,
) {
    if from.0 == to.0 {
        set_line_between_grid(ctx, (from.0, to.1), (to.0 + 1, to.1), space);
    }
    if from.1 == to.1 {
        set_line_between_grid(ctx, (to.0, from.1), (to.0, to.1 + 1), space);
    }
}

pub fn draw_lines(
    ctx: &CanvasRenderingContext2d,
    edges: Vec<(usize, usize)>,
    width: usize,
    space: f64,
) {
    for (from, to) in edges {
        let start = (from / width, from % width);
        let end = (to / width, to % width);
        set_line_between_grid(ctx, start, end, space);
    }
}
