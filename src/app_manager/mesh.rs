use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::io::{BufReader, Cursor, Read};
use glam::{Mat4, Vec2, Vec3};
use wgpu::{Device, util::DeviceExt};
use crate::{advanced_rendering::{render_vertex::Vertex, texture::Texture}, app_manager::app::IndicesSize};

pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
    let txt = {
        let path = std::path::Path::new(env!("OUT_DIR"))
            .join("res")
            .join(file_name);
        // println!("{}",path.display());
        std::fs::read_to_string(path)?
    };

    Ok(txt)
}

pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let data = {
        let path = std::path::Path::new(env!("OUT_DIR"))
            .join("res")
            .join(file_name);
        // println!("{}",path.display());
        std::fs::read(path)?
    };

    Ok(data)
}


pub fn construct_vertex_buffer(vertices: &Vec<Vertex>, device: &Device) -> wgpu::Buffer {
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }
    )
}
pub fn construct_index_buffer(indices: &Vec<IndicesSize>, device: &Device) -> wgpu::Buffer {
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        }
    )
}

pub fn vertex_buffer_desc() -> wgpu::VertexBufferLayout<'static> {
    return Vertex::desc();
}