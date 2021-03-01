/// From: https://github.com/chinedufn/webgl-water-tutorial/blob/master/src/app/store/camera.rs 
/// Camera
/// The perspective projection matrix, is used to mimic the effects of a typical camera serving as the stand-in for the viewer in the 3D virtual world.
/// The view matrix is responsible for moving the objects in the scene to simulate the position of the camera being changed, altering what the viewer is currently able to see.
/// More:
/// * Explaination: https://webglfundamentals.org/webgl/lessons/webgl-3d-camera.html
/// * Code: https://github.com/sebcrozet/kiss3d/blob/master/src/camera/first_person.rs
//use std::sync::Arc;
//use crate::util::*;
extern crate nalgebra as na;
use nalgebra::{
    Perspective3,
    Isometry3,
    Vector3,
    Point3,
    Translation3,
    Rotation3,
    UnitQuaternion,
};
use super::constants::*;
use super::util::*;
use std::collections::HashSet;



// Touch speed
const TS: f32 = 0.1;
// Rotate Speed
const RS: f32 = 0.01;

const YAW_STEP: f32 = 0.005;
const PITCH_STEP: f32 = 0.005;


pub struct Camera {
    pub keys_down: HashSet<u32>,
    pub position: [f32; 3],
    pub target: [f32; 3],

    pub projection: Perspective3<f32>,
    pub displace: Isometry3<f32>,
    pub yaw: f32,
    pub pitch: f32,

    pub rotation_to_y_up: UnitQuaternion<f32>,
    //pub translate: Translation3<f32>,
    //pub rotate: Rotation3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            keys_down: HashSet::with_capacity(32),
            position: [0.0, 0.0, -6.0],
            target: [0.0, 0.0, 0.0],
            projection: Perspective3::new(PI/3.0, 1.0, 0.1, 50.0),
            // A matrix of self displacement
            displace: Isometry3::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., 0.),
            ),
            yaw: 0.0,
            pitch: 0.0,
            rotation_to_y_up: UnitQuaternion::rotation_between_axis(&Vector3::y_axis(), &Vector3::y_axis())
                .unwrap_or_else(|| {
                    UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::PI)
                }),

            //translate: Matrix3::new(0., 0., 6.),
            //rotate: Rotation3::new(0., 0., 0.),
        }
    }

    /// The point the camera is looking at.
    pub fn at(&self) -> Point3<f32> {
        let x = self.position;
        let view_eye = self.rotation_to_y_up * Point3::new(x[0], x[1], x[2]); //self.eye;
        let ax = view_eye.x + self.yaw.cos() * self.pitch.sin();
        let ay = view_eye.y + self.pitch.cos();
        let az = view_eye.z + self.yaw.sin() * self.pitch.sin();
        self.rotation_to_y_up.inverse() * Point3::new(ax, ay, az)
    }

    pub fn key_press(&mut self, key: u32) -> () {
        let translate = match key {
            JS_KEY_W => [ 0., 0., -TS],
            JS_KEY_S => [ 0., 0., TS],
            JS_KEY_A => [-TS, 0., 0.],
            JS_KEY_D => [TS, 0., 0.],
            _ => [0.0, 0.0, 0.0],
        };
        self.position = add_array3(self.position, translate);
        let rotate = match key {
            JS_KEY_LEFT => [ 0., 0., -RS],
            JS_KEY_RIGHT => [ 0., 0., RS],
            JS_KEY_DOWN => [-RS, 0., 0.],
            JS_KEY_UP => [RS, 0., 0.],
            _ => [0.0, 0.0, 0.0],
        };
        self.target = add_array3(self.target, rotate);
    }

    /// Update according to key pressed
    pub fn update(&mut self) {
        for key in self.get_keys_down() {
            self.key_press(key);
        }
    }


    pub fn get_model(&self) -> Isometry3<f32> {
        let x = self.position;
        let model = Isometry3::new(
            // Translate
            //Vector3::new(-0.0, 0.0, -6.0),
            Vector3::new(x[0], x[1], x[2]),
            // Rotate
            //Vector3::new(0.2, 0.7, 0.3).scale(read_state.cube_rotation),
            Vector3::new(0.0, 0.0, 0.0),
        );
        model.clone()
    }

    pub fn update_model(&self) -> [f32; 16] {
        let mut mat_model = self.get_model().to_homogeneous();
        //let mat_view = self.view().to_homogeneous();

        //let _ = mat_model
        //    .try_inverse()
        //    .map(|inverse_proj| mat_model = inverse_proj);

        // Convert -> [16]
        let mut arr_model = [0.; 16];
        arr_model.copy_from_slice(mat_model.as_slice());

        arr_model



        //let mut mat_model = model.to_homogeneous();

        //// Position
        //let x = self.position;
        //mat4.append_translation_mut(&Vector3::new(-x[0], -x[1], -x[2]));

        //// Rotation
        ////let x = self.position;
        ////let mat4 = mat4.rotate(&Vector3::new(-x[0], -x[1], -x[2]));

        //// Convert -> [16]
        //let mut mat_model = [0.; 16];
        //mat_model.copy_from_slice(mat4.as_slice());

        //mat_model
    }


    /// Copy to keep me immutable
    pub fn get_keys_down(&self) -> HashSet<u32> {
        self.keys_down.clone()
    }

    /// Retuns the uProjectionMatrix for uniform_matrix4fv_with_f32_array
    pub fn project(&self) -> &[f32] {
        self.projection.as_matrix().as_slice()
    }

    /// Get the view matrix -> Self position rotation
    pub fn view(&self) -> Isometry3<f32> {
        let x = self.position;
        let eye = Point3::new(x[0], x[1], x[2]);
        let at = Point3::new(0, 0, 0);

        web_sys::console::log_1(&(&*format!("Eye {:?}", eye) as &str).into());
        web_sys::console::log_1(&(&*format!("At {:?}", at) as &str).into());
        Isometry3::look_at_rh(&eye, &self.at(), &Vector3::y())
    }
}
