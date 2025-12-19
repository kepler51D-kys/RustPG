use bevy::asset::RenderAssetUsages;
use bevy::ecs::component::Component;
use bevy::mesh::{Mesh, PrimitiveTopology};

// use crate::base_render::Quad;
use crate::base_voxel::BlockID;

pub const CHUNKSIZE: usize = 16;
pub const CHUNKLEN: usize = CHUNKSIZE*CHUNKSIZE*CHUNKSIZE;
#[derive(Component)]
pub struct Chunk {
    pub mesh_cache: Mesh,
    pub data: [BlockID; CHUNKLEN],
    pub mesh_dirty: bool,
    pub loaded: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        return Self {
            mesh_cache: Mesh::new(PrimitiveTopology::TriangleList,RenderAssetUsages::default()),
            data: [BlockID::Stone; CHUNKLEN],
            mesh_dirty: true,
            loaded: false
        }
    }
}