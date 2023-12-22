use bevy::prelude::*;

pub const TILE_W: usize = 16;
pub const TILE_H: usize = 16;

pub const SPRITE_SCALE_FACTOR: usize = 2;

pub trait SpriteSheet {}

pub struct TileSprite {
    pub column: i32,
    pub row: i32,
}

#[derive(Resource)]
pub struct TileMap(pub Handle<TextureAtlas>);

pub fn get_index(sprite: TileSprite) -> Result<i32, String> {
    let (col, row) = (sprite.column, sprite.row);

    if col > 11 || row > 10 {
        return Err(String::from("Invalid sprite index"));
    }

    let idx = (row * 12) + col;

    Ok(idx)
}

pub fn load_sprite_sheets(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tilemap_handle = get_texture_atlas(
        "sprites/tilemap.png".to_string(),
        12,
        11,
        Some(Vec2::new(1.0, 1.0)),
        TILE_W as f32,
        TILE_H as f32,
        &mut texture_atlases,
        &assets_server,
    );

    commands.insert_resource(TileMap(tilemap_handle));
}

pub fn get_texture_atlas(
    path: String,
    columns: usize,
    rows: usize,
    padding: Option<Vec2>,
    tile_width: f32,
    tile_height: f32,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets_server: &Res<AssetServer>,
) -> Handle<TextureAtlas> {
    let tilemap = assets_server.load(path);

    let texture_atlas = TextureAtlas::from_grid(
        tilemap,
        Vec2::new(tile_width, tile_height as f32),
        columns,
        rows,
        padding,
        None,
    );

    texture_atlases.add(texture_atlas)
}
