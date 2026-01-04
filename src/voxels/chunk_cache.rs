use glam::{UVec3,Vec3};

use crate::app_manager::mesh::{Mesh, Vertex};
use crate::app_manager::window::State;
use crate::voxels::base_voxel::{BlockID, WORLDSIZE};
use crate::voxels::base_chunk::{CHUNKSIZE, Chunk,ChunkState};
use crate::voxels::base_render::{BACK_QUAD, BOTTOM_QUAD, FRONT_QUAD, LEFT_QUAD, Quad, RIGHT_QUAD, TOP_QUAD};
use crate::v3::get_block_index;
use std::collections::HashMap;

const COLOUR: Vec3 = Vec3 {x:1.0,y:0.0,z:1.0};

pub struct ChunkCacheManager {
    chunk_cache: HashMap<UVec3,Chunk>,
    cache_size: usize,
}
impl ChunkCacheManager {
    pub fn new(cache_size: usize) -> Self {
        Self {
            chunk_cache: HashMap::with_capacity(cache_size),
            cache_size,
        }
    }
    pub fn chunk_present(&self, index: &UVec3) -> bool {
        self.chunk_cache.contains_key(index)
    }
    pub fn get_mut_chunk(&mut self, index: UVec3) -> Option<&mut Chunk> {
        return self.chunk_cache.get_mut(&index);
    }
    pub fn get_chunk(&self, index: UVec3) -> Option<&Chunk> {
        return self.chunk_cache.get(&index);
    }
    pub fn add_chunk(&mut self, index: UVec3, chunk: Chunk) {
        self.chunk_cache.insert(index, chunk);
    }
    pub fn remove_chunk(&mut self, index: UVec3) {
        self.chunk_cache.remove(&index);
    }
    pub fn render_chunk(&mut self, index: UVec3, state: &mut State) {
        // let chunk: &Chunk = self.get_chunk(index).unwrap();
        let chunk: &Chunk;
        match self.get_chunk(index) {
            Some(val) => {chunk = val;},
            None => {return;}
        }
        println!("{}",chunk.mesh_length);
        match chunk.state {
            ChunkState::Invalid => {return;} // todo handle this
            ChunkState::Loading => {return;}
            ChunkState::MeshDirty => {self.gen_cache(index);}
            ChunkState::Valid => {
                // match &chunk.mesh_cache {
                    // Some(mesh) => {
                        let err = state.render_vertices(&chunk.mesh_cache);
                        match err {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                let size = state.window.inner_size();
                                state.resize(size.width, size.height);
                            }
                            Err(e) => {
                                log::error!("Unable to render {}", e);
                            }
                        }
                    // }
                // }
            }
        }
    }
    fn add_vertices(&self, quad: Quad,indices: &mut Vec<u16>, vertices: &mut Vec<Vertex>) {
        for i in 0..3 {
            vertices.push(quad.data[i]);
        }
        indices.push(1);
        indices.push(3);
        indices.push(0);

        indices.push(3);
        indices.push(2);
        indices.push(0);
    }
    pub fn gen_cache(&mut self, chunk_index: UVec3) {

        let mut vertices:Vec<Vertex> = Vec::new();
        let mut indices:Vec<u16> = Vec::new();

        let chunk: &Chunk = self.get_chunk(chunk_index).unwrap();
        // let mut mesh: Vec<Quad> = Vec::new();
        let mut block_index: UVec3 = UVec3 {
            x: CHUNKSIZE as u32 - 1,
            y: CHUNKSIZE as u32 - 1,
            z: CHUNKSIZE as u32 - 1
        };
        let chunk_pos: Vec3 = Vec3 {
            x: chunk_index.x as f32*CHUNKSIZE as f32,
            y: chunk_index.y as f32*CHUNKSIZE as f32,
            z: chunk_index.z as f32*CHUNKSIZE as f32,
        };
        // println!("hello");
        for x in 0..CHUNKSIZE {
            for y in 0..CHUNKSIZE {
                for z in 0..CHUNKSIZE {
                    let block_pos: Vec3 = Vec3 {
                        x:x as f32,
                        y:y as f32,
                        z:z as f32,
                    };
                    let skip: bool = chunk.data[get_block_index(block_index)] == BlockID::Air;
                    if !skip {
                        if top_neighbour_solid(&self.chunk_cache,chunk_index, block_index) {
                            let mut new_quad: Quad = TOP_QUAD;
                            for i in 0..4 {
                                new_quad.data[i] += Vertex::from(block_pos) + Vertex::from(chunk_pos);
                            }
                            self.add_vertices(new_quad,&mut indices, &mut vertices);
                        }
                        if bottom_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                            let mut new_quad: Quad = BOTTOM_QUAD;
                            for i in 0..4 {
                                new_quad.data[i] += Vertex::from(block_pos) + Vertex::from(chunk_pos);
                            }
                            self.add_vertices(new_quad, &mut indices, &mut vertices);
                        }
                        if left_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                            let mut new_quad: Quad = LEFT_QUAD;
                            for i in 0..4 {
                                new_quad.data[i] += Vertex::from(block_pos) + Vertex::from(chunk_pos);
                            }
                            self.add_vertices(new_quad, &mut indices, &mut vertices);
                        }
                        if right_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                            let mut new_quad: Quad = RIGHT_QUAD;
                            for i in 0..4 {
                                new_quad.data[i] += Vertex::from(block_pos) + Vertex::from(chunk_pos);
                            }
                            self.add_vertices(new_quad, &mut indices, &mut vertices);
                        }
                        if front_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                            let mut new_quad: Quad = FRONT_QUAD;
                            for i in 0..4 {
                                new_quad.data[i] += Vertex::from(block_pos) + Vertex::from(chunk_pos);
                            }
                            self.add_vertices(new_quad, &mut indices, &mut vertices);
                        }
                        if back_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                            let mut new_quad: Quad = BACK_QUAD;
                            for i in 0..4 {
                                new_quad.data[i] += Vertex::from(block_pos) + Vertex::from(chunk_pos);
                            }
                            self.add_vertices(new_quad, &mut indices, &mut vertices);
                        }
                    }
                    
                    block_index.y -= (block_index.x >= CHUNKSIZE as u32) as u32;
                    block_index.x %= CHUNKSIZE as u32;
                    block_index.z -= (block_index.y >= CHUNKSIZE as u32) as u32;
                    block_index.y %= CHUNKSIZE as u32;
                    block_index.x -= 1;
                }
            }
        }   
        let chunk: &mut Chunk = self.get_mut_chunk(chunk_index).unwrap();
        chunk.mesh_cache = Mesh {
            vertices,
            indices
        };
        chunk.state = ChunkState::Valid;
        
    }
}

