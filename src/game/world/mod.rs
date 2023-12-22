use bevy::prelude::*;

pub mod components;
mod systems;
// pub mod resources;

use systems::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_world);
    }
}
