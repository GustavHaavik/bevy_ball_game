use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub sprite: i32,
}
