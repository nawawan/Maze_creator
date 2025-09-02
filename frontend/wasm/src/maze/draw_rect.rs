use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};


pub fn run(ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {

    ctx.begin_path();
    ctx.rect(10.0, 10.0, 20.0, 30.0);
    ctx.stroke();

    Ok(())
}