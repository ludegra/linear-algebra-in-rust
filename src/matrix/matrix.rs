pub mod matrix3;
pub mod matrix4;

use num_traits::Num;

pub trait Matrix {
    const SIZE: [usize; 2];
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A 3 by 3 matrix.
pub struct Matrix3<T: Num + Copy> {
    pub values: [[T; 3]; 3]
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A 4 by 4 matrix.
/// 
/// #### Note
/// Some methods and functionality only exists for certian types
pub struct Matrix4<T: Num + Copy> {
    pub values: [[T; 4]; 4]
}