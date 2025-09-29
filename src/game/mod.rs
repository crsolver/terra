use bevy::{
    color::palettes::css::BLACK, prelude::*, render::{
        camera::{RenderTarget, Viewport},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    }, sprite::Anchor, window::WindowResized
};

use crate::game::{player::PlayerPlugin, tilemap::{setup_map, spawn_tiles, TileMapPlugin}};

mod tilemap;
mod player;

const RES_WIDTH: u32 = 320;
const RES_HEIGHT: u32 = 180;
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
        app.add_systems(Startup, setup_camera);
        app.add_plugins(TileMapPlugin);
        app.add_systems(Update, scale_canvas_on_resize);
        app.add_plugins(PlayerPlugin);
    }
}

#[derive(Component)]
struct PixelatedCanvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct OuterCamera;



/// Spawns a capsule mesh on the pixel-perfect layer.
fn setup_mesh(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
	let mut sprite = Sprite::from_image(asset_server.load("block.png"));
	sprite.anchor = Anchor::TopLeft;

	commands.spawn((
		sprite,
		Transform::from_xyz(0., 0., 0.),
		HIGH_RES_LAYERS,
	));
}

fn setup_mesh2(
    mut commands: Commands,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
	let layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 2, 1, None, None);
    let h_layout = texture_atlas_layouts.add(layout);
	let texture = asset_server.load("block.png");

    let mut sprite = Sprite {
        image: texture.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: h_layout.clone(),
            index: 0,
        }),
        ..Default::default()
    };

    sprite.anchor = Anchor::TopLeft;

    commands.spawn((
        sprite,
        Transform::from_xyz(0., 0., 0.),
        PIXEL_PERFECT_LAYERS,
    ));
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // This Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // Fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    let half_width = (RES_WIDTH as f32) / 2.0;
    let half_height = (RES_HEIGHT as f32) / 2.0;

    // This camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2d,
        Camera {
            // Render before the "main pass" camera
            order: -1,
            target: RenderTarget::Image(image_handle.clone().into()),
            clear_color: ClearColorConfig::Custom(BLACK.into()),
            ..default()
        },
        Transform::from_xyz(half_width, -half_height, 0.0), // shift camera
        Msaa::Off,
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    // Spawn the canvas
    commands.spawn((
        Sprite::from_image(image_handle), 
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::splat(4.0), // <-- SCALE FACTOR (integer to keep pixel perfect)
            ..default()
        },
        PixelatedCanvas, 
        HIGH_RES_LAYERS
    ));

    // The "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2d, Msaa::Off, OuterCamera, HIGH_RES_LAYERS));
}

fn scale_canvas_on_resize(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut Transform, With<PixelatedCanvas>>,
) {
    for event in resize_events.read() {
        let h_scale = (event.width / RES_WIDTH as f32).floor();
        let v_scale = (event.height / RES_HEIGHT as f32).floor();
        let scale = h_scale.min(v_scale).max(1.0); // at least 1
        for mut transform in &mut query {
            transform.scale = Vec3::splat(scale);
        }
    }
}