use crate::advanced_rendering::model::{Mesh};

const WORLD_SIZE: usize = 65536;

pub fn get_distance_index(x:u32,y:u32,z:u32) -> usize {
    (x & 63) as usize * WORLD_SIZE * WORLD_SIZE +
    (y & 63) as usize * WORLD_SIZE +
    (z & 63) as usize
}

#[repr(u16)]
#[derive(Clone, Copy,Debug)]
pub enum BlockID {
    Air,
    Stone,
    Dirt,
    CobbleStone,
}

// size of the Node, in terms of the area it covers, can be calculated by MAX_TREE_DEPTH / current depth.
pub struct OctNode {
    pub parent: u32, // address of parent
    pub children: u32, // address of first child: if value is zero, then its a leaf
    pub dist: [[[f32; 2]; 2]; 2], // distance to surface at 8 points of cube
    pub block_type: BlockID,
} // if size is too large, change dist to f16

pub struct OctTree {
    pub mesh_cache: Mesh,
    pub distances: Vec<[[[f32; 2]; 2]; 2]>,
    pub parents: Vec<u32>,
    pub children: Vec<u32>,
    pub block_type: Vec<BlockID>,
}
impl OctTree {
    pub const MAX_TREE_DEPTH: usize = 64;
    
    pub fn new_node(&mut self, parent: u32, block_type: BlockID, dist: [[[f32; 2]; 2]; 2]) {
        self.distances.push(dist);
        self.parents.push(parent);
        self.block_type.push(block_type);
        self.children.push(0);
    }
    pub fn get_node(&self, index: usize) -> OctNode {
        OctNode {
            parent: self.parents[index],
            children: self.children[index],
            dist: self.distances[index],
            block_type: self.block_type[index],
        }
    }
    pub fn set_node(&mut self, node: OctNode, index: usize) {
        self.distances[index] = node.dist;
        self.block_type[index] = node.block_type;
        self.children[index] = node.children;
        self.parents[index] = node.parent;
    }
    pub fn del_node(&mut self,index: usize) {
        self.distances.remove(index);
        self.parents.remove(index);
        self.block_type.remove(index);
        self.children.remove(index);
    }
    pub fn is_leaf(&self, index: usize) -> bool {
        self.children[index] == 0
    }
    pub fn merge_children(&mut self, parent_index: usize) -> bool { // returns true if success
        if self.children[parent_index] == 0 {
            false
        }
        else {
            for i in self.children[parent_index]..(self.children[parent_index]+7) {
                self.del_node(i as usize);
            }
            self.children[parent_index] = 0;
            true
        }
    }
    pub fn make_mesh(&self) {

    }
}