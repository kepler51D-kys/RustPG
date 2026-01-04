use std::cmp::{min,max};

use glam::{UVec3,Mat4};
use crate::app_manager::window::State;
use crate::voxels::base_chunk::Chunk;
use crate::voxels::chunk_cache::ChunkCacheManager;
use crate::v3::{loose_less, loose_more_eq};
use crate::voxels::world_file::FileManager;
use crate::entities::{PlayerEntity};

pub struct WorldManager {
    chunk_manager: ChunkCacheManager,
    file_manager: FileManager,
    player: PlayerEntity,
    world_size: UVec3
}
impl WorldManager {
    pub fn new(render_distance_hor:u32,render_distance_ver:u32) -> Self {
        Self {
            chunk_manager:ChunkCacheManager::new(
                (render_distance_hor*render_distance_hor*render_distance_ver*2) as usize
            ),
            world_size: UVec3 {x:128,y:128,z:64},
            player: PlayerEntity::new(
                UVec3 {x:128,y:128,z:64}/2,
                render_distance_hor,
                render_distance_ver
            ),
            file_manager:FileManager::new("worlds/world1/region_0_0_0.world".to_string()),
        }
    }
    pub fn render_world(&mut self, state: &mut State) {
        if 
            loose_more_eq(self.player.chunk_pos, self.world_size) ||
            loose_less(self.player.chunk_pos, UVec3::ZERO)
        {
            return;
        }

        let mut transform: Mat4 = Mat4::IDENTITY;
        transform.w_axis.x = self.player.chunk_pos.x as f32;
        transform.w_axis.y = self.player.chunk_pos.y as f32;
        transform.w_axis.z = self.player.chunk_pos.z as f32;
        
        // init both bounds
        let start: UVec3 = UVec3 {
            x: max(min(self.player.chunk_pos.x-self.player.render_distance_hor, self.world_size.x-1),0),
            y: max(min(self.player.chunk_pos.y-self.player.render_distance_ver, self.world_size.y-1),0),
            z: max(min(self.player.chunk_pos.z-self.player.render_distance_hor, self.world_size.z-1),0),
        };
        let end: UVec3 = UVec3 {
            x: max(min(self.player.chunk_pos.x+self.player.render_distance_hor, self.world_size.x-1),0),
            y: max(min(self.player.chunk_pos.y+self.player.render_distance_ver, self.world_size.y-1),0),
            z: max(min(self.player.chunk_pos.z+self.player.render_distance_hor, self.world_size.z-1),0),
        };
        println!("{} | {}",start,end);
        for x in start.x..=end.x {
            // println!("hello");
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    let index: UVec3 = UVec3 {x:x,y:y,z:z};
                    if self.chunk_manager.chunk_present(&index) {
                        let chunk: Chunk = self.file_manager.read_chunk(index).unwrap();
                        self.chunk_manager.add_chunk(index, chunk);
                    }
                    self.chunk_manager.render_chunk(index,state);
                }
            }
        }
    }
}