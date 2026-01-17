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
    pub children: u32, // address of first child: if zero, then its a leaf
    pub block_type: BlockID,
    pub pos: usize, // reference distance to surface at 8 points of cube
} // if size to large, change dist to f16

pub struct OctTree {
    pub nodes: Vec<OctNode>,
}
impl OctTree {
    pub fn new_node(&mut self, parent: u32, block: BlockID, pos: usize) {
        self.nodes.push(OctNode {
            block_type: block,
            parent,
            children: 0,
            pos,
        });
    }
}
pub struct MapData { // per chunk
    pub oct_tree: OctTree,
    pub distance: [f32; MAX_TREE_DEPTH*MAX_TREE_DEPTH*MAX_TREE_DEPTH],
}