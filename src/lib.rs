use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_use]
extern crate lazy_static;

pub mod prelude;
mod util;
mod util_gl;
mod canvas;
mod webgl;
mod camera;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Hi
    log!("Main rust start");

    // Create page
    //let canvas = init_page();

    // Draw head
    //canvas::canvas_head(canvas);
    //canvas::canvas_paint()?;
    webgl::canvas_gl2()?;

    // Bye
    log!("Main rust end");
    Ok(())
}


/// Create page -> canvas
pub fn init_page() -> web_sys::HtmlCanvasElement {
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture paragraph
    let p = document.create_element("p").expect("could not create a p in body");
    p.set_inner_html("Hello Tin V0.1.1 from Rust!");

    // Manufacture canvas 
    let canvas = document
        .create_element("canvas")
        .expect("Create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Canvas well casted");
    // -- Set id
    canvas.set_id("id_canvas");


    body.append_child(&p).expect("Cant append p to body");

    body.append_child(&canvas).expect("Cant append canvas to body");

    canvas
}
