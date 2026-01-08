// use crate::base_render::Quad;
use crate::{app_manager::mesh::Mesh, voxels::base_voxel::BlockID};

pub const CHUNKSIZE: usize = 16;
pub const CHUNKLEN: usize = CHUNKSIZE*CHUNKSIZE*CHUNKSIZE;

#[repr(u8)]
#[derive(Copy,Clone)]
pub enum ChunkState {
    Invalid,
    Loading,
    MeshDirty,
    Valid,
}
#[derive(Clone)]
pub struct Chunk {
    pub mesh_cache: Mesh,
    pub data: [BlockID; CHUNKLEN],
    pub state: ChunkState,
}

impl Default for Chunk {
    fn default() -> Self {
        return Self {
            mesh_cache: Mesh::new(),
            data: [BlockID::Stone; CHUNKLEN],
            state: ChunkState::MeshDirty, // this is bad fix fix fix
        }
    }
}