use glam::{IVec3};
use slotmap::{SlotMap, new_key_type};
use crate::dual_contouring::oct_tree::OctTree;

pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}
const  SIDE_AXIS: &[IVec3] = &[
    IVec3 {x:-1,y:0,z:0},
    IVec3 {x:1,y:0,z:0},

    IVec3 {x:0,y:1,z:0},
    IVec3 {x:0,y:-1,z:0},
    
    IVec3 {x:0,y:0,z:1},
    IVec3 {x:0,y:0,z:-1},
];
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
    pub fn get_index(&self, index: IVec3) -> usize {
        let index_vec: IVec3 = index + self.offset;
        index_vec.x as usize * self.size * self.size +
        index_vec.y as usize * self.size +
        index_vec.z as usize
    }
    pub fn shift(&mut self, side_to_add: Side, new_data: Vec<T>) {
        // let side_axis: IVec3 = SIDE_AXIS[side_to_add as usize];
        // let sign: i32 = side_axis.x + side_axis.y + side_axis.z;
        match side_to_add {
            Side::Back => { // add to z beginning
                self.offset = (self.offset + IVec3 {x:0,y:0,z:1}) % IVec3::splat(self.size as i32);
                for x in 0..self.size {
                    for y in 0..self.size {
                        let index: usize = self.get_index(IVec3 { x: x as i32, y: y as i32, z: 0});
                        self.data[index] = new_data[x*self.size+y];
                    }
                }
            },
            Side::Front => { // add to z end
                self.offset = (self.offset + IVec3 {x:0,y:0,z:-1}) % IVec3::splat(self.size as i32);
                for x in 0..self.size {
                    for y in 0..self.size {
                        let index: usize = self.get_index(IVec3 { x: x as i32, y: y as i32, z: self.size as i32 -1});
                        self.data[index] = new_data[x*self.size+y];
                    }
                }
            },
            Side::Left => { // add to x beginning
                self.offset = (self.offset + IVec3 {x:-1,y:0,z:0}) % IVec3::splat(self.size as i32);
                for y in 0..self.size {
                    for z in 0..self.size {
                        let index: usize = self.get_index(IVec3 { x: 0, y: y as i32, z: z as i32});
                        self.data[index] = new_data[y*self.size+z];
                    }
                }
            },
            Side::Right => { // add to x end
                self.offset = (self.offset + IVec3 {x:0,y:0,z:1}) % IVec3::splat(self.size as i32);
                for y in 0..self.size {
                    for z in 0..self.size {
                        let index: usize = self.get_index(IVec3 { x: self.size as i32 -1, y: y as i32, z: z as i32 });
                        self.data[index] = new_data[y*self.size+z];
                    }
                }
            },
            Side::Top => { // add to y end
                self.offset = (self.offset + IVec3 {x:0,y:0,z:1}) % IVec3::splat(self.size as i32);
                for x in 0..self.size {
                    for z in 0..self.size {
                        let index: usize = self.get_index(IVec3 { x: x as i32, y: self.size as i32 -1, z: z as i32});
                        self.data[index] = new_data[x*self.size+z];
                    }
                }
            },
            Side::Bottom => { // add to y beginning
                self.offset = (self.offset + IVec3 {x:0,y:0,z:1}) % IVec3::splat(self.size as i32);
                for x in 0..self.size {
                    for z in 0..self.size {
                        let index: usize = self.get_index(IVec3 { x: x as i32, y: 0, z: z as i32});
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

// #[allow(dead_code)]
pub struct WorldManager {
    pub chunk_pool: SlotMap<ChunkKey,OctTree>,
    pub render_pool: RingBuffer3D<ChunkKey>,
}
impl WorldManager {
    
}