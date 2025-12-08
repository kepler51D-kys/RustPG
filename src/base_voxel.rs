use bevy::prelude::*;
// use std::ops::Index;

use crate::v3;
pub const WORLDSIZE: u32 = 32;
pub const CHUNKSIZE: usize = 16;
pub const CHUNKLEN: usize = CHUNKSIZE*CHUNKSIZE*CHUNKSIZE;
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum BlockID {
    Air,
    Stone,
    Dirt,
}
impl BlockID {
    pub fn to_num(self) -> u16 {
        return self as u16;
    }
}
#[derive(Clone, Copy)]
pub struct Quad {
    pub data: [v3::V3; 4],
}
pub const topQuad: Quad = Quad {data: [
    v3::new(0,1,1),
    v3::new(1,1,1),
    v3::new(1,1,0),
    v3::new(0,1,0)
]};
pub const bottomQuad: Quad = Quad {data: [
    v3::new(0,0,1),
    v3::new(1,0,1),
    v3::new(1,0,0),
    v3::new(0,0,0)
]};
pub const leftQuad: Quad = Quad {data: [
    v3::new(0,0,0),
    v3::new(0,0,1),
    v3::new(0,1,1),
    v3::new(0,1,0)
]};
pub const rightQuad: Quad = Quad {data: [
    v3::new(1,0,0),
    v3::new(1,0,1),
    v3::new(1,1,1),
    v3::new(1,1,0)
]};
pub const backQuad: Quad = Quad {data: [
    v3::new(0,0,0),
    v3::new(1,0,0),
    v3::new(1,1,0),
    v3::new(0,1,0)
]};
pub const frontQuad: Quad = Quad {data: [
    v3::new(0,0,1),
    v3::new(1,0,1),
    v3::new(1,1,1),
    v3::new(0,1,1)
]};
pub struct Chunk {
    pub mesh_cache: Vec<Quad>,
    pub data: [u16; CHUNKLEN],
    pub mesh_dirty: bool,
    pub loaded: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        return Self {
            mesh_cache: Vec::new(),
            data: [0; CHUNKLEN],
            mesh_dirty: true,
            loaded: false
        }
    }
}