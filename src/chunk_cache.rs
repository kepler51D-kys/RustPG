use crate::v3;
use crate::base_voxel::{CHUNKLEN, CHUNKSIZE, Chunk, WORLDSIZE, topQuad, Quad};
use std::collections::HashMap;

pub struct Manager {
    chunk_cache: HashMap<u128,Chunk>,
    cache_size: usize,
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
    pub fn render_chunk(&self, index: v3::V3) {
        // todo
    }
    pub fn gen_cache(&mut self, chunk_index: v3::V3) {
        let working_chunk: &mut Chunk;
        match self.get_mut_chunk(chunk_index) {
            Some(val) => working_chunk = val,
            None => return
        };
        working_chunk.mesh_cache.clear();
        let mut block_index: v3::V3 = v3::V3 {x: CHUNKSIZE as u32 -1,y: CHUNKSIZE as u32 -1,z: CHUNKSIZE as u32 -1};
        while block_index.z < CHUNKSIZE as u32 {
            if working_chunk.data[block_index.toBlockKey()] != 0 {
                if self.topNeighbourSolid(chunk_index, block_index) {
                    let mut newQuad: Quad = topQuad;
                    for i in 0..4 {
                        newQuad.data[i] = newQuad.data[i] + block_index + (chunk_index*16);
                    }
                }
            }

            block_index.y -= (block_index.x >= CHUNKSIZE as u32) as u32;
            block_index.x %= CHUNKSIZE as u32;
            block_index.z -= (block_index.y >= CHUNKSIZE as u32) as u32;
            block_index.y %= CHUNKSIZE as u32;
            block_index.x -= 1;
        }
    }
    fn topNeighbourSolid(&self, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
        block_index.y += 1;
        if (block_index.y >= CHUNKSIZE as u32) {
            block_index.y = 0;
            chunk_index.y += 1;
        }
        if (chunk_index.y >= WORLDSIZE) {
            return false;
        }
        else {
            match self.chunk_cache.get(&chunk_index.toKey()) {
                Some(val) => {return val.data[block_index.toBlockKey()] != 0},
                None => return false
            }
        }
    }
    fn bottomNeighbourSolid(&self, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
        block_index.y -= 1;
        if (block_index.y >= CHUNKSIZE as u32) {
            block_index.y = CHUNKSIZE as u32 - 1;
            chunk_index.y -= 1;
        }
        if (chunk_index.y >= WORLDSIZE) {
            return false;
        }
        else {
            match self.chunk_cache.get(&chunk_index.toKey()) {
                Some(val) => {return val.data[block_index.toBlockKey()] != 0},
                None => return false
            }
        }
    }
    fn rightNeighbourSolid(&self, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
        block_index.x += 1;
        if (block_index.x >= CHUNKSIZE as u32) {
            block_index.x = 0;
            chunk_index.x += 1;
        }
        if (chunk_index.x >= WORLDSIZE) {
            return false;
        }
        else {
            match self.chunk_cache.get(&chunk_index.toKey()) {
                Some(val) => {return val.data[block_index.toBlockKey()] != 0},
                None => return false
            }
        }
    }
    fn leftNeighbourSolid(&self, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
        block_index.x -= 1;
        if (block_index.x >= CHUNKSIZE as u32) {
            block_index.x = CHUNKSIZE as u32 - 1;
            chunk_index.x -= 1;
        }
        if (chunk_index.x >= WORLDSIZE) {
            return false;
        }
        else {
            match self.chunk_cache.get(&chunk_index.toKey()) {
                Some(val) => {return val.data[block_index.toBlockKey()] != 0},
                None => return false
            }
        }
    }
    fn frontNeighbourSolid(&self, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
        block_index.z += 1;
        if (block_index.z >= CHUNKSIZE as u32) {
            block_index.z = 0;
            chunk_index.z += 1;
        }
        if (chunk_index.z >= WORLDSIZE) {
            return false;
        }
        else {
            match self.chunk_cache.get(&chunk_index.toKey()) {
                Some(val) => {return val.data[block_index.toBlockKey()] != 0},
                None => return false
            }
        }
    }
    fn backNeighbourSolid(&self, mut chunk_index: v3::V3, mut block_index: v3::V3) -> bool {
        block_index.z -= 1;
        if (block_index.z >= CHUNKSIZE as u32) {
            block_index.z = CHUNKSIZE as u32 - 1;
            chunk_index.z -= 1;
        }
        if (chunk_index.z >= WORLDSIZE) {
            return false;
        }
        else {
            match self.chunk_cache.get(&chunk_index.toKey()) {
                Some(val) => {return val.data[block_index.toBlockKey()] != 0},
                None => return false
            }
        }
    }
}