use glam::{IVec3, UVec3, Vec3};
use slotmap::{SlotMap, new_key_type};
use crate::dual_contouring::{chunk_retriever::{WorldFileManager}, oct_tree::OctTree};

pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}
#[derive(Debug,Clone)]
pub struct RingBuffer3D<T: Copy> {
    pub data: Vec<T>,
    pub size: usize,
    pub offset: IVec3,
}
impl<T: Copy> RingBuffer3D<T> {
    pub fn new(size: usize) -> Self {
        Self {
            data:Vec::with_capacity(size*size*size),
            size,
            offset: IVec3::ZERO,
        }
    }
    pub fn get_index(&self, index: UVec3) -> usize {
        let index_vec: IVec3 = IVec3::from((
            index.x as i32,
            index.y as i32,
            index.z as i32,
        )) + self.offset;
        index_vec.x as usize * self.size * self.size +
        index_vec.y as usize * self.size +
        index_vec.z as usize
    }
    pub fn add(&mut self, side_to_add: Side, new_data: Vec<T>) {
        // let side_axis: IVec3 = SIDE_AXIS[side_to_add as usize];
        // let sign: i32 = side_axis.x + side_axis.y + side_axis.z;
        match side_to_add {
            Side::Back => { // add to z beginning
                self.offset = (self.offset + IVec3 {x:0,y:0,z:1}) % IVec3::splat(self.size as i32);
                for x in 0..self.size {
                    for y in 0..self.size {
                        let index: usize = self.get_index(UVec3 { x: x as u32, y: y as u32, z: 0});
                        self.data[index] = new_data[x*self.size+y];
                    }
                }
            },
            Side::Front => { // add to z end
                self.offset = (self.offset + IVec3 {x:0,y:0,z:-1}) % IVec3::splat(self.size as i32);
                for x in 0..self.size {
                    for y in 0..self.size {
                        let index: usize = self.get_index(UVec3 { x: x as u32, y: y as u32, z: self.size as u32 -1});
                        self.data[index] = new_data[x*self.size+y];
                    }
                }
            },
            Side::Left => { // add to x beginning
                self.offset = (self.offset + IVec3 {x:-1,y:0,z:0}) % IVec3::splat(self.size as i32);
                for y in 0..self.size {
                    for z in 0..self.size {
                        let index: usize = self.get_index(UVec3 { x: 0, y: y as u32, z: z as u32});
                        self.data[index] = new_data[y*self.size+z];
                    }
                }
            },
            Side::Right => { // add to x end
                self.offset = (self.offset + IVec3 {x:0,y:0,z:1}) % IVec3::splat(self.size as i32);
                for y in 0..self.size {
                    for z in 0..self.size {
                        let index: usize = self.get_index(UVec3 { x: self.size as u32 -1, y: y as u32, z: z as u32 });
                        self.data[index] = new_data[y*self.size+z];
                    }
                }
            },
            Side::Top => { // add to y end
                self.offset = (self.offset + IVec3 {x:0,y:0,z:1}) % IVec3::splat(self.size as i32);
                for x in 0..self.size {
                    for z in 0..self.size {
                        let index: usize = self.get_index(UVec3 { x: x as u32, y: self.size as u32 -1, z: z as u32});
                        self.data[index] = new_data[x*self.size+z];
                    }
                }
            },
            Side::Bottom => { // add to y beginning
                self.offset = (self.offset + IVec3 {x:0,y:0,z:1}) % IVec3::splat(self.size as i32);
                for x in 0..self.size {
                    for z in 0..self.size {
                        let index: usize = self.get_index(UVec3 { x: x as u32, y: 0, z: z as u32});
                        self.data[index] = new_data[x*self.size+z];
                    }
                }
            }
        }
    }
}

new_key_type! {
    pub struct ChunkKey;
}

pub struct RenderManager {
    pub chunk_pool: SlotMap<ChunkKey,OctTree>,
    pub render_pool: RingBuffer3D<ChunkKey>,
    pub camera_pos: Vec3,
    pub chunk_retriever: WorldFileManager,
}
impl RenderManager {
    pub fn new(camera_pos: Vec3,render_distance: usize) -> Self {
        let mut manager = Self {
            chunk_pool: SlotMap::with_key(),
            render_pool: RingBuffer3D::new(render_distance),
            camera_pos,
            chunk_retriever: WorldFileManager::open_world(String::from("test")),
        };
        for x in 0..render_distance {
            for y in 0..render_distance {
                for z in 0..render_distance {
                    let index = manager.render_pool.get_index(UVec3 {x:x as u32, y:y as u32, z:z as u32});
                    let chunk_key = manager.chunk_pool.insert(manager.chunk_retriever.retrieve_chunk(UVec3 {
                        x: x as u32,
                        y: y as u32,
                        z: z as u32,
                    }));
                    manager.render_pool.data[index] = chunk_key;

                    // either get chunk from file or generate it
                    // or fetch from server but thats a low priority todo
                }
            }
        }
        manager
    }
    pub fn get_chunk(&mut self, index: UVec3) -> ChunkKey {
        let chunk_index: usize = self.render_pool.get_index(index);
        let chunk_key: ChunkKey = self.render_pool.data[chunk_index];
        if self.chunk_pool.contains_key(chunk_key) { // chunk in memory
            chunk_key
        }
        else { // chunk in file or doesnt exist
            let chunk: OctTree = self.chunk_retriever.retrieve_chunk(index);
            let new_key: ChunkKey = self.chunk_pool.insert(chunk);
            self.render_pool.data[chunk_index] = new_key;
            new_key
        }
    }
}