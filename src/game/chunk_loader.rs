use bevy::{color::palettes::css::{BLUE, GRAY, GREEN, LIGHT_GRAY, RED, WHITE, YELLOW}, prelude::*, sprite_render::{TileData, TilemapChunk, TilemapChunkTileData}};
use noise::{NoiseFn, Perlin};
use rand::Rng as _;

use crate::game::{screen::{PixelatedCanvas, HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS, RES_HEIGHT, RES_WIDTH}, PixelGizmos};

pub struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (update_tileset_image, debug_chunk, draw_chunk_border));
    }
}

#[derive(Resource)]
pub struct DebugOptions {
    show_grid: bool,
}

pub const TILEMAP_COLS: usize = 40;
pub const TILEMAP_ROWS: usize = 23;
pub const CHUNK_COLS: usize = TILEMAP_COLS - 2;
pub const CHUNK_ROWS: usize = TILEMAP_ROWS - 2;
pub const TILE_SIZE: u32 = 8;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.insert_resource(DebugOptions{show_grid:false});
    let chunk_size = UVec2::new(TILEMAP_COLS as u32, TILEMAP_ROWS as u32);
    let tile_display_size = UVec2::splat(TILE_SIZE);
    /*let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|_| {Some(TileData::from_tileset_index(rng.gen_range(0..4)))})
        .collect();
    */
    let mut tile_data: Vec<Option<TileData>> = Vec::new();
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(1);

    for y in -1..=(CHUNK_ROWS as i32) {
        for x in -1..=(CHUNK_COLS as i32) {
            let gx = (0. * 39. + x as f32) as f64 * 0.1;
            let gy = (0. * 22. + y as f32) as f64 * 0.1;
            let val = perlin.get([gx, gy, 0.1]);

            if val > 0.3 {
                tile_data.push(Some(TileData::from_tileset_index(rng.gen_range(0..4))));
            } else {
                tile_data.push(None)
            }
        }
    }

    commands.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets.load("tileset.png"),
            ..default()
        },
        Transform::from_xyz(0., -2., 0.),
        TilemapChunkTileData(tile_data),
    ));
}

fn update_tileset_image(
    chunk_query: Single<&TilemapChunk>,
    mut events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    let chunk = *chunk_query;
    for event in events.read() {
        if event.is_loaded_with_dependencies(chunk.tileset.id()) {
            let image = images.get_mut(&chunk.tileset).unwrap();
            image.reinterpret_stacked_2d_as_array(4);
        }
    }
}

fn draw_chunk_border(
	mut gizmos: Gizmos<PixelGizmos>,
) {
    gizmos.rect_2d(
        Isometry2d::new(
            Vec2::new(0., -2.), 
            Rot2::radians(0.)
        ),
        Vec2::new(38. * TILE_SIZE as f32 + 2., 19. * TILE_SIZE as f32 + 2.),
        WHITE,
    );
}

pub fn debug_chunk(
    mut gizmos: Gizmos,
    transform: Single<&Transform, With<PixelatedCanvas>>,
    mut debug_options: ResMut<DebugOptions>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        let show = debug_options.show_grid; 
        debug_options.show_grid = !show;
    }

    if !debug_options.show_grid {
        return;
    }

    let scale = transform.scale.xy();
    let pos = transform.translation.xy(); //+ Vec2::new(RES_WIDTH as f32/2., RES_HEIGHT as f32/-2.) * scale;
    /*
    let mut size = Vec2::new(RES_WIDTH as f32, RES_HEIGHT as f32) * scale;
    let mut offset = pos - size * 0.5;

    for y in 0..TILEMAP_ROWS {
        let ly = (y as f32) * (TILE_SIZE as f32) * scale.y;
        let start = Vec2::new(0., ly) + offset;
        let end = Vec2::new(TILEMAP_COLS as f32 * TILE_SIZE as f32 * scale.x, ly) + offset;
        gizmos.line_2d(start, end, WHITE);
    }
    gizmos.rect_2d(    
        Isometry2d::new(pos.xy(), Rot2::radians(0.)), 
        size, 
        WHITE 
    );
    for x in 0..TILEMAP_COLS {
        let lx= (x as f32) * (TILE_SIZE as f32) * scale.x;
        let start = Vec2::new(lx, 0.) + offset;
        let end = Vec2::new(lx, TILEMAP_ROWS as f32 * TILE_SIZE as f32 * scale.x) + offset;
        gizmos.line_2d(start, end, GREEN);
    }*/
    //gizmos.cross_2d(Isometry2d::new(pos-offset, Rot2::radians(0.)), 0.5, RED);
    gizmos.grid_2d(
        Isometry2d::new(pos-Vec2::new(0., 2.*scale.x.min(scale.y)), Rot2::radians(0.)), 
        UVec2::new(TILEMAP_COLS as u32, TILEMAP_ROWS as u32), 
        Vec2::splat(TILE_SIZE as f32 * scale.x.min(scale.y)),
        LIGHT_GRAY,
    );
}