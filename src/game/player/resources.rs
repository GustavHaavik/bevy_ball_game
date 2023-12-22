use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerMovementSheet(pub Handle<TextureAtlas>);