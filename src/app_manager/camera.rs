use glam::{Mat4, Vec3, Vec4};
use wgpu::{BindGroupLayout, Device, Queue, SurfaceConfiguration, util::DeviceExt};
use crate::advanced_rendering::camera;
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_uniform: CameraUniform,
}
#[repr(C)]
#[derive(Debug,bytemuck::Pod,Copy,Clone,bytemuck::Zeroable)]
pub struct CameraUniform {
    pub pos: Vec4,
    pub matrix: Mat4,
}
impl CameraUniform {
    pub fn update_view_proj(&mut self, camera: &camera::Camera, projection: &camera::Projection) {
        self.pos = camera.position.to_homogeneous().into();
        self.matrix = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}

impl Camera {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        queue: &Queue,
        camera_bind_group_layout: &BindGroupLayout) -> Self
        {
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[CameraUniform {pos: Vec4::default(), matrix: Mat4::default()}]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let mut camera = Camera {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 8.0, -16.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: Vec3::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 1000.0,
            camera_uniform: CameraUniform {pos:Vec4::default(),matrix:Mat4::default()},
            // camera_bind_group_layout,
            camera_bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }
                ],
                label: Some("camera_bind_group"),
            }),
            camera_buffer,
        };
        camera.build_view_projection_matrix();
        queue.write_buffer(&camera.camera_buffer, 0, bytemuck::cast_slice(&[camera.camera_uniform]));
        return camera;
    }
    pub fn build_view_projection_matrix(&mut self) {
        
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        
        // let proj = Mat4::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let proj: Mat4 = Mat4::perspective_rh( // note: might be left hand (lh) try both
            self.fovy,
            self.aspect,
            self.znear,
            self.zfar
        );
        self.camera_uniform = CameraUniform {pos:Vec4::from((self.eye,0.0)), matrix:proj * view};
    }
}