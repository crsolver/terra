use bevy::{color::palettes::css::{GREEN, RED, WHITE}, prelude::*, sprite::Anchor};
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

use crate::game::{tilemap::{Tile, TileMap, COLS, ROWS, TCOLS, TILE_SIZE, TROWS}, PixelatedCanvas, PIXEL_PERFECT_LAYERS, RES_HEIGHT, RES_WIDTH};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
		app.add_systems(Update, (update_player,move_world ).chain());
    }
}

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyRoundGizmos {}

#[derive(Component)]
pub struct Player {
	pub speed: f32,
	pub remainder: Vec2,
	pub velocity: Vec2,
	pub gravity: f32 ,
	pub on_ground: bool,
	pub inside: bool,
	pub was_inside: bool,
}

fn setup(
	mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
	let mut sprite = Sprite::from_image(asset_server.load("player.png"));
	sprite.anchor = Anchor::TopLeft;
	commands.spawn((
		Player{speed:100.0, remainder: Vec2::ZERO, gravity: 90., velocity: Vec2::ZERO, on_ground: false,  inside: true, was_inside: true},
		sprite,
		Transform::from_xyz(200., -100.0, 0.0),
        PIXEL_PERFECT_LAYERS,
	));
}

fn update_player(
    tilemap: ResMut<TileMap>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
) {
    let (mut player, mut transform) = player_query.single_mut().unwrap();
	let mut player_pos = transform.translation;
	let mut offset = Vec2::ZERO;
	let mut dir = Vec2::ZERO;

	if keyboard_input.pressed(KeyCode::KeyD) {
		dir.x = 1.;
		offset.x = 8.;
    }
	if keyboard_input.pressed(KeyCode::KeyA) {
		dir.x = -1.;
    }

	player.velocity.x = dir.x * player.speed;
	//player.velocity.y = dir.y * player.speed;
	let dt = time.delta_secs();
	// --- Gravity ---
    let gravity = -1000.0; // downward acceleration
	
    player.velocity.y += gravity * dt;

	 // --- Jump --- 
    if  player.on_ground && keyboard_input.just_pressed(KeyCode::Space) {
        player.velocity.y = 300.0; // jump strength
		player.on_ground = false;
		println!("_______________________________________________________")
    }

	// --- Apply velocity to remainder ---
	let p_vel = player.velocity * dt;
    player.remainder += p_vel;
    let mut mov = player.remainder.round();
	
	let signx = mov.x.signum();
	let signy = mov.y.signum();
	
	if signy < 0. { 
		offset.y = -8.;
	}

	// right
	// |-----x
	// |
	// |_____x

	if mov.x != 0. {
		player.remainder.x -= mov.x;
		//draw_point(&mut gizmos,Vec3::new(player_pos.x + signx + offset.x, player_pos.y + signy, 0.));
		//draw_point(&mut gizmos,Vec3::new(player_pos.x + signx + offset.x, player_pos.y + signy + -8., 0.));
		while mov.x != 0. {
			if !tilemap.collide_at(Vec2::new(player_pos.x + signx + offset.x , player_pos.y)) &&
				!tilemap.collide_at(Vec2::new(player_pos.x + signx + offset.x, player_pos.y + -8.))
			{
				player_pos.x += signx;
				mov.x -= signx;
			} else {
				player.velocity.x = 0.0;
				break
			}
		}
	}

	
	
	if mov.y != 0. {
		player.remainder.y -= mov.y;
		//draw_point_red(&mut gizmos,Vec3::new(player_pos.x, player_pos.y + signy + offset.y, 0.));
		//draw_point_red(&mut gizmos,Vec3::new(player_pos.x + 8., player_pos.y + signy + offset.y, 0.));
		while mov.y != 0. {
			if  (player.velocity.x < 0. && player_pos.x < 0.) || (player.velocity.x>0. && player_pos.x >= (TCOLS as f32*TILE_SIZE as f32)) {
				break;
			}
			if !tilemap.collide_at(Vec2::new(player_pos.x, player_pos.y + signy + offset.y)) &&
				!tilemap.collide_at(Vec2::new(player_pos.x + 8., player_pos.y + signy + offset.y))
			{
				player_pos.y += signy;
				mov.y -= signy;
			} else {
                if signy < 0.0 { // Collision in Y
                    player.on_ground = true;
                }
                player.velocity.y = 0.0;
                break;
			}
		}
	}

	transform.translation = player_pos;
}

