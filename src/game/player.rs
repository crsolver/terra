use bevy::{prelude::*, sprite::Anchor};

use crate::game::{tilemap::TileMap, PIXEL_PERFECT_LAYERS};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
		app.add_systems(Update, update_player);
    }
}

#[derive(Component)]
struct Player {
	pub speed: f32,
	pub remainder: Vec2 
}

fn setup(
	mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
	let mut sprite = Sprite::from_image(asset_server.load("player.png"));
	sprite.anchor = Anchor::TopLeft;
	commands.spawn((
		Player{speed:100.0, remainder: Vec2::ZERO},
		sprite,
		Transform::from_xyz(100., -100.0, 0.0),
        PIXEL_PERFECT_LAYERS,
	));
}

fn update_player(
    tilemap: ResMut<TileMap>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut().unwrap();
	let mut player_pos = transform.translation;
	let mut amount = Vec2::ZERO;
	let mut offset = Vec2::ZERO;

	if keyboard_input.pressed(KeyCode::KeyD) {
		amount.x = player.speed * time.delta_secs();
		offset.x = 8.;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
		amount.x = -player.speed * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::KeyW) {
		amount.y = player.speed * time.delta_secs();
		offset.y = 8.;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
		amount.y = -player.speed * time.delta_secs();
    } else {
		return;
	}

	player.remainder.x += amount.x;
	let mut mov = player.remainder.x.round();

	if mov != 0. {
		player.remainder.x -= mov;
		let sign = mov.signum();
		while mov != 0. {
			if !tilemap.collide_at(Vec2::new(player_pos.x + sign + offset.x, player_pos.y)) {
				player_pos.x += sign;
				mov -= sign;
			} else {
				break
			}
		}
	}

	player.remainder.y += amount.y;
	let mut mov = player.remainder.y.round();

	if mov != 0. {
		player.remainder.y -= mov;
		let sign = mov.signum();
		while mov != 0. {
			if !tilemap.collide_at(Vec2::new(player_pos.x + sign + offset.x, player_pos.y + offset.y)) {
				player_pos.y += sign;
				mov -= sign;
			} else {
				break
			}
		}
	}

	transform.translation = player_pos;
}