use std::f32::consts::PI;

use bevy::{prelude::*, input::mouse::MouseMotion, core_pipeline::clear_color::ClearColorConfig};
use super::AppState;
use crate::consts;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_cam)
            .add_system(camera_move_system.in_set(OnUpdate(AppState::Camera)))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1.1,
            });
    }
}

#[derive(Default, Component)]
pub struct CamRotation {
    yaw: f32,
    pitch: f32
}

pub fn spawn_cam(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.87, 0.87, 0.855)),
                ..default()
            },
            ..default()
        },
        CamRotation::default()
    ));
    // commands.spawn(
    //     DirectionalLightBundle {
    //         directional_light: DirectionalLight {
    //             illuminance: 7000.,
    //             shadows_enabled: false,
    //             ..default()
    //         },
    //         transform: Transform::looking_at(
    //             Transform::from_translation(Vec3::ZERO),
    //             Vec3::new(1., -2., 1.), 
    //             Vec3::Y
    //         ),
    //         ..default()
    //     }
    // );
}

pub fn movement_axis<const N: usize>(input: &Res<Input<KeyCode>>, plus: [KeyCode; N], minus: [KeyCode; N]) -> f32 {
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
    mut ev_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CamRotation)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut m_delta = Vec2::ZERO;
    let (mut tf, mut rot) = query.single_mut();
    for e in ev_motion.iter() {
        m_delta += e.delta;
    }
    m_delta*=consts::MOUSE_SENS;

    rot.pitch += m_delta.y;
    rot.yaw -= m_delta.x;
	rot.pitch = rot.pitch.clamp(-PI/2., PI/2.);
    if rot.yaw > 2.*PI {rot.yaw-=2.*PI;}
    if rot.yaw < -2.*PI {rot.yaw+=2.*PI;}

    tf.rotation = 
        Quat::from_axis_angle(Vec3::Y, rot.yaw) *
		Quat::from_axis_angle(-Vec3::X, rot.pitch);

    let mut movespeed = consts::CAM_SPEED;
    if keyboard.pressed(KeyCode::LShift) {movespeed*=consts::SHIFT_BOOST;}

    let lr = movement_axis(&keyboard, [KeyCode::D, KeyCode::Right], [KeyCode::A, KeyCode::Left]);
    let fb = movement_axis(&keyboard, [KeyCode::S, KeyCode::Down], [KeyCode::W, KeyCode::Up]);
    let ud = movement_axis(&keyboard, [KeyCode::Space], [KeyCode::LControl]);
    let k_delta = tf.rotation.mul_vec3(Vec3::new(lr, ud, fb))*movespeed*time.delta_seconds();
    tf.translation += k_delta;
}