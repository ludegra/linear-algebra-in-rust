pub mod matrix4;

pub trait Matrix {
    fn size(&self) -> [usize; 2];
}