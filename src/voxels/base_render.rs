use crate::app_manager::mesh::Vertex;

#[derive(Clone, Copy)]
pub struct Quad {
    pub data: [Vertex; 4],
}
pub const RENDER_DISTANCE: u32 = 8;
pub const TOP_QUAD: Quad = Quad {data: [
    Vertex {pos:[0.0,1.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,1.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,1.0,0.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[0.0,1.0,0.0], texture_coord: [0.0,1.0]}
]};
pub const BOTTOM_QUAD: Quad = Quad {data: [
    Vertex {pos:[0.0,0.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,0.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,0.0,0.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[0.0,0.0,0.0], texture_coord: [0.0,1.0]}
]};
pub const LEFT_QUAD: Quad = Quad {data: [
    Vertex {pos:[0.0,0.0,0.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[0.0,0.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[0.0,1.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[0.0,1.0,0.0], texture_coord: [0.0,1.0]}
]};
pub const RIGHT_QUAD: Quad = Quad {data: [
    Vertex {pos:[1.0,0.0,0.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,0.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,1.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,1.0,0.0], texture_coord: [0.0,1.0]}
]};
pub const BACK_QUAD: Quad = Quad {data: [
    Vertex {pos:[0.0,0.0,0.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,0.0,0.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,1.0,0.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[0.0,1.0,0.0], texture_coord: [0.0,1.0]}
]};
pub const FRONT_QUAD: Quad = Quad {data: [
    Vertex {pos:[0.0,0.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,0.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[1.0,1.0,1.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[0.0,1.0,1.0], texture_coord: [0.0,1.0]}
]};