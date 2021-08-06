use linear_algebra::vector::{vector4::Vector4};
use linear_algebra::matrix::{matrix4::Matrix4};
use linear_algebra::Axis;

use std::f32::consts::PI;

fn main() {
    let identity_matrix = Matrix4::<i32>::identity_matrix();

    let vector = Vector4::new(1, 2, 3, 4);
    let translation_matrix = Matrix4::<i32>::translate(vector);

    let rotation_matrix = Matrix4::rotation(Axis::X, PI/3.0);

    println!("{:?}", identity_matrix);
    println!("{:?}", translation_matrix);
    println!("{:?}", rotation_matrix);
    }