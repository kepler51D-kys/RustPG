use glam::{Vec3,Quat,Mat4};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov: f32,
    pub aspect: f32,
    
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
}
impl Camera {
    pub fn update_matrices(&mut self) {
        self.view_matrix = Mat4::look_to_rh(
            self.position,
            self.rotation * Vec3::NEG_Z,
            Vec3::Y,
        );
        
        self.projection_matrix = Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect,
            0.1,
            100.0,
        );
    }
    pub fn new(fov: f32) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::default(),
            fov,
            aspect: 16.0/9.0,
            view_matrix: Mat4::ZERO,
            projection_matrix: Mat4::ZERO,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform(Mat4);