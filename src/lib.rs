#[path = "vector/vector.rs"]
pub mod vector;

#[path = "matrix/matrix.rs"]
pub mod matrix;

pub mod angles;

pub enum Axis {
    X,
    Y,
    Z
}