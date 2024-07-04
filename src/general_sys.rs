mod camera;

use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};

use crate::ui::UiState;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ReloadVoxelsEvent>()
            .add_plugin(camera::CameraPlugin)
            // .insert_resource(AppState::Ui)
            .add_state::<AppState>()
            .insert_resource(HandleHolder::default())
            .insert_resource(RotationConfig::default())
            .insert_resource(crate::schematic::Schematic::default())
            .add_startup_system(init_handles)
            .add_system(state_cycle_system)
            .add_system(reload_voxel_system);
        // app.add_system_to_stage(CoreStage::PreUpdate, state_cycle_system);
        // app.add_startup_system(spawn_debug_cube);
        // .add_system(exit_on_esc);
        // app.add_system_to_stage(CoreStage::PostUpdate, reload_voxel_system);
    }
}

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum AppState {
    #[default]
    Ui,
    Camera
}

#[derive(Resource)]
pub struct RotationConfig {
    pub scale: f32,
    pub rotx: f32,
    pub roty: f32,
    pub rotz: f32,
    pub yrange: (i32, i32), // min, max
    quat: Quat
}

#[derive(Resource, Default)]
pub struct HandleHolder {
    cube: Handle<Mesh>,
    materials: [Handle<StandardMaterial>; 8]
}

fn state_cycle_system(mut state: ResMut<State<AppState>>, keyboard: Res<Input<KeyCode>>, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if keyboard.just_pressed(crate::consts::MODE_SWITCH) {
        let mut window = windows.get_single_mut().unwrap();
        match state.0 {
            AppState::Ui => {
                state.0 = AppState::Camera;
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
                // window.set_cursor_grab_mode();
                // window.set_cursor_visibility(false);
            },
            AppState::Camera => {
                state.0 = AppState::Ui;
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
                // window.set_cursor_grab_mode(CursorGrabMode::None);
                // window.set_cursor_visibility(true);
            },
        }
    }
}

// fn exit_on_esc(keyboard: Res<Input<KeyCode>>, mut writer: EventWriter<bevy::app::AppExit>) {
//     if keyboard.just_pressed(KeyCode::Escape) {
//         writer.send(bevy::app::AppExit);
//     }
// }

fn init_handles (
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut handles: ResMut<HandleHolder>,
    asset_server: Res<AssetServer>,
) {
    handles.cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let texture = Some(asset_server.load("cubeface.png"));
    for (i, c) in [
        Color::RED,
        Color::rgb(1.0, 0.5, 0.),
        Color::YELLOW,
        Color::GREEN,
        Color::rgb(0., 0.5, 1.), // BLUE
        Color::rgb(0.3, 0., 0.8), // ueue
        Color::rgb(1.0, 0.1, 0.5), // aii
    ].into_iter().enumerate() {
        handles.materials[i] = materials.add(StandardMaterial {
            base_color_texture: texture.clone(),
            base_color: c,
            ..default()
        });
    }
    handles.materials[7] = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("tpface.png")),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
}

pub struct ReloadVoxelsEvent;

fn reload_voxel_system (
    mut commands: Commands,
    handles: Res<HandleHolder>,
    schematic: Res<crate::schematic::Schematic>,
    mut rot_con: ResMut<RotationConfig>,
    mut ui_state: ResMut<UiState>,
    mut reader: EventReader<ReloadVoxelsEvent>,
    previous: Query<Entity, With<Voxel>>
) {
    if reader.is_empty() {
        return;
    }
    reader.clear();

    for e in previous.iter() {
        commands.entity(e).despawn();
    }

    rot_con.compute_quat();
    let voxels = schematic.voxelize_with_transform(rot_con.quat, rot_con.scale);
    let mut miny = 100000; let mut maxy = -100000;
    for (_, y, _) in voxels.iter() {if *y<miny {miny = *y;} if *y>maxy {maxy = *y;}} // quick minmax search
    rot_con.yrange = (miny, maxy);
    ui_state.voxel_count = voxels.len();
    for p in voxels {
        let mat_index = {
            let diff = p.1 - ui_state.get_uptoy();
            if diff > 2 {continue;}
            else if diff > 0 {7}
            else {(p.1.rem_euclid(14) / 2) as usize}
        };
        let v = Vec3::new(p.0 as f32, p.1 as f32, p.2 as f32);
        commands.spawn((
            PbrBundle {
                mesh: handles.cube.clone(),
                material: handles.materials[mat_index].clone(),
                transform: Transform::from_translation(v),
                ..default()
            },
            Voxel
        ));
    }
}

#[derive(Component)]
pub struct Voxel;

impl Default for RotationConfig {
    fn default() -> Self {
        Self {scale: 1.,
            rotx: 0.,
            roty: 0.,
            rotz: 0.,
            yrange: (0, 0),
            quat: default(),
        }
    }
}

impl RotationConfig {
    pub fn compute_quat(&mut self) {
        self.quat = Quat::from_euler(EulerRot::YXZ, self.rotx, self.roty, self.rotz);
    }
}
