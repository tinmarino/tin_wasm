/// From: https://github.com/chinedufn/webgl-water-tutorial/blob/master/src/app/store/camera.rs 
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
    console,
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

const KEY_A: u32 = 0x41;
const KEY_W: u32 = 0x57;
const KEY_S: u32 = 0x53;
const KEY_D: u32 = 0x44;

let mut g_state: State = None;

pub fn input(key: i32, x: f32, y:f32){
    console::log_1(&(&*format!("Calledback {:?}, {:?}, {:?}", key, x, y) as &str).into());
}

// the size for values of type `[(&str, dyn FnMut(MouseEvent))]` cannot be known at compilation time: doesn't have a size known at compile-time
pub fn attach_handlers(canvas: &HtmlCanvasElement, state: &State) -> Result<(), JsValue> {

    add_handler("mousedown", canvas, state, move |event: MouseEvent| {
        input(1, event.client_x() as f32, event.client_y() as f32);
    }).expect("Adding mousedown");

    //add_handler("mouseup", canvas, move |event: MouseEvent| {
    //    input(2, event.client_x() as f32, event.client_y() as f32);
    //}).expect("Adding mouseup");

    //add_handler("wheel", canvas, move |event: WheelEvent| {
    //    event.prevent_default();
    //    let zoom_amount: f32 = event.delta_y() as f32 / 50.;
    //    input(3, zoom_amount, 0.);
    //}).expect("Adding wheel");

    //add_handler("keydown", canvas, move |event: KeyboardEvent| {
    //    let key = event.key_code() as f32;
    //    input(4, key, 0.);
    //}).expect("Adding keydown");

    Ok(())
}

/// Helper mouse handler
// the trait bound `T: FromWasmAbi` is not satisfied: the trait `FromWasmAbi` is not implemented for `T`
// the parameter type `impl FnMut(T)` may not live long enough: ...so that the type `impl FnMut(T)` will meet its required lifetime bounds
// expected a `Fn<(T,)>` closure, found `impl FnMut(T) + 'static`: expected an `Fn<(T,)>` closure, found `impl FnMut(T) + 'static`
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

/*
 * Did no work
//pub fn add_handlers<T>(canvas: &HtmlCanvasElement, closures: &[(&str, Box<dyn Fn(T) + 'static>)] ) -> Result<(), JsValue>
//        where T: FromWasmAbi + 'static {
//        //where F: FnMut(MouseEvent) + 'static{
//    for (name, closure) in closures {
//        let heap = Box::new(*closure) as Box<dyn FnMut(_)>;
//        let handler = Closure::wrap(heap);
//        canvas.add_event_listener_with_callback(name, handler.as_ref().unchecked_ref() as &Function)?;
//        handler.forget();
//    }
//    Ok(())
//}

/// Memory
/// Helper: touch hanlder
trait A { fn f() -> Self; }
impl A for i32 { fn f() -> i32 { 42 } }
impl A for f64 { fn f() -> f64 { 3.14 } }
//impl A for ~str { fn f() -> ~str { "blah".to_owned() } }
fn main() {
    let x: i32 = A::f();
    let y: f64 = A::f();
}


impl A for &MouseEvent { fn f() -> Self }
*/

//pub fn add_handler<F: Fn(TouchEvent)>(name: &str, canvas: &HtmlCanvasElement, closure: F) -> Result<(), JsValue>
//        where F: Fn(TouchEvent) + 'static{
//    let handler = Closure::wrap(Box::new(closure) as Box<dyn FnMut(_)>);
//    canvas.add_event_listener_with_callback(name, handler.as_ref().unchecked_ref() as &Function)?;
//    handler.forget();
//    Ok(())
//}
