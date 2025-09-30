use bevy::{color::palettes::css::{GREEN, RED}, prelude::*, sprite::Anchor};

use crate::game::{tilemap::TileMap, PIXEL_PERFECT_LAYERS};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
		app.add_systems(Update, update_player);
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
}

fn setup(
	mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
	let mut sprite = Sprite::from_image(asset_server.load("player.png"));
	sprite.anchor = Anchor::TopLeft;
	commands.spawn((
		Player{speed:100.0, remainder: Vec2::ZERO, gravity: 90., velocity: Vec2::ZERO, on_ground: false},
		sprite,
		Transform::from_xyz(400., -100.0, 0.0),
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
	/*if keyboard_input.pressed(KeyCode::KeyW) {
		amount.y = player.speed * time.delta_secs();
		offset.y = 8.;
    }*/

	player.velocity.x = dir.x * player.speed;
	let dt = time.delta_secs();
	// --- Gravity ---
    let gravity = -1000.0; // downward acceleration
	
    player.velocity.y += gravity * dt;

	 // --- Jump ---
    if player.on_ground && keyboard_input.just_pressed(KeyCode::Space) {
        player.velocity.y = 300.0; // jump strength
		player.on_ground = false;
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
			if !tilemap.collide_at(Vec2::new(player_pos.x + signx + offset.x, player_pos.y)) &&
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