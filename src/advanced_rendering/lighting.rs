use crate::app_manager::mesh::Vertex;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    pos: [f32;3],
    _padding0: u32,
    col: [f32;3],
    _padding1: u32,
}
impl LightUniform {
    pub fn new(dat: Vertex) -> Self {
        Self {
            _padding0: 0,
            _padding1: 0,
            pos: dat.pos,
            col: [dat.texture_coord[0],dat.texture_coord[1],0.0],
        }
    }
}