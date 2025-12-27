use std::cmp::{min,max};

use glam::UVec3;

use crate::voxels::chunk_cache::ChunkCacheManager;
use crate::v3::{loose_more,loose_less};
use crate::voxels::world_file::FileManager;
use crate::entities::{PlayerEntity};

struct WorldManager {
    chunk_manager: ChunkCacheManager,
    file_manager: FileManager,
    player: PlayerEntity,
    render_distance_hor: u32,
    render_distance_ver: u32,
    world_size: UVec3
}
impl WorldManager {
    pub fn render_world(&mut self) {
        if 
            loose_more(self.player.chunk_pos, self.world_size) ||
            loose_less(self.player.chunk_pos, UVec3::ZERO)
        {
            return;
        }
        // init both bounds
        let start: UVec3 = UVec3 {
            x: max(min(self.player.chunk_pos.x-self.render_distance_hor, self.world_size.x-1),0),
            y: max(min(self.player.chunk_pos.y-self.render_distance_ver, self.world_size.y-1),0),
            z: max(min(self.player.chunk_pos.z-self.render_distance_hor, self.world_size.z-1),0),
        };
        let end: UVec3 = UVec3 {
            x: max(min(self.player.chunk_pos.x+self.render_distance_hor, self.world_size.x-1),0),
            y: max(min(self.player.chunk_pos.y+self.render_distance_ver, self.world_size.y-1),0),
            z: max(min(self.player.chunk_pos.z+self.render_distance_hor, self.world_size.z-1),0),
        };
        for x in start.x..end.x {
            for y in start.y..end.y {
                for z in start.z..end.z {
                    self.chunk_manager.render_chunk(UVec3 {x:x,y:y,z:z});
                }
            }
        }
    }
}