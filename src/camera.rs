/// From: https://github.com/chinedufn/webgl-water-tutorial/blob/master/src/app/store/camera.rs 
/// Camera
/// The perspective projection matrix, is used to mimic the effects of a typical camera serving as the stand-in for the viewer in the 3D virtual world.
/// The view matrix is responsible for moving the objects in the scene to simulate the position of the camera being changed, altering what the viewer is currently able to see.
/// More:
/// * Explaination: https://webglfundamentals.org/webgl/lessons/webgl-3d-camera.html
/// * Code: https://github.com/sebcrozet/kiss3d/blob/master/src/camera/first_person.rs
extern crate nalgebra as na;
use nalgebra::{
    Perspective3,
    Isometry3,
    Vector3,
    Point3,
    UnitQuaternion,
};

use super::constants::*;
//use super::util::*;
use std::collections::HashSet;



// Translate speed
const TS: f32 = 0.1;
// Rotate Speed
const RS: f32 = 0.03;


pub struct Camera {
    pub position: [f32; 3],
    pub pitch: f32,
    pub yaw: f32,
    pub keys_down: HashSet<u32>,
    pub projection: Perspective3<f32>,
}

lazy_static!{
    static ref ROTATION_TO_Y_UP: UnitQuaternion<f32> = UnitQuaternion::rotation_between_axis(&Vector3::y_axis(), &Vector3::y_axis())
        .unwrap_or_else(|| {
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::PI)
        });
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: [0.0, 0.0, -6.0],
            pitch: -PI/2.0,
            yaw: -PI/2.0,
            keys_down: HashSet::with_capacity(32),
            projection: Perspective3::new(PI/3.0, 1.0, 0.1, 50.0),
        }
    }

    /// Update in game loop according to (async) key pressed
    pub fn update(&mut self) {
        for key in self.get_keys_down() {
            self.key_press(key);
        }
    }

    /// Helper: Copy to keep me immutable
    pub fn get_keys_down(&self) -> HashSet<u32> {
        self.keys_down.clone()
    }

    /// Try to translate and rotate
    pub fn key_press(&mut self, key: u32) -> () {
        self.translate(key);
        self.rotate(key);
    }

    /// The direction this camera is being moved by the keyboard keys for a given set of key states.
    pub fn translate(&mut self, key: u32) -> () {
        let t = self.observer_frame();
        let frontv = t * Vector3::z();
        let leftv = t * Vector3::x();
        let upv = t * Vector3::y();

        let mut movement = na::zero::<Vector3<f32>>();

        movement += match key {
            JS_KEY_W => frontv,
            JS_KEY_S => -frontv,
            JS_KEY_A => leftv,
            JS_KEY_D => -leftv,
            JS_KEY_R => upv,
            JS_KEY_F => -upv,
            _ => return,
        };

        if movement != Vector3::new(0.0, 0.0, 0.0) {
            movement = movement.normalize();
        }

        let move_amount = movement * TS;
        self.position[0] += move_amount[0];
        self.position[1] += move_amount[1];
        self.position[2] += move_amount[2];
    }

    /// Rotate camera view
    pub fn rotate(&mut self, key: u32) -> () {
        self.pitch += match key {
            JS_KEY_DOWN => -RS,
            JS_KEY_UP => RS,
            _ => 0.0,
        };
        self.yaw += match key {
            JS_KEY_LEFT => -RS,
            JS_KEY_RIGHT => RS,
            _ => 0.0,
        };
    }

    /// The point the camera is looking at.
    pub fn at(&self) -> Point3<f32> {
        let x = self.position;
        let (pitch, yaw) = (self.pitch, self.yaw);

        let view_eye = *ROTATION_TO_Y_UP * Point3::new(x[0], x[1], x[2]); //self.eye;
        let ax = view_eye.x + yaw.cos() * pitch.sin();
        let ay = view_eye.y + pitch.cos();
        let az = view_eye.z + yaw.sin() * pitch.sin();

        ROTATION_TO_Y_UP.inverse() * Point3::new(ax, ay, az)
    }


    /// Retuns the uProjectionMatrix for uniform_matrix4fv_with_f32_array
    pub fn project(&self) -> &[f32] {
        self.projection.as_matrix().as_slice()
    }

    /// Get eye as point3
    pub fn eye(&self) -> Point3<f32> {
        let x = self.position;
        Point3::new(x[0], x[1], x[2])
    }

    /// Get the view matrix -> Self position rotation
    pub fn view(&self) -> Isometry3<f32> {
        Isometry3::look_at_rh(&self.eye(), &self.at(), &Vector3::y())
    }

    /// The camera observer local frame.
    fn observer_frame(&self) -> Isometry3<f32> {
        Isometry3::face_towards(&self.eye(), &self.at(), &Vector3::y()) //&self.coord_system.up_axis)
    }
}
