use glam::Vec3;
use wgpu::naga::Block;

#[derive(Clone, Copy)]
pub enum BlockID {
    Air,Stone,Dirt,CobbleStone
}
pub const CHUNKSIZE: u32 = 64;
pub struct OctNode {
    pub block: BlockID,
    pub children: [u32; 8],
    pub is_leaf: bool,
}
impl OctNode {
    pub fn new_leaf(block : BlockID) -> OctNode {
        Self {
            block,
            children: [0;8],
            is_leaf: true,
        }
    }
}
const BOUNDS: [Vec3;8] = [
    Vec3::from_array([0.5,0.5,0.5]),
    Vec3::from_array([0.5,0.5,0.0]),
    Vec3::from_array([0.5,0.0,0.5]),
    Vec3::from_array([0.5,0.0,0.0]),

    Vec3::from_array([0.0,0.5,0.5]),
    Vec3::from_array([0.0,0.5,0.0]),
    Vec3::from_array([0.0,0.0,0.5]),
    Vec3::from_array([0.0,0.0,0.0]),
];
fn get_offset(index: u32) -> Vec3 {
    BOUNDS[(7-index) as usize]
}
fn get_bound(index: u32) -> Vec3 {
    BOUNDS[index as usize]
}

pub struct OctTree {
    pub children: Vec<[u32;8]>,
    pub is_leaf: Vec<bool>,
    pub blocks: Vec<BlockID>,
}
impl OctTree {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            is_leaf: Vec::new(),
            blocks: Vec::new(),
        }
    }
    pub fn add_node(&mut self, node: OctNode) {
        self.children.push(node.children);
        self.is_leaf.push(node.is_leaf);
        self.blocks.push(node.block);
    }
    pub fn get_node(&self, index: usize) -> OctNode {
        OctNode {
            block: self.blocks[index],
            children: self.children[index],
            is_leaf: self.is_leaf[index],
        }
    }
    pub fn split_node(&mut self, index: usize, blocks: [BlockID;8]) {
        self.is_leaf[index] = false;
        let addr: usize = self.children.len();
        for i in 0..8 {
            self.children[index][i] = (addr+i) as u32;
            self.blocks[addr+i] = blocks[i];
            self.is_leaf[addr+i] = false;
        }
        
    }
}