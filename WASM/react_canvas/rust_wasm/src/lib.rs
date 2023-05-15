use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(element_id: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let canvas = document.get_element_by_id(element_id)
        .expect("no canvas found");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();
    context
        .arc(150.0, 150.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();
    context.stroke();
}

