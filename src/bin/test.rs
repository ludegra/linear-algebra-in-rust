use linear_algebra::angles::{degrees_to_radians, radians_to_degrees};
use linear_algebra::matrix::Matrix4;
use linear_algebra::vector::Vector4;
use linear_algebra::Axis;

use std::f32::consts::PI;

fn main() {
    let _identity_matrix = Matrix4::<i32>::identity_matrix();

    let vector = Vector4::new(1, 2, 3, 4);
    let _translation_matrix = Matrix4::<i32>::translate(vector);

    let _rotation_matrix = Matrix4::rotation(Axis::X, PI / 3.0);

    println!("{}", degrees_to_radians(1.0));
    println!("{}", radians_to_degrees(0.0174532925))
}
