use std::cmp::{min,max};
use std::process;

use glam::{UVec3, Vec3};
use crate::app_manager::mesh::Mesh;
use crate::app_manager::state::State;
use crate::voxels::base_chunk::Chunk;
use crate::voxels::chunk_cache::ChunkCacheManager;
use crate::v3::{length, loose_less, loose_more_eq};
use crate::voxels::world_file::FileManager;
use crate::entities::player::{PlayerEntity};
pub struct WorldManager {
    chunk_manager: ChunkCacheManager,
    file_manager: FileManager,
    player: PlayerEntity,
    world_size: UVec3,
    // camera_buffer: wgpu::Buffer,
    // camera_bind_group: wgpu::BindGroup,
}
impl WorldManager {
    pub fn new(render_distance_hor:u32,render_distance_ver:u32) -> Self {
        Self {
            chunk_manager:ChunkCacheManager::new(
                (render_distance_hor*render_distance_hor*render_distance_ver*2) as usize
            ),
            world_size: UVec3 {x:128,y:128,z:64},
            player: PlayerEntity::new(
                Vec3 {x:1280.0,y:1280.0,z:640.0}/2.0,
                // UVec3 {x:0,y:0,z:0},
                render_distance_hor,
                render_distance_ver
            ),
            file_manager:FileManager::new("worlds/world1/region_0_0_0.world".to_string()),
        }
    }
    pub fn render_world(&mut self, state: &mut State) {
        if
            loose_more_eq(self.player.get_chunk_pos(), self.world_size) ||
            loose_less(self.player.get_chunk_pos(), UVec3::ZERO)
        {
            return;
        }
        // self.player.pos.z -= 0.2;
        // state.cam.eye = -self.player.pos;
        // state.cam.eye.y += 50.0;
        // state.cam.target = self.player.pos;
        // state.cam.target.z += 50.0;
        // state.cam.camera_uniform = state.cam.build_view_projection_matrix();
        // state.queue.write_buffer(&state.cam.camera_buffer, 0, bytemuck::cast_slice(&[state.cam.camera_uniform]));

        // init both bounds
        let start: UVec3 = UVec3 {
            x: max(min(self.player.get_chunk_pos().x-self.player.render_distance_hor, self.world_size.x-1),0),
            y: max(min(self.player.get_chunk_pos().y-self.player.render_distance_ver, self.world_size.y-1),0),
            z: max(min(self.player.get_chunk_pos().z-self.player.render_distance_hor, self.world_size.z-1),0),
        };
        let end: UVec3 = UVec3 {
            x: max(min(self.player.get_chunk_pos().x+self.player.render_distance_hor, self.world_size.x-1),0),
            y: max(min(self.player.get_chunk_pos().y+self.player.render_distance_ver, self.world_size.y-1),0),
            z: max(min(self.player.get_chunk_pos().z+self.player.render_distance_hor, self.world_size.z-1),0),
        };
        // println!("{}|||",state.cam.target);
        // println!("{} | {}",start,end);
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    let index: UVec3 = UVec3 {x:x,y:y,z:z};
                    if !self.chunk_manager.chunk_present(&index) {
                        // let chunk: Chunk = self.file_manager.read_chunk(index).unwrap();
                        let chunk: Chunk = Chunk::default();
                        self.chunk_manager.add_chunk(&index, chunk);
                    }
                    else {
                    }
                    let mesh: &Mesh = &self.chunk_manager.get_chunk(index).unwrap().mesh_cache;
                    println!("{}\n---------------",self.player.pos);
                    for i in 0..mesh.vertices.len() {
                        if length(mesh.vertices[i].to_vec3()-self.player.pos) < 1.0 {
                            // println!("{} : {} {} {}",i,
                            //     mesh.vertices[i].pos[0],
                            //     mesh.vertices[i].pos[1],
                            //     mesh.vertices[i].pos[2],
                            // );
                            // println!("{}",mesh.vertices[i].to_vec3());
                        }
                        // println!("{} {} {}",
                        //     mesh.vertices[i].pos[0],
                            // mesh.vertices[i].pos[1],
                        //     mesh.vertices[i].pos[2],
                        // );
                        println!("{}",mesh.vertices[i].to_vec3());
                    }
                    self.chunk_manager.render_chunk(index,state);
                }
            }
        }
    }
}