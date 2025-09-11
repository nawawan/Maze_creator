use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

pub fn fetch_2d_context(canvas_name: &str) -> CanvasRenderingContext2d {
    let document = document();

    // windowにHtmlCanvasElementがあること前提
    let canvas_elem = get_element_by_id(&document, canvas_name);

    let canvas = canvas_elem
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("canvas element should have converted into HtmlCanvasElement");

    context(&canvas)
}

fn window() -> Window {
    web_sys::window().expect("no global window exists")
}

fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn get_element_by_id(doc: &Document, id: &str) -> Element {
    doc.get_element_by_id(id)
        .expect(&format!("document should have a {}", id))
}

fn context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("canvas should has a 2d context")
        .expect("2d context is not available")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("convert should be succeeded")
}
