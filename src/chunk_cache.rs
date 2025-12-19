use bevy::asset::RenderAssetUsages;
use bevy::math::{UVec3, Vec3};
use bevy::mesh::{Indices, Mesh, PrimitiveTopology};

use crate::base_voxel::{BlockID, WORLDSIZE};
use crate::base_chunk::{CHUNKSIZE, Chunk};
use crate::base_render::{Quad, BACK_QUAD,FRONT_QUAD,LEFT_QUAD,RIGHT_QUAD,TOP_QUAD,BOTTOM_QUAD};
use crate::v3::get_block_index;
use std::collections::HashMap;

pub struct Manager {
    chunk_cache: HashMap<UVec3,Chunk>,
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
    pub fn render_chunk(&mut self, index: UVec3) {
        let chunk: &Chunk = self.get_chunk(index).unwrap();
        if !chunk.loaded {
            return;
        }
        if chunk.mesh_dirty {
            self.gen_cache(index);
        }
    }
    pub fn gen_cache(&mut self, chunk_index: UVec3) {

        let mut vertices:Vec<Vec3> = Vec::new();
        let mut normals:Vec<Vec3> = Vec::new();
        let mut indices:Vec<u32> = Vec::new();

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
        while block_index.z < CHUNKSIZE as u32 {
            let block_pos: Vec3 = Vec3 {
                x: block_index.x as f32,
                y: block_index.y as f32,
                z: block_index.z as f32
            };
            let skip: bool = chunk.data[get_block_index(block_index)] == BlockID::Air;
            if !skip {
                if top_neighbour_solid(&self.chunk_cache,chunk_index, block_index) {
                    let start_val: u32 = vertices.len() as u32;
                    let mut new_quad: Quad = TOP_QUAD;
                    for i in 0..4 {
                        new_quad.data[i] += block_pos + chunk_pos;
                        vertices.push(new_quad.data[i]);
                    }
                    // mesh.push(new_quad);
                    indices.push(start_val);
                    indices.push(start_val+1);
                    indices.push(start_val+3);
                    indices.push(start_val);
                    indices.push(start_val+3);
                    indices.push(start_val+2);
                    
                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                }
                if bottom_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let start_val: u32 = vertices.len() as u32;
                    let mut new_quad: Quad = BOTTOM_QUAD;
                    for i in 0..4 {
                        new_quad.data[i] += block_pos + chunk_pos;
                        vertices.push(new_quad.data[i]);
                    }
                    // mesh.push(new_quad);
                    indices.push(start_val);
                    indices.push(start_val+1);
                    indices.push(start_val+3);
                    indices.push(start_val);
                    indices.push(start_val+3);
                    indices.push(start_val+2);

                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                }
                if left_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let start_val: u32 = vertices.len() as u32;
                    let mut new_quad: Quad = LEFT_QUAD;
                    for i in 0..4 {
                        new_quad.data[i] += block_pos + chunk_pos;
                        vertices.push(new_quad.data[i]);
                    }
                    // mesh.push(new_quad);
                    indices.push(start_val);
                    indices.push(start_val+1);
                    indices.push(start_val+3);
                    indices.push(start_val);
                    indices.push(start_val+3);
                    indices.push(start_val+2);

                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                }
                if right_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let start_val: u32 = vertices.len() as u32;
                    let mut new_quad: Quad = RIGHT_QUAD;
                    for i in 0..4 {
                        new_quad.data[i] += block_pos + chunk_pos;
                        vertices.push(new_quad.data[i]);
                    }
                    // mesh.push(new_quad);
                    indices.push(start_val);
                    indices.push(start_val+1);
                    indices.push(start_val+3);
                    indices.push(start_val);
                    indices.push(start_val+3);
                    indices.push(start_val+2);

                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                }
                if front_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let start_val: u32 = vertices.len() as u32;
                    let mut new_quad: Quad = FRONT_QUAD;
                    for i in 0..4 {
                        new_quad.data[i] += block_pos + chunk_pos;
                        vertices.push(new_quad.data[i]);
                    }
                    // mesh.push(new_quad);
                    indices.push(start_val);
                    indices.push(start_val+1);
                    indices.push(start_val+3);
                    indices.push(start_val);
                    indices.push(start_val+3);
                    indices.push(start_val+2);

                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                }
                if back_neighbour_solid(&self.chunk_cache, chunk_index, block_index) {
                    let start_val: u32 = vertices.len() as u32;
                    let mut new_quad: Quad = BACK_QUAD;
                    for i in 0..4 {
                        new_quad.data[i] += block_pos + chunk_pos;
                        vertices.push(new_quad.data[i]);
                    }
                    // mesh.push(new_quad);
                    indices.push(start_val);
                    indices.push(start_val+1);
                    indices.push(start_val+3);
                    indices.push(start_val);
                    indices.push(start_val+3);
                    indices.push(start_val+2);

                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                    normals.push(Vec3 {x: 0.0, y: 0.0, z: 1.0});
                }
            }
            
            block_index.y -= (block_index.x >= CHUNKSIZE as u32) as u32;
            block_index.x %= CHUNKSIZE as u32;
            block_index.z -= (block_index.y >= CHUNKSIZE as u32) as u32;
            block_index.y %= CHUNKSIZE as u32;
            block_index.x -= 1;
        }
        // self.get_mut_chunk(chunk_index).unwrap().mesh_cache = mesh;
        
        let thingy: Mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices
        )
        // Assign a UV coordinate to each vertex.
        // .with_inserted_attribute(
        //     Mesh::ATTRIBUTE_UV_0,

        //     // vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]]
        // )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_inserted_indices(Indices::U32(
            indices
        ));
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
