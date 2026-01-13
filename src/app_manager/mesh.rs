use std::ops::{Add, AddAssign, Sub, SubAssign};

use glam::{Mat4, Vec3};
use wgpu::{Device, util::DeviceExt};

use crate::voxels::chunk_cache::IndicesSize;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub texture_coord: [f32; 2],
}
impl Vertex {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x:self.pos[0],
            y:self.pos[1],
            z:self.pos[2],
        }
    }
}
impl From<Vec3> for Vertex {
    fn from(value: Vec3) -> Self {
        Self {
            pos: [
                value.x,
                value.y,
                value.z,
            ],
            texture_coord: [
                1.0,
                1.0,
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
            ],
            texture_coord: self.texture_coord,
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
            ],
            texture_coord: self.texture_coord,
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
    pub indices: Vec<IndicesSize>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
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
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
    
    pub fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
}