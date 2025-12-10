use crate::v3;
#[derive(Clone, Copy)]
pub struct Quad {
    pub data: [v3::V3; 4],
}
pub const top_quad: Quad = Quad {data: [
    v3::new(0,1,1),
    v3::new(1,1,1),
    v3::new(1,1,0),
    v3::new(0,1,0)
]};
pub const bottom_quad: Quad = Quad {data: [
    v3::new(0,0,1),
    v3::new(1,0,1),
    v3::new(1,0,0),
    v3::new(0,0,0)
]};
pub const left_quad: Quad = Quad {data: [
    v3::new(0,0,0),
    v3::new(0,0,1),
    v3::new(0,1,1),
    v3::new(0,1,0)
]};
pub const right_quad: Quad = Quad {data: [
    v3::new(1,0,0),
    v3::new(1,0,1),
    v3::new(1,1,1),
    v3::new(1,1,0)
]};
pub const back_quad: Quad = Quad {data: [
    v3::new(0,0,0),
    v3::new(1,0,0),
    v3::new(1,1,0),
    v3::new(0,1,0)
]};
pub const front_quad: Quad = Quad {data: [
    v3::new(0,0,1),
    v3::new(1,0,1),
    v3::new(1,1,1),
    v3::new(0,1,1)
]};