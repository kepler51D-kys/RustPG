use std::ops::{Add, AddAssign, Sub, SubAssign};

use glam::{Mat4, Vec3};
use wgpu::{Device, util::DeviceExt};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3]
}
impl From<Vec3> for Vertex {
    fn from(value: Vec3) -> Self {
        Self {
            pos: [
                value.x,
                value.y,
                value.z,
            ]
        }
    }
}
impl Add for Vertex {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pos: [
                self.pos[0] + rhs.pos[0],
                self.pos[1] + rhs.pos[1],
                self.pos[2] + rhs.pos[2],
            ]
        }
    }
}
impl Sub for Vertex {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            pos: [
                self.pos[0] - rhs.pos[0],
                self.pos[1] - rhs.pos[1],
                self.pos[2] - rhs.pos[2],
            ]
        }
    }
}
impl AddAssign for Vertex {
    fn add_assign(&mut self, rhs: Self) {
        self.pos[0] += rhs.pos[0];
        self.pos[1] += rhs.pos[1];
        self.pos[2] += rhs.pos[2];
    }
}
impl SubAssign for Vertex {
    fn sub_assign(&mut self, rhs: Self) {
        self.pos[0] -= rhs.pos[0];
        self.pos[1] -= rhs.pos[1];
        self.pos[2] -= rhs.pos[2];
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn construct_transform_matrix_buffer(&self, device: &wgpu::Device, transform: Mat4) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Uniform Buffer"),
                contents: bytemuck::cast_slice(&[transform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        )
    }

    pub fn construct_vertex_buffer(&self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        )
    }
    
    pub fn construct_index_buffer(&self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        )
    }
    pub fn transform_matrix_buffer_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 1,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 2,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 3,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
    pub fn vertex_buffer_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
    pub fn indices_buffer_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<u16>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Uint16,
                },
            ]
        }
    }
    
    pub fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
}