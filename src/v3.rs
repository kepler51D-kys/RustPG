use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

use crate::base_voxel::CHUNKSIZE;

#[derive(Clone, Copy)]
pub struct V3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl Add for V3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for V3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<u32> for V3 {
    type Output = Self;
    
    fn mul(self, scalar: u32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl V3 {
    pub fn toKey(&self) -> u128 {
        let mut key: u128 = 0;
        key |= self.x as u128;
        key <<= 32;
        key |= self.y as u128;
        key <<= 32;
        key |= self.z as u128;
        return key;
    }
    pub fn toBlockKey(&self) -> usize {
        let mut key: usize = 0;
        key += self.x as usize;
        key *= CHUNKSIZE as usize;
        key += self.y as usize;
        key *= CHUNKSIZE as usize;
        key += self.z as usize;
        return key;    
    }
}
pub const fn new(x: u32, y: u32, z: u32) -> V3 {
    V3 {
        x: x,
        y: y,
        z: z
    }
}