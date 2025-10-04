use bevy::{color::palettes::css::{GREEN, RED, YELLOW}, prelude::*};

use crate::game::{chunk_loader::{debug_chunk, CHUNK_COLS, CHUNK_ROWS, TILE_SIZE}, screen::{window_to_world, world_to_high_res_canvas, PixelatedCanvas, HIGH_RES_LAYERS, RES_HEIGHT, RES_WIDTH}, PixelGizmos};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
		app.add_systems(Update, debug_inside_outside.after(debug_chunk));
    }
}

#[derive(Component)]
pub struct CursorLabel;

#[derive(Component)]
pub struct Player {
	pub inside: bool
}

fn setup(
	mut commands: Commands,
	assets: ResMut<AssetServer>
) {
	let text_font = TextFont {
        font: assets.load("MedodicaRegular.otf"),
        font_size: 40.0,
        ..default()
    };

	commands.spawn((
		Text2d::new(""),
		text_font,
		Transform::from_xyz(0.,0.,0.),
		CursorLabel,
		HIGH_RES_LAYERS
	));
}

fn is_inside(mut pos: Vec2) -> bool {
	let width = CHUNK_COLS as f32 * TILE_SIZE as f32;
	let height = 20. as f32 * TILE_SIZE as f32;
	let offset = Vec2::new(width, height) * 0.5;
	pos.x += offset.x;
	pos.y -= offset.y + 2.;
	pos.x > 0. && pos.x <= width as f32 &&
		pos.y < -8. && pos.y >= -height as f32
}

pub fn debug_inside_outside(
	window: Single<&Window>,
	canvas: Single<&Transform, (With<PixelatedCanvas>, Without<CursorLabel>)>,
	mut gizmos: Gizmos<PixelGizmos>,
	mut gizmos_h: Gizmos,
	mut query: Query<(&mut Text2d, &mut Transform), With<CursorLabel>>,
) {
	let (mut text2d, mut transform) = query.single_mut().unwrap();

	if let Some(cursor_pos) = window.cursor_position() {
		let pos = window_to_world(&window, canvas.scale.xy(), cursor_pos);
		if is_inside(pos) {
			draw_point_green(&mut gizmos, Vec3::new(pos.x, pos.y, 0.));
		} else {
			draw_point(&mut gizmos, Vec3::new(pos.x, pos.y, 0.));
		}
		let h_pos = world_to_high_res_canvas(&window, canvas.scale.xy(), pos);
		transform.translation = Vec3::new(h_pos.x, h_pos.y+10., 0.);
		text2d.0 = pos.to_string();
	}
}

pub fn draw_point<T: GizmoConfigGroup>(gizmos: &mut Gizmos<T>, pos: Vec3) {
	gizmos.rect_2d(    
		Isometry2d::new(Vec2::new(pos.x, pos.y), Rot2::radians(0.0)), 
		Vec2::splat(1.), 
		RED	
	);
}

pub fn draw_point_green<T: GizmoConfigGroup>(gizmos: &mut Gizmos<T>, pos: Vec3) {
	gizmos.rect_2d(    
		Isometry2d::new(Vec2::new(pos.x, pos.y), Rot2::radians(0.0)), 
		Vec2::splat(1.), 
		GREEN
	);
}