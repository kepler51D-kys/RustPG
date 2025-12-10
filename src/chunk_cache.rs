use crate::v3;
use crate::base_voxel::{BlockID, WORLDSIZE};
use crate::base_chunk::{CHUNKSIZE, Chunk};
use crate::base_render::{Quad, back_quad, bottom_quad, front_quad, left_quad, right_quad, top_quad};
use std::collections::HashMap;

pub struct Manager {
    chunk_cache: HashMap<u128,Chunk>,
    cache_size: usize,
}
impl Default for Manager {
    fn default() -> Self {
        return Self {
            chunk_cache: HashMap::new(),
            cache_size: 4*4*4*2
        };
    }
}
impl Manager {
    pub fn new(cache_size: usize) -> Self {
        Self {
            chunk_cache: HashMap::with_capacity(cache_size),
            cache_size,
        }
    }
    pub fn get_mut_chunk(&mut self, index: v3::V3) -> Option<&mut Chunk> {
        return self.chunk_cache.get_mut(&index.toKey());
    }
    pub fn get_chunk(&self, index: v3::V3) -> Option<&Chunk> {
        return self.chunk_cache.get(&index.toKey());
    }
    pub fn add_chunk(&mut self, index: v3::V3, chunk: Chunk) {
        self.chunk_cache.insert(index.toKey(), chunk);
    }
    pub fn remove_chunk(&mut self, index: v3::V3) {
        self.chunk_cache.remove(&index.toKey());
    }
    pub fn render_chunk(&mut self, index: v3::V3) {
        let chunk: &Chunk = self.get_chunk(index).unwrap();
        if !chunk.loaded {
            return;
        }
        if chunk.mesh_dirty {
            self.gen_cache(index);
        }
    }
    pub fn gen_cache(&mut self, chunk_index: v3::V3) {
        let chunk: &Chunk = self.get_chunk(chunk_index).unwrap();
        let mut mesh: Vec<Quad> = Vec::new();
        let mut block_index: v3::V3 = v3::V3 {
            x: CHUNKSIZE as u32 - 1,
            y: CHUNKSIZE as u32 - 1,
            z: CHUNKSIZE as u32 - 1
        };

        while block_index.z < CHUNKSIZE as u32 {
            let skip: bool = chunk.data[block_index.toBlockKey()] == BlockID::Air;
            if !skip {
                if top_neighbour_solid(&self.chunk_cache,chunk_index, block_index) {
                    let mut new_quad: Quad = top_quad;
                    for i in 0..4 {
                        new_quad.data[i] += block_index + (chunk_index*CHUNKSIZE as u32);
                    }
                    mesh.push(new_quad);
                }
                if bottom_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let mut new_quad: Quad = bottom_quad;
                    for i in 0..4 {
                        new_quad.data[i] += block_index + (chunk_index*CHUNKSIZE as u32);
                    }
                    mesh.push(new_quad);
                }
                if left_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let mut new_quad: Quad = left_quad;
                    for i in 0..4 {
                        new_quad.data[i] += block_index + (chunk_index*CHUNKSIZE as u32);
                    }
                    mesh.push(new_quad);
                }
                if right_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let mut new_quad: Quad = right_quad;
                    for i in 0..4 {
                        new_quad.data[i] += block_index + (chunk_index*CHUNKSIZE as u32);
                    }
                    mesh.push(new_quad);
                }
                if front_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let mut new_quad: Quad = front_quad;
                    for i in 0..4 {
                        new_quad.data[i] += block_index + (chunk_index*CHUNKSIZE as u32);
                    }
                    mesh.push(new_quad);
                }
                if back_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let mut new_quad: Quad = back_quad;
                    for i in 0..4 {
                        new_quad.data[i] += block_index + (chunk_index*CHUNKSIZE as u32);
                    }
                    mesh.push(new_quad);
                }
            }
            
            block_index.y -= (block_index.x >= CHUNKSIZE as u32) as u32;
            block_index.x %= CHUNKSIZE as u32;
            block_index.z -= (block_index.y >= CHUNKSIZE as u32) as u32;
            block_index.y %= CHUNKSIZE as u32;
            block_index.x -= 1;
        }
        self.get_mut_chunk(chunk_index).unwrap().mesh_cache = mesh;
    }
}
fn top_neighbour_solid(chunk_cache: &HashMap<u128,Chunk>, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
    block_index.y += 1;
    if block_index.y >= CHUNKSIZE as u32 {
        block_index.y = 0;
        chunk_index.y += 1;
    }
    if chunk_index.y >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index.toKey()) {
            Some(val) => {return val.data[block_index.toBlockKey()] != BlockID::Air},
            None => return false
        }
    }
}
fn bottom_neighbour_solid(chunk_cache: &HashMap<u128,Chunk>, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
    block_index.y -= 1;
    if block_index.y >= CHUNKSIZE as u32 {
        block_index.y = CHUNKSIZE as u32 - 1;
        chunk_index.y -= 1;
    }
    if chunk_index.y >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index.toKey()) {
            Some(val) => return val.data[block_index.toBlockKey()] != BlockID::Air,
            None => return false
        }
    }
}
fn right_neighbour_solid(chunk_cache: &HashMap<u128,Chunk>, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
    block_index.x += 1;
    if block_index.x >= CHUNKSIZE as u32 {
        block_index.x = 0;
        chunk_index.x += 1;
    }
    if chunk_index.x >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index.toKey()) {
            Some(val) => return val.data[block_index.toBlockKey()] != BlockID::Air,
            None => return false
        }
    }
}
fn left_neighbour_solid(chunk_cache: &HashMap<u128,Chunk>, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
    block_index.x -= 1;
    if block_index.x >= CHUNKSIZE as u32 {
        block_index.x = CHUNKSIZE as u32 - 1;
        chunk_index.x -= 1;
    }
    if chunk_index.x >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index.toKey()) {
            Some(val) => return val.data[block_index.toBlockKey()] != BlockID::Air,
            None => return false
        }
    }
}
fn front_neighbour_solid(chunk_cache: &HashMap<u128,Chunk>, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
    block_index.z += 1;
    if block_index.z >= CHUNKSIZE as u32 {
        block_index.z = 0;
        chunk_index.z += 1;
    }
    if chunk_index.z >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index.toKey()) {
            Some(val) => return val.data[block_index.toBlockKey()] != BlockID::Air,
            None => return false
        }
    }
}
fn back_neighbour_solid(chunk_cache: &HashMap<u128,Chunk>, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
    block_index.z -= 1;
    if block_index.z >= CHUNKSIZE as u32 {
        block_index.z = CHUNKSIZE as u32 - 1;
        chunk_index.z -= 1;
    }
    if chunk_index.z >= WORLDSIZE {
        return false;
    }
    else {
        match chunk_cache.get(&chunk_index.toKey()) {
            Some(val) => return val.data[block_index.toBlockKey()] != BlockID::Air,
            None => return false
        }
    }
}
