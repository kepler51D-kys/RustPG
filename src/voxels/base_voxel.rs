pub const WORLDSIZE: u32 = 32;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum BlockID {
    Air,
    Stone,
    Dirt,
}

impl BlockID {
    pub fn to_num(self) -> u16 {
        return self as u16;
    }
}
impl PartialEq for BlockID {
    fn eq(&self, other: &Self) -> bool {
        return (*self as u16) == (*other as u16);
    }
}