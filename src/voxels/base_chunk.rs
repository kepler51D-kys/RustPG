// use crate::base_render::Quad;
use crate::{app_manager::mesh::Mesh, voxels::base_voxel::BlockID};

pub const CHUNKSIZE: usize = 16;
pub const CHUNKLEN: usize = CHUNKSIZE*CHUNKSIZE*CHUNKSIZE;

#[repr(u8)]
pub enum ChunkState {
    Invalid,
    Loading,
    MeshDirty,
    Valid,
}

pub struct Chunk {
    pub mesh_cache: Mesh,
    pub mesh_length: u32,
    pub data: [BlockID; CHUNKLEN],
    pub state: ChunkState,
}

impl Default for Chunk {
    fn default() -> Self {
        return Self {
            mesh_length: 0,
            mesh_cache: Mesh::new(),
            data: [BlockID::Stone; CHUNKLEN],
            state: ChunkState::Invalid,
        }
    }
}