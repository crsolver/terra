use bevy::prelude::*;
use crate::game::{chunk_loader::ChunkLoaderPlugin, player::PlayerPlugin, screen::{ScreenPlugin, HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS}};

mod screen;
mod chunk_loader;
mod tilemap;
mod player;

pub struct GamePlugin;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct PixelGizmos;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
        app.insert_gizmo_config(
            DefaultGizmoConfigGroup,
            GizmoConfig {
                render_layers: HIGH_RES_LAYERS,  // Matches your camera's layers
                ..default()
            }
        );
        app.insert_gizmo_config(
            PixelGizmos,
            GizmoConfig {
                render_layers: PIXEL_PERFECT_LAYERS,  // Matches your camera's layers
                ..default()
            }
        );
        app.add_plugins(ScreenPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(ChunkLoaderPlugin);
    }
}