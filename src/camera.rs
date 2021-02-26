/// From: https://github.com/chinedufn/webgl-water-tutorial/blob/master/src/app/store/camera.rs 
use std::sync::Arc;
use nalgebra::{Perspective3};  //, Isometry3, Point3, Vector3};
use std::f32::consts::PI;

use crate::util::*;


pub struct Camera {
    projection: Perspective3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        let fovy = PI / 3.0;

        Camera {
            projection: Perspective3::new(fovy, 1.0, 0.1, 50.0),
        }
    }

    pub fn view(&self) -> &[f32] {
        // TODO auto convert to array [f32]
        self.projection.as_matrix().as_slice()
    }
}


//use wasm_bindgen::{ JsValue };
use web_sys::{
    //console,
    HtmlCanvasElement,
    MouseEvent,
    WheelEvent,
    KeyboardEvent,
};

use js_sys::Function;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
//use wasm_bindgen::prelude::*;

use wasm_bindgen::{JsValue};
use wasm_bindgen::convert::FromWasmAbi;

// the size for values of type `[(&str, dyn FnMut(MouseEvent))]` cannot be known at compilation time: doesn't have a size known at compile-time
// explicit lifetime required in the type of `state`: lifetime `'static` required: static to state
//pub fn attach_handlers(canvas: &HtmlCanvasElement, state: &'static mut State) -> Result<(), JsValue> {
pub fn attach_handlers(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    //let toto: &mut f32 = &mut state.cube_rotation;

    add_handler("mousedown", canvas, move |event: MouseEvent| {
        input(1, event.client_x() as f32, event.client_y() as f32);
        // Update game
        let mut state = STATE.lock().unwrap();
        *state = Arc::new(State {
            cube_rotation: state.cube_rotation + 1.0,
            ..*state.clone()
        });

    }).expect("Adding mousedown");

    add_handler("mouseup", canvas, move |event: MouseEvent| {
        input(2, event.client_x() as f32, event.client_y() as f32);
    }).expect("Adding mouseup");

    add_handler("wheel", canvas, move |event: WheelEvent| {
        event.prevent_default();
        let zoom_amount: f32 = event.delta_y() as f32 / 50.;
        input(3, zoom_amount, 0.);
    }).expect("Adding wheel");

    add_handler("keydown", canvas, move |event: KeyboardEvent| {
        let key = event.key_code() as f32;
        input(4, key, 0.);
    }).expect("Adding keydown");

    Ok(())
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
