/// Utilities
use wasm_bindgen::{ JsCast, JsValue };
use web_sys::{
    // console,
    HtmlCanvasElement,
};

use js_sys::Function;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::convert::FromWasmAbi;



#[allow(dead_code)]
pub fn add_array3(a: [f32; 3], b:[f32; 3]) -> [f32; 3] {
    let mut c = [0.0; 3];
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
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

/// Helper mouse handler
pub fn add_handler<T>(
        name: &str,
        canvas: &HtmlCanvasElement,
        closure: impl FnMut(T) + 'static)
    -> Result<(), JsValue>
        where T: FromWasmAbi + 'static {
        //where F: FnMut(MouseEvent) + 'static{
    let handler = Closure::wrap(Box::new(closure) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback(name, handler.as_ref().unchecked_ref() as &Function)?;
    handler.forget();
    Ok(())
}
