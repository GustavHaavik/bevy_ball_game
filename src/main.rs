use bevy::{prelude::*, window::PrimaryWindow};

pub mod constants;
mod game;
pub mod sprites;

use bevy_pancam::{PanCam, PanCamPlugin};
use game::*;
use sprites::load_sprite_sheets;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PanCamPlugin::default(),
        ))
        .add_systems(PreStartup, load_sprite_sheets)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            )),
            ..default()
        })
        .insert(PanCam::default());
}

// print mouse position
fn inspect_mouse_pos(q_windows: Query<&Window, With<PrimaryWindow>>) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
