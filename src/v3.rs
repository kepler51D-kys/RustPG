use std::f32;
use glam::{UVec3, Vec3};

use crate::{app_manager::mesh::Vertex, voxels::base_chunk::CHUNKSIZE};

pub fn get_block_index(vec: UVec3) -> usize {
    return (vec.x*CHUNKSIZE as u32*CHUNKSIZE as u32 + vec.y*CHUNKSIZE as u32+vec.z) as usize;
}
pub fn loose_more(vec1: UVec3, vec2: UVec3) -> bool {
    vec1.x > vec2.x || vec1.y > vec2.y || vec1.z > vec2.z
}
pub fn loose_more_eq(vec1: UVec3, vec2: UVec3) -> bool {
    vec1.x >= vec2.x || vec1.y >= vec2.y || vec1.z >= vec2.z
}
pub fn loose_less(vec1: UVec3, vec2: UVec3) -> bool {
    vec1.x < vec2.x || vec1.y < vec2.y || vec1.z < vec2.z
}
pub fn loose_less_eq(vec1: UVec3, vec2: UVec3) -> bool {
    vec1.x <= vec2.x || vec1.y <= vec2.y || vec1.z <= vec2.z
}

pub fn length(input: Vec3) -> f32 {
    return (input.x*input.x+input.y*input.y+input.z*input.z).powf(0.5);
}