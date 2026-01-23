use std::ops::Add;

pub mod oct_tree;
pub mod world;
pub mod chunk_retriever;

pub struct whhh {
    pub hello: i32,

}

impl Add for whhh {
    type Output = i32;
    fn add(self, rhs: Self) -> Self::Output {
        self.hello + rhs.hello
    }
}