fn top_neighbour_solid(chunk_cache: &HashMap<UVec3,Chunk>, mut chunk_index: UVec3, mut block_index: UVec3) -> bool {
    block_index.y += 1;
    if block_index.y >= CHUNKSIZE as u32 {
        block_index.y = 0;
        chunk_index.y += 1;
    }
    if chunk_index.y >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index) {
            Some(val) => {return val.data[get_block_index(block_index)] != BlockID::Air},
            None => return false
        }
    }
}
fn bottom_neighbour_solid(chunk_cache: &HashMap<UVec3,Chunk>, mut chunk_index: UVec3, mut block_index: UVec3) -> bool {
    block_index.y -= 1;
    if block_index.y >= CHUNKSIZE as u32 {
        block_index.y = CHUNKSIZE as u32 - 1;
        chunk_index.y -= 1;
    }
    if chunk_index.y >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index) {
            Some(val) => return val.data[get_block_index(block_index)] != BlockID::Air,
            None => return false
        }
    }
}
fn right_neighbour_solid(chunk_cache: &HashMap<UVec3,Chunk>, mut chunk_index: UVec3, mut block_index: UVec3) -> bool {
    block_index.x += 1;
    if block_index.x >= CHUNKSIZE as u32 {
        block_index.x = 0;
        chunk_index.x += 1;
    }
    if chunk_index.x >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index) {
            Some(val) => return val.data[get_block_index(block_index)] != BlockID::Air,
            None => return false
        }
    }
}
fn left_neighbour_solid(chunk_cache: &HashMap<UVec3,Chunk>, mut chunk_index: UVec3, mut block_index: UVec3) -> bool {
    block_index.x -= 1;
    if block_index.x >= CHUNKSIZE as u32 {
        block_index.x = CHUNKSIZE as u32 - 1;
        chunk_index.x -= 1;
    }
    if chunk_index.x >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index) {
            Some(val) => return val.data[get_block_index(block_index)] != BlockID::Air,
            None => return false
        }
    }
}
fn front_neighbour_solid(chunk_cache: &HashMap<UVec3,Chunk>, mut chunk_index: UVec3, mut block_index: UVec3) -> bool {
    block_index.z += 1;
    if block_index.z >= CHUNKSIZE as u32 {
        block_index.z = 0;
        chunk_index.z += 1;
    }
    if chunk_index.z >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index) {
            Some(val) => return val.data[get_block_index(block_index)] != BlockID::Air,
            None => return false
        }
    }
}
fn back_neighbour_solid(chunk_cache: &HashMap<UVec3,Chunk>, mut chunk_index: UVec3, mut block_index: UVec3) -> bool {
    block_index.z -= 1;
    if block_index.z >= CHUNKSIZE as u32 {
        block_index.z = CHUNKSIZE as u32 - 1;
        chunk_index.z -= 1;
    }
    if chunk_index.z >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index) {
            Some(val) => return val.data[get_block_index(block_index)] != BlockID::Air,
            None => return false
        }
    }
}
// ahh