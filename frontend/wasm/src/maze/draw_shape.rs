use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

use crate::maze::shape::Point;

pub fn line(ctx: &CanvasRenderingContext2d, from: Point, to: Point) -> Result<(), JsValue> {

    ctx.begin_path();
    ctx.move_to(from.x, from.y);
    ctx.line_to(to.x, to.y);
    ctx.stroke();

    Ok(())
}