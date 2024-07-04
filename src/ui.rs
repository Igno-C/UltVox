use std::{fs::read_dir, path::Path};

use bevy::prelude::*;
use bevy_egui::{egui::{self, Context}, EguiContexts};
use crate::{general_sys::{AppState, ReloadVoxelsEvent, RotationConfig}, schematic::Schematic};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default());
        app.add_system(draw_ui_system.in_set(OnUpdate(AppState::Ui)));
    }
}

#[derive(Resource)]
pub struct UiState {
    ewindow_open: bool,
    helpwindow_open: bool,
    ron_files: Vec<String>,
    obj_files: Vec<String>,
    uptoy_slider: i32,
    pub voxel_count: usize,
}

impl Default for UiState {
    fn default() -> Self {
        let mut ret = Self {ewindow_open: false, helpwindow_open: false, ron_files: Vec::new(), obj_files: Vec::new(), uptoy_slider: 0, voxel_count: 0};
        ret.reload_files();
        ret
    }
}

impl UiState {
    fn reload_files(&mut self) {
        let spath = "./shapes/";
        let path = Path::new(&spath);
        let display = path.display();
    
        if !path.is_dir() {
            println!("Path {} isn't a directory somehow", display);
            return;
        }
        let dirs = match read_dir(path) {
            Ok(dir) => dir,
            Err(why) => {
                println!("couldn't read directory {}: {}", display, why);
                return;
            },
        };

        self.obj_files.clear();
        self.ron_files.clear();

        for entry in dirs {
            match entry {
                Ok(e) => {
                    let fname = e.file_name().into_string().expect("Somehow invalid file name");
                    if fname.ends_with(".ron") {
                        self.ron_files.push(fname);
                    }
                    else if fname.ends_with(".obj") {
                        self.obj_files.push(fname);
                    }
                },
                Err(why) => {
                    println!("Error on dir entry: {}", why);
                },
            }
        }
    }

    pub fn get_uptoy(&self) -> i32 {
        self.uptoy_slider
    }
}

fn draw_ui_system(
    mut ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut schematic: ResMut<Schematic>,
    mut rot_con: ResMut<RotationConfig>,
    reloader: EventWriter<ReloadVoxelsEvent>
) {
    let c = ctx.ctx_mut();
    let u = ui_state.as_mut();
    let s = schematic.as_mut();
    let rc = rot_con.as_mut();

    top_panel(c, u, s, rc, reloader);

    edit_window(c, u, s);

    help_window(c, u);
}

fn top_panel(
    ctx: &mut Context,
    ui_state: &mut UiState,
    schematic: &mut Schematic,
    rot_con: &mut RotationConfig,
    mut reloader: EventWriter<ReloadVoxelsEvent>
) {
    let UiState {ewindow_open, helpwindow_open, ron_files, obj_files, uptoy_slider, voxel_count} = ui_state;
    let RotationConfig {scale, rotx, roty, rotz, yrange, .. } = rot_con;
    let mut refresh_state = false;
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("Load schematic", |ui| {
                if ui.button("Refresh").clicked() {
                    refresh_state = true;
                }
                ui.separator();
                for ron_file in ron_files.iter() {
                    if ui.button(ron_file).clicked() {
                        schematic.load_from_file(ron_file);
                        reloader.send(ReloadVoxelsEvent);
                        ui.close_menu();
                    }
                }
                ui.separator();
                for obj_file in obj_files.iter() {
                    if ui.button(obj_file).clicked() {
                        schematic.load_from_obj_file(obj_file);
                        reloader.send(ReloadVoxelsEvent);
                        ui.close_menu();
                    }
                }
            });
            ui.separator();
            if ui.button("Reset rotations").clicked() {*scale = 1.; *rotx = 0.; *roty = 0.; *rotz = 0.;}
            ui.separator();
            ui.label("Rotations (ypr):");
            ui.drag_angle(rotx);
            ui.drag_angle(roty);
            ui.drag_angle(rotz);
            ui.label("Scale:"); ui.add(egui::DragValue::new(scale).speed(0.05));
            ui.label("Up to Y: "); 
            ui.add(
                egui::Slider::new(uptoy_slider, (yrange.0-1)..=yrange.1)
                    .integer()
            );
            if ui.button("Reload voxels").clicked() {reloader.send(ReloadVoxelsEvent);}
            ui.separator();
            ui.label(format!("Voxel count: {}", voxel_count));
            ui.separator();
            ui.add_space(10.);
            ui.toggle_value(ewindow_open, "Dumps");
            ui.toggle_value(helpwindow_open, "Help");
        });
    });

    if refresh_state {ui_state.reload_files();}
}

fn edit_window(ctx: &mut Context, ui_state: &mut UiState, schematic: &mut Schematic) {
    let UiState {ewindow_open, ..} = ui_state;
    egui::Window::new("Dumps")
        .open(ewindow_open)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            if ui.button("Dump example").clicked() {
                Schematic::example().save_to_file("example.ron");
                println!("Dumped");
            }
            if ui.button("Dump current").clicked() {
                schematic.save_to_file("currentdump.ron");
                println!("Dumped");
            }
        }
    );
}

fn help_window(ctx: &mut Context, ui_state: &mut UiState) {
    let UiState {helpwindow_open, ..} = ui_state;
    egui::Window::new("Help")
        .open(helpwindow_open)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.strong("Menu:");
            ui.label(
r#" - Load Schematic:  Load an .obj or .ron file holding a shape. The file must be in the ./shapes/ folder. 'Refresh' refreshes the list if a new file is put into or removed from the folder while the program is running.
 - Reset Rotations:  Resets rotations and scale to 0, 0, 0, 1.0.
 - Rotations (ypr):  Rotate the shape by Yaw, Pitch and Roll in degrees.
 - Scale:  Scale the shape by a factor. The sizes of shapes at scale 1.0 can vary greatly, use this to accommodate.
 - Up to Y:  Select Y level to render up to. The top two layers are transparent.
 - Reload Voxels:  After changing rotation, scale, or 'up to Y' use this to apply the changes.
 - Dumps:  Opens the schematic dump window.
'Dump example' dumps an example .ron file named 'example.ron' that showcases how to create custom schematics.
'Dump current' dumps the current loaded shape into a .ron schematic named 'currentdump.ron' - used mostly for debugging.
 - Help:  Opens this window. It's very helpful.
"#
            );
            ui.strong("Camera:");
            ui.label(
r#" - Press 'Z' to toggle between menu mode and camera control. Using the camera captures the cursor.
 - Use WASD to move forward/backward and to the sides. Arrow keys also work.
 - Spacebar and Control can be used to move the camera up and down as well.
 - Hold Shift to increase the camera movement speed by 4.0 times"#
            );
        }
    );
}
