use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_world);
    }
}
