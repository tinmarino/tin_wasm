use std::f32::consts::PI;

pub const PI: f32 = std::f32::consts::PI;


pub const FIELD_OF_VIEW: f32 = 45. * PI / 180.; //in radians
pub const GRID_SIZE: usize = 100;
pub const Z_FAR: f32 = 100.;
pub const Z_NEAR: f32 = 0.1;
pub const Z_PLANE: f32 = -2.414213; //-1 / tan(pi/8)
