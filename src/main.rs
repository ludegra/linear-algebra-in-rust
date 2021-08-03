#[path = "vector/vector.rs"]
mod vector;
use vector::{FromVector1, FromVector2, FromVector3, FromVector4};
use vector::{vector1::Vector1, vector2::Vector2, vector3::Vector3, vector4::Vector4};

#[path = "matrix/matrix.rs"]
mod matrix;
use matrix::four_by_four::FourByFourMatrix;

fn main() {
    let values1 = [
        [11, 12, 13, 14],
        [21, 22, 23, 24],
        [31, 32, 33, 34],
        [41, 42, 43, 44],
    ];
    let matrix1 = FourByFourMatrix::new(values1);

    let values2 = [
        [51, 52, 53, 54],
        [61, 62, 63, 64],
        [71, 72, 73, 74],
        [81, 82, 83, 84],
    ];
    let matrix2 = FourByFourMatrix::new(values2);

    println!("{:?}", matrix1 * matrix2);
    println!("{:?}", matrix2 * matrix1);
}