use glam::{UVec3,Vec3};

pub struct PlayerEntity {
   pub chunk_pos: UVec3,
   pub fine_pos: Vec3,
   pub render_distance_hor: u32,
   pub render_distance_ver: u32,
}
impl PlayerEntity {
   pub fn new(startpos:UVec3,render_distance_hor:u32,render_distance_ver:u32) -> Self {
      Self {
         chunk_pos: startpos,
         fine_pos: Vec3::ZERO,
         render_distance_hor,
         render_distance_ver
      }
   }
}