use bevy::{prelude::*, window::PrimaryWindow};

mod game;
use bevy_pancam::{PanCam, PanCamPlugin};
use game::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanCamPlugin::default()))
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
