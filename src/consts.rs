use std::f32::consts::PI;

pub const CAM_SPEED: f32 = 20.; // speed of wasd movement
pub const SHIFT_BOOST: f32 = 4.0; // how much more speeder holding shift
pub const MOUSE_SENS: f32 = 0.001*2.*PI; // mouse sens of camera

pub const BASE_WINDOW_X: f32 = 1200.;
pub const BASE_WINDOW_Y: f32 = 900.;

pub const MODE_SWITCH: bevy::prelude::KeyCode = bevy::prelude::KeyCode::Z;