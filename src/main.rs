#![allow(dead_code)]
mod base_chunk;
mod base_render;
mod base_voxel;
mod chunk_cache;
mod v3;
mod world_file;
mod world;

use bevy::camera::Camera3d;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy::{prelude::*};

fn setup(
    mut commands: Commands
) {
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Transform::from_xyz(-20.0, 10.0, -20.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection::default()),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 100.0,
        affects_lightmapped_meshes: true,
    });
}
fn main() {
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
            }
            ),
            ..default()
        }))
        // .add_plugins((
        //     FrameTimeDiagnosticsPlugin::default(),
        //     LogDiagnosticsPlugin::default(),
        // ))
        .add_systems(Startup, setup)
        .run();
}