/// From: Javascript: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial
/// web_sys::console::log_1(&(&*format!("Now {:?}", now) as &str).into());

use wasm_bindgen::prelude::*;
use web_sys::console;

#[macro_use]
extern crate lazy_static;

mod util;
mod util_gl;
mod game;
mod camera;
mod constants;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Hi
    console::log_1(&format!( "Main Rust start" ).into());

    // Go
    game::start()?;

    // Bye
    console::log_1(&format!( "Main Rust end" ).into());
    Ok(())
}
