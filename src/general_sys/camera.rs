use std::f32::consts::PI;

use bevy::prelude::*;
use super::AppState;
use crate::consts;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_cam)
            .add_systems(Update, camera_move_system);
    }
}

#[derive(Default, Component, Clone)]
pub struct CamRotation {
    yaw: f32,
    pitch: f32
}

pub fn spawn_cam(mut commands: Commands) {
    commands.spawn_scene(bsn!(
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.87, 0.87, 0.855))
        }
        Camera3d
        Transform::from_translation(Vec3::new(0., 0., 10.))
        AmbientLight {
            color: Color::WHITE,
            brightness: 550.,
        }
        CamRotation
    ));
}

pub fn movement_axis<const N: usize>(input: &Res<ButtonInput<KeyCode>>, plus: [KeyCode; N], minus: [KeyCode; N]) -> f32 {
	let mut axis = 0.0;
	if input.any_pressed(plus) {
		axis += 1.0;
	}
	if input.any_pressed(minus) {
		axis -= 1.0;
	}
	axis
}

pub fn camera_move_system(
    state: Res<State<AppState>>,
    mouse_motion: Res<bevy::input::mouse::AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &mut CamRotation)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    if state.get() != &AppState::Camera {
        return
    }
    let (mut tf, mut rot) = query.single_mut().unwrap();
    let m_delta = mouse_motion.delta * consts::MOUSE_SENS;

    rot.pitch += m_delta.y;
    rot.yaw -= m_delta.x;
	rot.pitch = rot.pitch.clamp(-PI/2., PI/2.);
    if rot.yaw > 2.*PI {rot.yaw-=2.*PI;}
    if rot.yaw < -2.*PI {rot.yaw+=2.*PI;}

    tf.rotation = 
        Quat::from_axis_angle(Vec3::Y, rot.yaw) *
		Quat::from_axis_angle(-Vec3::X, rot.pitch);

    let mut movespeed = consts::CAM_SPEED;
    if keyboard.pressed(KeyCode::ShiftLeft) {movespeed*=consts::SHIFT_BOOST;}

    let lr = movement_axis(&keyboard, [KeyCode::KeyD, KeyCode::ArrowRight], [KeyCode::KeyA, KeyCode::ArrowLeft]);
    let fb = movement_axis(&keyboard, [KeyCode::KeyS, KeyCode::ArrowDown], [KeyCode::KeyW, KeyCode::ArrowUp]);
    let ud = movement_axis(&keyboard, [KeyCode::Space], [KeyCode::ControlLeft]);
    let k_delta = tf.rotation.mul_vec3(Vec3::new(lr, ud, fb))*movespeed*time.delta_secs();
    tf.translation += k_delta;
}