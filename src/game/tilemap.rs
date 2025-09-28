use bevy::{color::palettes::css::{BLACK, RED}, prelude::*, sprite::Anchor};
use rand::Rng;
use crate::game::{OuterCamera, PixelatedCanvas, PIXEL_PERFECT_LAYERS, RES_HEIGHT, RES_WIDTH};

const COLS: usize = 40;
const ROWS: usize = 23;
const TILE_SIZE: u32 = 8;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_index: usize, // Which sprite in the atlas
}

#[derive(Resource)]
pub struct TileMap {
    pub tiles: [[Option<Tile>; COLS]; ROWS],
    pub entities: [[Option<Entity>; COLS]; ROWS],
	pub layout: Handle<TextureAtlasLayout>,
}

impl TileMap {
    pub fn new(layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            tiles: [[None; COLS]; ROWS],
            entities: [[None; COLS]; ROWS],
			layout,
        }
    }

    pub fn get_entity_at(&self, x: usize, y: usize) -> Option<Entity> {
        if (x >= COLS) || ( y >= ROWS) {
            None
        } else {
            self.entities[y][x]
        }
    }

    pub fn getTile(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles[y][x]
    }
    
    pub fn get(&self, x: usize, y: usize) -> Option<Entity> {
        self.entities[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Option<Tile>) {
        self.tiles[y][x] = tile;
    }

    pub fn to_map_coords(&self, pos: Vec2) -> Vec2 {
        let x = pos.x / TILE_SIZE as f32;
        let y = pos.y / TILE_SIZE as f32;

        Vec2::new(x,y)
    }
}
pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_map, spawn_tiles).chain());
		//app.add_systems(Update, update_tiles);
    }
}

pub fn setup_map(
	mut commands: Commands,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	let layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 4, 3, None, None);
    let h_layout = texture_atlas_layouts.add(layout);
	let mut tilemap = TileMap::new(h_layout);

    let mut rng = rand::thread_rng();

    for y in 0..ROWS {
        for x in 0..COLS {
            //if (x + y) % 3 == 0 {
                tilemap.set(x, y, Some(Tile { tile_index: rng.gen_range(0..12) }));
            //}
        }
    }

	commands.insert_resource(tilemap);
}

pub fn spawn_tiles(
    mut commands: Commands,
    mut tilemap: ResMut<TileMap>,
    asset_server: Res<AssetServer>,
) {
	let texture = asset_server.load("block.png");

    for y in 0..ROWS {
        for x in 0..COLS {
            //println!("tilemap[{}][{}] = {:?}", x, y, tilemap.tiles[y][x]);
            if let Some(tile) = tilemap.getTile(x, y) {
                let world_pos = Vec3::new(
                    x as f32 * TILE_SIZE as f32,
                    (y as f32-(ROWS as f32)+1.) * TILE_SIZE as f32,
                    0.0,
                );

				let mut sprite = Sprite {
					image: texture.clone(),
					texture_atlas: Some(TextureAtlas {
						layout: tilemap.layout.clone(),
						index: tile.tile_index,
					}),
					..Default::default()
				};

				sprite.anchor = Anchor::TopLeft;

                // print the worldpos
				let e = commands.spawn((
					sprite,
					Transform::from_xyz(world_pos.x, world_pos.y, world_pos.z),
					PIXEL_PERFECT_LAYERS,
				)).id();

				tilemap.entities[y][x] = Some(e);
            }
        }
    }
}

fn update_tiles(
    //camera_query: Single<(&Camera, &GlobalTransform)>,
    tilemap: ResMut<TileMap>,
    mut sprites: Query<&mut Sprite, Without<PixelatedCanvas>>,
    window: Single<&Window>,
    canvas_query: Query<(&Transform, &Sprite), With<PixelatedCanvas>>,
) {
    if let Some(cursor_pos) = window.cursor_position()
    {
        let (canvas_tf, _canvas_sprite) = canvas_query.single().unwrap();

        // cursor_pos is top-left origin
        let win_w = window.width() as f32;
        let win_h = window.height() as f32;

        // 1) window (top-left origin) -> world (center origin)
        let world_x = cursor_pos.x - win_w * 0.5;
        let world_y =  win_h * 0.5 - cursor_pos.y;

        // 2) world -> canvas-local (canvas_transform.translation is the canvas center)
        let local_x = world_x - canvas_tf.translation.x;
        let local_y = world_y - canvas_tf.translation.y;

        // 3) undo canvas scale (assumes uniform scale)
        let scale = canvas_tf.scale.x.max(1e-6);
        let sprite_local_x = local_x / scale;
        let sprite_local_y = local_y / scale;

        // 4) sprite-local -> canvas pixel coords (top-left origin)
        let canvas_x = sprite_local_x + (RES_WIDTH as f32) * 0.5;
        let canvas_y = sprite_local_y + (RES_HEIGHT as f32) * 0.5;

        // 5) tile indices
        let tile_x = ((canvas_x) / TILE_SIZE as f32).floor() as i32;
        let tile_y = ((canvas_y + 0.5 * TILE_SIZE as f32) / TILE_SIZE as f32).floor() as i32;

        // --- Step 5: Clamp and update ---
        if tile_x >= 0 && tile_x < COLS as i32 && tile_y >= 0 && tile_y < ROWS as i32 {
            if let Some(ent) = tilemap.get_entity_at(tile_x as usize, tile_y as usize) {
                if let Ok(mut sprite) = sprites.get_mut(ent) {
                    if let Some(at) = &mut sprite.texture_atlas {
                        // Toggle tile index
                        at.index = 1;
                    }
                }
            }
        }
    }
}