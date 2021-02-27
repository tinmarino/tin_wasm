/// From: https://github.com/chinedufn/webgl-water-tutorial/blob/master/src/app/store/camera.rs 
use std::sync::Arc;
use nalgebra::{
    Perspective3,
    Isometry3,
    Vector3,
    Translation3,
};  //, Isometry3, Point3, Vector3};

use super::constants::*;
use crate::util::*;


pub struct Camera {
    pub projection: Perspective3<f32>,
    pub displace: Isometry3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            projection: Perspective3::new(PI/3.0, 1.0, 0.1, 50.0),
            displace: Isometry3::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., 0.),
            ),
        }
    }

    /// Retuns the uProjectionMatrix for uniform_matrix4fv_with_f32_array
    pub fn view(&self) -> &[f32] {
        // TODO auto convert to array [f32]
        self.projection.as_matrix().as_slice()
    }

    pub fn displace(&self) -> Isometry3<f32> {
        self.displace
    }

    //pub fn displace(&self) -> [f32; 16] {
    //    // TODO cloning
    //    //let position = self.position.clone();
    //    //
    //    let mut res = [0.; 16];
    //    res.copy_from_slice(self.position.to_homogeneous().as_slice());
    //    res
    //}

    pub fn forward(&mut self, amount: f32) -> () {
        self.displace.append_translation_mut(&Translation3::new(amount, amount, amount));
        ()
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
use wasm_bindgen::{ JsCast, JsValue };
//use wasm_bindgen::prelude::*;

use wasm_bindgen::convert::FromWasmAbi;
use std::rc::Rc;
use std::cell::RefCell;
//use super::webgl::GameGl;
use super::webgl::GlContext;

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
