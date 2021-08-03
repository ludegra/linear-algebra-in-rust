pub mod four_by_four;

pub trait Matrix {
    fn size(&self) -> [usize; 2];
}