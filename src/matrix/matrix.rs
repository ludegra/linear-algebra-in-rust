pub mod matrix4;

use num_traits::Num;

pub trait Matrix {
    fn size(&self) -> [usize; 2];
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4<T: Num + Copy> {
    pub values: [[T; 4]; 4]
}