use glam::{UVec3, Vec3};

use crate::voxels::base_chunk::CHUNKSIZE;

pub struct PlayerEntity {
   pub pos: Vec3,
   pub render_distance_hor: u32,
   pub render_distance_ver: u32,
}
impl PlayerEntity {
   pub fn new(startpos: Vec3, render_distance_hor: u32, render_distance_ver: u32) -> Self {
      Self {
         pos: startpos,
         render_distance_hor,
         render_distance_ver
      }
   }
   pub fn get_chunk_pos(&self) -> UVec3 {
      UVec3 {
         x: (self.pos.x/CHUNKSIZE as f32) as u32,
         y: (self.pos.y/CHUNKSIZE as f32) as u32,
         z: (self.pos.z/CHUNKSIZE as f32) as u32,
      }
   }
}