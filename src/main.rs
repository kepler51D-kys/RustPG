mod v3;
mod base_voxel;
mod base_chunk;
mod base_render;
mod chunk_cache;
mod world_file;

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use crate::base_chunk::CHUNKSIZE;

struct World {
    chunk_cache_manager: chunk_cache::Manager,
    world_file_manager: world_file::Manager,
    render_distance: u16,
}
impl World {
    fn render(&mut self, mut centerCoord: v3::V3) {
        centerCoord /= CHUNKSIZE as u32;
        let start: v3::V3 = v3::V3 {
            x: centerCoord.x-(self.render_distance as u32)/2,
            y: centerCoord.y-(self.render_distance as u32)/2,
            z: centerCoord.z-(self.render_distance as u32)/2,
        };
        let end: v3::V3 = v3::V3 {
            x: centerCoord.x+(self.render_distance as u32)/2,
            y: centerCoord.y+(self.render_distance as u32)/2,
            z: centerCoord.z+(self.render_distance as u32)/2,
        };
        for x in start.x..end.x {
            for y in start.y..end.y {
                for z in start.z..end.z {
                    let offset: v3::V3 = v3::V3 {x:x, y:y, z:z};
                    self.chunk_cache_manager.render_chunk(offset);
                }
            }
        }
    }
}
impl Default for World {
    fn default() -> Self {
        let world_file_manager: world_file::Manager = world_file::Manager::Default();
        return Self {
            chunk_cache_manager: chunk_cache::Manager::Default(),
            world_file_manager: world_file::Manager::init(),
            render_distance: 6
        };
    }
}

fn main() {
    let newWorld: World = World::Default();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "rawr".into(),
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
        .add_systems(Update, systems)
        .run();
}