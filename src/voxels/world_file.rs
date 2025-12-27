use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Seek;

use glam::UVec3;

use crate::voxels::base_chunk::{Chunk,CHUNKLEN};

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
    pub fn read_chunk(&mut self, index: UVec3) -> io::Result<Option<Chunk>> {
        match self.chunk_pos.get(&index) {
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
                println!("Chunk not found at index {:?}", index);
                Ok(None)
            }
        }
    }
    // todo: write_chunk
}