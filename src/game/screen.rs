use bevy::{
	camera::{visibility::RenderLayers, RenderTarget}, color::palettes::css::{BLACK, GRAY}, prelude::*, render::render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        }, window::{PrimaryWindow, WindowResized}
};

pub const RES_WIDTH: u32 = 320;
pub const RES_HEIGHT: u32 = 180;
pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);
pub const HIGH_RES_LAYERS2: RenderLayers = RenderLayers::layer(2);

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, scale_canvas_on_resize);
    }
}

#[derive(Component)]
pub struct PixelatedCanvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct OuterCamera;

#[derive(Component)]
struct EguiCamera;

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>, asset_server: Res<AssetServer>) {
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

    /* */
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
        Transform::from_xyz(0., 0., 0.), // shift camera
        Msaa::Off,
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    // The "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((
        Camera2d, 
        Camera {
            order: 0,
            ..default()
        },
        Msaa::Off, 
        Transform::from_xyz(0.,0.,0.),
        OuterCamera, 
        HIGH_RES_LAYERS,
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
        HIGH_RES_LAYERS,
    ));
}

fn scale_canvas_on_resize(
    mut resize_events: MessageReader<WindowResized>,
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

pub fn window_to_world(window: &Window, scale: Vec2, win_coords: Vec2) -> Vec2 {
	//win_coords.y *= -1.;
	let local = win_coords - window.size() * 0.5;
	let mut canvas = local / scale.x.max(1e-6);
    canvas.y *= -1.;
    canvas
}

pub fn world_to_window(window: &Window, scale: Vec2, world_coords: Vec2) -> Vec2 {
    let mut local = world_coords * scale.x.max(1e-6);
    local.y *= -1.;
    local + window.size() * 0.5
}

pub fn world_to_high_res_canvas(window: &Window, scale: Vec2, world_coords: Vec2) -> Vec2 {
    world_coords * scale.x.max(1e-6)
}

