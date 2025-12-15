#![allow(dead_code)]
use bevy::camera::Camera3d;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy_voxel_world::prelude::{VoxelWorldCamera, VoxelWorldConfig, VoxelWorldPlugin, WorldVoxel};


#[derive(Resource, Clone, Default)]
struct MainWorld;

impl VoxelWorldConfig for MainWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        25
    }
    // fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
    //     Box::new(|_, _, _| Box::new(|_, _| WorldVoxel::Solid(0)))
    // }
}
fn setup(
    mut commands: Commands,
    mut voxel_world: bevy_voxel_world::prelude::VoxelWorld<MainWorld>
) {
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Transform::from_xyz(-20.0, 10.0, -20.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection::default()),
        VoxelWorldCamera::<MainWorld>::default(),
    ));

    // commands.insert_resource(AmbientLight {
    //     color: Color::srgb(0.98, 0.95, 0.82),
    //     brightness: 100.0,
    //     affects_lightmapped_meshes: true,
    // });

    for x in -8..8 {
        for y in -8..8 {
            for z in -8..8 {
                voxel_world.set_voxel(IVec3 { x: x, y: y, z: z}, WorldVoxel::Solid(0));
            }
        }
    }
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
        .add_plugins(VoxelWorldPlugin::with_config(MainWorld))
        .add_systems(Startup, setup)
        .run();
}