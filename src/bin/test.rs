use linear_algebra::vector::{Vector, vector1::Vector1, vector2::Vector2, vector3::Vector3, vector4::Vector4};
use linear_algebra::matrix::{Matrix, matrix4::Matrix4};

fn main() {
    let identity_matrix = Matrix4::<i32>::identity_matrix();

    let vector = Vector4::new(1, 2, 3, 4);
    let translation_matrix = Matrix4::<i32>::translate(vector);

    println!("{:?}", identity_matrix);
    println!("{:?}", translation_matrix);
    }