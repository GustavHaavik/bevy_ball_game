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

#[derive(Resource)]
pub struct SpritesSheet(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn get_index(sprite: TileSprite, columns: i32) -> i32 {
    let (col, row) = (sprite.column, sprite.row);

    (row * columns) + col
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

    let sprite_sheet_handle: Handle<TextureAtlas> = get_texture_atlas(
        "sprites/spritesheet.png".to_string(),
        28,
        9,
        None,
        TILE_W as f32,
        TILE_H as f32,
        &mut texture_atlases,
        &assets_server,
    );

    commands.insert_resource(TileMap(tilemap_handle));
    commands.insert_resource(SpritesSheet(sprite_sheet_handle));
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
