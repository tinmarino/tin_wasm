/// Utilities
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Append a canvas to main window
pub fn create_canvas(id: &str) -> Result<web_sys::HtmlCanvasElement, JsValue>{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas").expect("Cannot create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Failed at casting canvas to a rust type");
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.set_id(id);
    log!("Drawing paint");
    canvas.style().set_property("border", "solid")?;
    Ok(canvas)
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
