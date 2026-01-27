use glam::UVec3;
use std::{fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom}};

// theoretical max range of world:
const MAX_WORLD_SIZE: usize = 1_048_576;

use crate::dual_contouring::oct_tree::OctTree;
// pub struct OctTreeStore {
//     pub len: u32,
//     pub distances: [[[f32; 2]; 2]; 2]>,
// }
pub struct WorldFileManager {
    // pub chunk_generator: todo!()
    filename: String,
    address_cache: Vec<u64>,
    chunk_file: File,
}

impl WorldFileManager {
    pub fn get_index(&self, index:UVec3) -> u64 {
        self.address_cache[
            index.x as usize * MAX_WORLD_SIZE * MAX_WORLD_SIZE +
            index.y as usize * MAX_WORLD_SIZE +
            index.z as usize
        ]
    }
    pub fn retrieve_chunk(&mut self, index: UVec3) -> OctTree {
        let file_index = self.get_index(index);
        self.chunk_file.seek(SeekFrom::Start(file_index)).unwrap();
        let mut buf = vec![0; 4];
        self.chunk_file.read_exact(&mut buf).unwrap();

        todo!()
    }
    pub fn open_world(world_name: String) -> Self {
        
        let mut metadata: File = OpenOptions::new()
            .read(true)
            .open(format!("worlds/{}/metadata",world_name))
            .unwrap();
        let chunk_file: File = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("worlds/{}/chunkdata",world_name))
            .unwrap();

        let buffer: &mut [u8] = &mut[];
        metadata.read(buffer).unwrap();
        Self {
            filename: world_name,
            address_cache: bytemuck::cast_slice(buffer).to_vec(),
            chunk_file,
        }
    }
}

/*

*/