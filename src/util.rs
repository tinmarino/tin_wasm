/// Utilities
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::camera::Camera;

/// Other single container, track changes
pub struct State {
    pub cube_rotation: f32,
    pub camera: Camera,
}

/// Append a canvas to main window
pub fn create_canvas(id: &str) -> Result<web_sys::HtmlCanvasElement, JsValue>{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas").expect("Cannot create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Failed at casting canvas to a rust type");
    // Focusable
    canvas.set_tab_index(1);
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(800);
    canvas.set_height(800);
    canvas.set_id(id);
    log!("Drawing paint");
    canvas.style().set_property("border", "solid")?;
    Ok(canvas)
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
