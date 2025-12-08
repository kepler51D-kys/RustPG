mod v3;
mod base_voxel;
mod chunk_cache;
mod world_file;

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};

struct World {
    chunk_cache_manager: chunk_cache::Manager,
    world_file_manager: world_file::Manager,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "gay".into(),
                resolution: WindowResolution::new(1280, 720),
                resizable: true,
                mode: bevy::window::WindowMode::Windowed,
                position: bevy::window::WindowPosition::Centered(MonitorSelection::Primary),
                decorations: true,
                transparent: false,
                ..default()
            }),
            ..default()
        }))
        .run();
}