use crate::v3;
#[derive(Clone, Copy)]
pub struct Quad {
    pub data: [v3::V3; 4],
}
pub const RENDER_DISTANCE: u32 = 8;
pub const TOP_QUAD: Quad = Quad {data: [
    v3::new(0,1,1),
    v3::new(1,1,1),
    v3::new(1,1,0),
    v3::new(0,1,0)
]};
pub const BOTTOM_QUAD: Quad = Quad {data: [
    v3::new(0,0,1),
    v3::new(1,0,1),
    v3::new(1,0,0),
    v3::new(0,0,0)
]};
pub const LEFT_QUAD: Quad = Quad {data: [
    v3::new(0,0,0),
    v3::new(0,0,1),
    v3::new(0,1,1),
    v3::new(0,1,0)
]};
pub const RIGHT_QUAD: Quad = Quad {data: [
    v3::new(1,0,0),
    v3::new(1,0,1),
    v3::new(1,1,1),
    v3::new(1,1,0)
]};
pub const BACK_QUAD: Quad = Quad {data: [
    v3::new(0,0,0),
    v3::new(1,0,0),
    v3::new(1,1,0),
    v3::new(0,1,0)
]};
pub const FRONT_QUAD: Quad = Quad {data: [
    v3::new(0,0,1),
    v3::new(1,0,1),
    v3::new(1,1,1),
    v3::new(0,1,1)
]};