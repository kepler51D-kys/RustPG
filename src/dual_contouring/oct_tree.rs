const WORLD_SIZE: usize = 65536;
pub const MAX_TREE_DEPTH: usize = 64;

pub fn get_distance_index(x:u32,y:u32,z:u32) -> usize {
    (x & 63) as usize * WORLD_SIZE * WORLD_SIZE +
    (y & 63) as usize * WORLD_SIZE +
    (z & 63) as usize
}

#[repr(u16)]
#[derive(Clone, Copy,Debug)]
pub enum BlockID {
    Air,Stone,Dirt,CobbleStone
}

// size of the Node, in terms of the area it covers, can be calculated by MAX_TREE_DEPTH / current depth.
pub struct OctNode {
    pub parent: u32, // address of parent
    pub children: u32, // address of first child: if value is zero, then its a leaf
    pub block_type: BlockID,
    pub dist: [[[f32; 2]; 2]; 2], // distance to surface at 8 points of cube
} // if size is too large, change dist to f16

pub struct OctTree {
    pub nodes: Vec<OctNode>,
    // pub pos: Vec<usize>,
    // pub parents: Vec<u32>,
    // pub children: Vec<u32>,
    // pub block_type: Vec<BlockID>,
}
impl OctTree {
    pub fn new_node(&mut self, parent: u32, block_type: BlockID, pos: [[[f32; 2]; 2]; 2]) {
        self.nodes.push(OctNode {
            block_type,
            parent,
            children: 0,
            pos,
        });
    }
    pub fn render_test(&self) {
        let mut pos_list: Vec<u32> = Vec::new();
        pos_list.push(0);
        while true {
            let node: OctNode = self.nodes(pos_list[-1])
        }
    }
}
pub struct MapData { // per chunk
    pub oct_tree: OctTree,
}