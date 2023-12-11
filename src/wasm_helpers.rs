use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().unwrap()
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn create_canvas() -> HtmlCanvasElement {
    let canv = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canv.set_width(window().inner_width().unwrap().as_f64().unwrap() as u32);
    canv.set_height(window().inner_height().unwrap().as_f64().unwrap() as u32 - 10);
    document().body().unwrap().append_child(&canv).unwrap();

    canv
}

pub fn canvas() -> HtmlCanvasElement {
    document()
        .get_elements_by_tag_name("canvas")
        .item(0)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

pub fn canvas_width() -> f64 {
    canvas().width().into()
}

pub fn canvas_height() -> f64 {
    canvas().height().into()
}

pub fn canvas_context() -> CanvasRenderingContext2d {
    document()
        .get_elements_by_tag_name("canvas")
        .item(0)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}
