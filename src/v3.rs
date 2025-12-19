use std::{fmt::Debug, hash::Hash, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use bevy::math::UVec3;

use crate::base_chunk::CHUNKSIZE;

pub fn get_block_index(vec: UVec3) -> usize {
    return (vec.x*CHUNKSIZE as u32*CHUNKSIZE as u32 + vec.y*CHUNKSIZE as u32+vec.z) as usize;
}

#[derive(Clone, Copy)]
pub struct Fvec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Add for Fvec {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl AddAssign for Fvec {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Fvec {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl MulAssign<f32> for Fvec {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f32> for Fvec {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl Sub for Fvec {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<f32> for Fvec {
    type Output = Self;
    
    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl Div<f32> for Fvec {
    type Output = Self;
    
    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl From<V3> for Fvec {
    fn from(v: V3) -> Self {
        Self {
            x: v.x as f32,
            y: v.y as f32,
            z: v.z as f32,
        }
    }
}

impl From<Fvec> for V3 {
    fn from(f: Fvec) -> Self {
        Self {
            x: f.x as u32,
            y: f.y as u32,
            z: f.z as u32,
        }
    }
}
#[derive(Clone, Copy,Hash,Eq,PartialEq)]
pub struct V3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl Debug for V3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x: {}, y: {}, z: {}]", self.x, self.y, self.z)
    }
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
impl AddAssign for V3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for V3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl MulAssign<u32> for V3 {
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<u32> for V3 {
    fn div_assign(&mut self, rhs: u32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
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
impl Div<u32> for V3 {
    type Output = Self;
    
    fn div(self, scalar: u32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl V3 {
    pub fn to_block_key(&self) -> usize {
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