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
    from: Point<usize>,
    to: Point<usize>,
    space: f64,
) {
    let from = Point::new(from.x as f64 * space, from.y as f64 * space);
    let to = Point::new(to.x as f64 * space, to.y as f64 * space);
    ctx.move_to(from.y, from.x);
    ctx.line_to(to.y, to.x);
}

pub fn set_grid_boundary(
    ctx: &CanvasRenderingContext2d,
    from: Point<usize>,
    to: Point<usize>,
    space: f64,
) {
    if from.x == to.x {
        set_line_between_grid(
            ctx,
            Point::new(from.x, to.y),
            Point::new(to.x + 1, to.y),
            space,
        );
    }
    if from.y == to.y {
        set_line_between_grid(
            ctx,
            Point::new(to.x, from.y),
            Point::new(to.x, to.y + 1),
            space,
        );
    }
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