pub fn draw_point(gizmos: &mut Gizmos, pos: Vec3) {
	gizmos.rect_2d(    
		Isometry2d::new(Vec2::new(pos.x, pos.y), Rot2::radians(0.0)), 
		Vec2::splat(1.), 
		GREEN
	);
}

pub fn draw_point_red(gizmos: &mut Gizmos, pos: Vec3) {
	gizmos.rect_2d(    
		Isometry2d::new(Vec2::new(pos.x, pos.y), Rot2::radians(0.0)), 
		Vec2::splat(1.), 
		RED	
	);
}

pub fn move_world(
    mut commands: Commands,
    mut tilemap: ResMut<TileMap>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    asset_server: Res<AssetServer>,
	mut gizmos: Gizmos,
) {
	let mut dir = Vec2::ZERO;

    let (mut transform, mut player) = player_query.single_mut().unwrap();

    let mut player_pos = transform.translation;

    let chunk_width = 39.0 * TILE_SIZE as f32;
    let chunk_height = 22.0 * TILE_SIZE as f32;
    let player_width = 8.0; // adjust to your sprite width / 2

    if player.inside {
        if  player.velocity.x > 0. && player_pos.x + player_width >= chunk_width { // no conflicto
            dir.x = 1.;
            transform.translation.x -= chunk_width;
        } else if player.velocity.x < 0. && player_pos.x < 8. { // correct
            dir.x = -1.;
            transform.translation.x += chunk_width;
        } 
		if  player.velocity.y > 0. && player_pos.y > -8. { // no conflicto
            dir.y = -1.;
            transform.translation.y -= chunk_height;
        } else if player.velocity.y < 0. && (player_pos.y -8.) <= -chunk_height -1. { // correct
            dir.y = 1.;
            transform.translation.y += chunk_height;
        }
    } else {
        if player_pos.x >= chunk_width + 8. { // derecha
            dir.x = 1.;
            transform.translation.x -= chunk_width;
        } else if player_pos.x + player_width < 0. { // no confilto
            dir.x = -1.;
            transform.translation.x += chunk_width;
        }

		if player_pos.y - 8. > 0. { // up
            dir.y = -1.;
            transform.translation.y -= chunk_height;
        } else if player_pos.y <= -chunk_height - 8. { // no confilto
            dir.y = 1.;
            transform.translation.y += chunk_height;
        }
    }

    player_pos = transform.translation;
    player.inside = player_pos.x >= 8. && player_pos.x < 39.0 * TILE_SIZE as f32 &&
					player_pos.y <= -8. && player_pos.y > -22.0 * TILE_SIZE as f32;

    /*if player.inside {
		draw_point(&mut gizmos,Vec3::new(player_pos.x, player_pos.y, 0.));
    } else {
		draw_point_red(&mut gizmos,Vec3::new(player_pos.x, player_pos.y, 0.));
    }*/
    
    if dir.x == 0. && dir.y == 0. {
        return;
    }

    tilemap.position += dir;

    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(1);
	let texture = asset_server.load("block.png");

    let gx1 = (tilemap.position.x * 40. -1.) as f64 * 0.1;
    let gx2 = (tilemap.position.x * 40. + COLS as f32) as f64 * 0.1;

    for y in -1..=(ROWS as i32) {
        for x in -1..=(COLS as i32) {
            let maybe_ent = tilemap.entities[(y+1)as usize][(x+1) as usize];
            if let Some(ent) = maybe_ent {
                commands.entity(ent).despawn();
                tilemap.entities[(y+1) as usize][(x+1) as usize] = None;
            }

            let gx = (tilemap.position.x * 39. + x as f32) as f64 * 0.1;
            let gy = (tilemap.position.y * 22. + y as f32) as f64 * 0.1;
            let val = perlin.get([gx, gy, 0.1]);

            if val > 0.2 {
                let tile = Tile { tile_index: rng.gen_range(0..12) };
                tilemap.set((x+1) as usize, (y+1)as usize, Some(tile));
                let world_pos = Vec3::new(
                    (x+1) as f32 * TILE_SIZE as f32,
                    (((y+1) as f32) * TILE_SIZE as f32) * -1.0,
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

                tilemap.entities[(y + 1) as usize][(x + 1) as usize] = Some(e);
            } else {
                tilemap.set((x+1)as usize,(y+1) as usize, None);
            }
        }
    }
}
