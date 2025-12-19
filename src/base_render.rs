use bevy::math::Vec3;

#[derive(Clone, Copy)]
pub struct Quad {
    pub data: [Vec3; 4],
}
pub const RENDER_DISTANCE: u32 = 8;
pub const TOP_QUAD: Quad = Quad {data: [
    Vec3 {x:0.0,y:1.0,z:1.0},
    Vec3 {x:1.0,y:1.0,z:1.0},
    Vec3 {x:1.0,y:1.0,z:0.0},
    Vec3 {x:0.0,y:1.0,z:0.0}
]};
pub const BOTTOM_QUAD: Quad = Quad {data: [
    Vec3 {x:0.0,y:0.0,z:1.0},
    Vec3 {x:1.0,y:0.0,z:1.0},
    Vec3 {x:1.0,y:0.0,z:0.0},
    Vec3 {x:0.0,y:0.0,z:0.0}
]};
pub const LEFT_QUAD: Quad = Quad {data: [
    Vec3 {x:0.0,y:0.0,z:0.0},
    Vec3 {x:0.0,y:0.0,z:1.0},
    Vec3 {x:0.0,y:1.0,z:1.0},
    Vec3 {x:0.0,y:1.0,z:0.0}
]};
pub const RIGHT_QUAD: Quad = Quad {data: [
    Vec3 {x:1.0,y:0.0,z:0.0},
    Vec3 {x:1.0,y:0.0,z:1.0},
    Vec3 {x:1.0,y:1.0,z:1.0},
    Vec3 {x:1.0,y:1.0,z:0.0}
]};
pub const BACK_QUAD: Quad = Quad {data: [
    Vec3 {x:0.0,y:0.0,z:0.0},
    Vec3 {x:1.0,y:0.0,z:0.0},
    Vec3 {x:1.0,y:1.0,z:0.0},
    Vec3 {x:0.0,y:1.0,z:0.0}
]};
pub const FRONT_QUAD: Quad = Quad {data: [
    Vec3 {x:0.0,y:0.0,z:1.0},
    Vec3 {x:1.0,y:0.0,z:1.0},
    Vec3 {x:1.0,y:1.0,z:1.0},
    Vec3 {x:0.0,y:1.0,z:1.0}
]};