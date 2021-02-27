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

    /// Retuns the uProjectionMatrix for uniform_matrix4fv_with_f32_array
    pub fn project(&self) -> &[f32] {
        // TODO auto convert to array [f32]
        self.projection.as_matrix().as_slice()
    }

    /// Get the view matrix -> Self position rotation
    pub fn view(&self) -> Isometry3<f32> {
        self.displace.inverse()
    }

    pub fn new_frame(&mut self){
        let state = get_curr_state();

        let x = state.position;
        let eye = Point3::new(x[0], x[1], x[2]);

        let x = state.target;
        let target = Point3::new(x[0], x[1], x[2]);

        Isometry3::look_at_rh(&eye, &target, &Vector3::y());
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
