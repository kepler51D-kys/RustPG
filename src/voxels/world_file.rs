use std::collections::HashMap;
use std::fs;
use std::io;
// use std::io::{Read,Seek;

use glam::UVec3;

use crate::voxels::base_chunk::ChunkState;
use crate::voxels::base_chunk::{Chunk,CHUNKLEN};
use crate::voxels::base_voxel::BlockID;

pub struct FileManager {
    chunk_pos: HashMap<UVec3,u64>,
    world_name: String,
    world_file: fs::File,
    // worker_thread: thread::Thread,
}
impl Default for FileManager {
    fn default() -> Self {
        return Self {
            chunk_pos: HashMap::new(),
            world_name: String::from("worlds/test.world"),
            world_file: fs::OpenOptions::new().read(true).write(true).create(true).open("worlds/test.world").unwrap()
        };
    }
}
impl FileManager {
    pub fn new(name: String, ) -> Self {
        Self {
            world_name: name.clone(),
            world_file: fs::OpenOptions::new().read(true).write(true).create(true).open(name).unwrap(),
            chunk_pos: HashMap::new()
        }
    }
    pub fn open_file(&mut self, file_name: String) -> io::Result<()> {
        self.world_name = file_name;
        self.world_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.world_name)?;
        
        return Ok(());
    }
    pub fn read_chunk(&mut self, _index: UVec3) -> io::Result<Chunk> {
        // let file_index: u64 = *self.chunk_pos.get(&index).unwrap();
        // self.world_file.seek(io::SeekFrom::Start(file_index))?;

        let mut ret_chunk: Chunk = Chunk::default();
        // let u8_buffer = unsafe {
        //     std::slice::from_raw_parts_mut(
        //         ret_chunk.data.as_mut_ptr() as *mut u8,
        //         CHUNKLEN * std::mem::size_of::<u16>()
        //     )
        // };
        ret_chunk.data = [BlockID::Stone; CHUNKLEN];
        ret_chunk.state = ChunkState::MeshDirty;
        // self.world_file.read_exact(u8_buffer)?;
        Ok(ret_chunk)
    }
    // todo: write_chunk
}