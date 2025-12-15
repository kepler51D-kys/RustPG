use std::fs;
use std::io;
use std::io::Read;
use std::io::Seek;
// use std::thread;

// use bevy::mesh::Mesh;
use bevy::platform::collections::HashMap;

// use crate::base_voxel::{BlockID};
use crate::base_chunk::{Chunk,CHUNKLEN};
use crate::v3;

pub struct Manager {
    chunk_pos: HashMap<u128,u64>,
    world_name: String,
    world_file: fs::File,
    // worker_thread: thread::Thread,
}
impl Default for Manager {
    fn default() -> Self {
        return Self {
            chunk_pos: HashMap::new(),
            world_name: String::from("worlds/test.world"),
            world_file: fs::OpenOptions::new().read(true).write(true).create(true).open("worlds/test.world").unwrap()
        };
    }
}
impl Manager {
    pub fn init(name: String, ) -> Self {

        return Self {
            world_name: name.clone(),
            world_file: fs::OpenOptions::new().read(true).write(true).create(true).open(name).unwrap(),
            chunk_pos: HashMap::new()
        };
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
    pub fn read_chunk(&mut self, index: v3::V3) -> io::Result<Option<Chunk>> {
        let key = index.to_key();
        match self.chunk_pos.get(&key) {
            Some(&file_pos) => {
                self.world_file.seek(io::SeekFrom::Start(file_pos))?;

                let mut ret_chunk: Chunk = Chunk::default();
                let u8_buffer = unsafe {
                    std::slice::from_raw_parts_mut(
                        ret_chunk.data.as_mut_ptr() as *mut u8,
                        CHUNKLEN * std::mem::size_of::<u16>()
                    )
                };
                self.world_file.read_exact(u8_buffer)?;

                Ok(Some(ret_chunk))
            }
            None => {
                println!("Chunk not found at index {:?}", index.to_key());
                Ok(None)
            }
        }
    }
    // todo: write_chunk
}