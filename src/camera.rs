/// From: https://github.com/chinedufn/webgl-water-tutorial/blob/master/src/app/store/camera.rs 
/// Camera
/// The perspective projection matrix, is used to mimic the effects of a typical camera serving as the stand-in for the viewer in the 3D virtual world.
/// The view matrix is responsible for moving the objects in the scene to simulate the position of the camera being changed, altering what the viewer is currently able to see.
//use std::sync::Arc;
//use crate::util::*;
use nalgebra::{
    Perspective3,
    Isometry3,
    Vector3,
    Translation3,
    UnitQuaternion,
    Point3,
};
use super::constants::*;
use super::util::*;
use std::collections::HashSet;



// Touch speed
const TS: f32 = 0.1;
// Rotate Speed
const RS: f32 = 0.01;


pub struct Camera {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub projection: Perspective3<f32>,
    pub displace: Isometry3<f32>,
    pub keys_down: HashSet<u32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: [0.0, 0.0, 0.0],
            target: [0.0, 0.0, 0.0],
            projection: Perspective3::new(PI/3.0, 1.0, 0.1, 50.0),
            // A matrix of self displacement
            displace: Isometry3::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., 0.),
            ),
            keys_down: HashSet::with_capacity(32),
        }
    }

    pub fn key_press(&mut self, key: u32) -> () {

        web_sys::console::log_1(&(&*format!("Key {:?}", key) as &str).into());
        let translate = match key {
            JS_KEY_W => [ 0., 0., -TS],
            JS_KEY_S => [ 0., 0., TS],
            JS_KEY_A => [-TS, 0., 0.],
            JS_KEY_D => [TS, 0., 0.],
            _ => [0.0, 0.0, 0.0],
        };
        self.position = add_array3(self.position, translate);
    }

    /// Copy to keep me immutable
    pub fn get_keys_down(&self) -> HashSet<u32> {
        self.keys_down.clone()
    }

    /// Update according to key pressed
    pub fn update(&mut self) {
        for key in self.get_keys_down() {
            self.key_press(key);
        }
    }

    /// Retuns the uProjectionMatrix for uniform_matrix4fv_with_f32_array
    pub fn project(&self) -> &[f32] {
        // TODO auto convert to array [f32]
        self.projection.as_matrix().as_slice()
    }

    /// Get the view matrix -> Self position rotation
    pub fn view(&self) -> Isometry3<f32> {
        //self.displace.inverse()
        let x = self.position;
        let eye = Point3::new(x[0], x[1], x[2]);

        let x = self.target;
        let target = Point3::new(x[0], x[1], x[2]);

        Isometry3::look_at_rh(&eye, &target, &Vector3::y())
    }

    /// Append translation
    pub fn translate_arr(&mut self, a: [f32; 3]){
        self.displace.append_translation_mut(&Translation3::new(a[0], a[1], a[2]));
    }

    /// Append rotation
    pub fn rotate_arr(&mut self, a: [f32; 3]){
        let rot = UnitQuaternion::from_scaled_axis(Vector3::new(a[0], a[1], a[2]));
        self.displace.append_rotation_mut(&rot);
    }

    pub fn forward(&mut self, amount: f32) -> () {
        self.displace.append_translation_mut(&Translation3::new(amount, amount, amount));
        ()
    }
}
