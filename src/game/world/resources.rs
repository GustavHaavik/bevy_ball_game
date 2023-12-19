use bevy::prelude::*;

use super::components::*;

#[derive(Resource)]
pub struct TileStorage {
    pub tiles: Vec<Tile>,
}

#[derive(Resource)]
pub struct Overworld {
    pub tiles: TileStorage,
}

// #[derive(Resource)]
// pub struct Textures {
//     pub grass: Handle<Image>,
//     pub water: Handle<Image>,
// }