use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod shapes;
mod voxelization;
mod general_sys;
mod ui;
mod schematic;
pub mod consts;

// magic angles are 0, 63.5, 17.3

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {title: "UltVox".into(), focused: true, ..default()}),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(general_sys::GeneralPlugin)
        .add_plugin(ui::UiPlugin)
        .run();
}