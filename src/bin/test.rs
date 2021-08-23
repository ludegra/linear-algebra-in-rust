use linear_algebra::angles::*;
use linear_algebra::matrix::*;
use linear_algebra::vector::*;
use linear_algebra::coords::Axis;

use std::f32::consts::PI;

fn main() {
    let mut vec = Vector4::<f32>::new(1.0, 2.0, 1.0, 1.0);

    let mut matrix = Matrix4::<f32>::identity_matrix();

    matrix = matrix.scale(Vector3::new(0.5, 0.5, 0.5));
    matrix = matrix.translate(Vector3::new(1.0, 2.0, 3.0));

    println!("{}", matrix);
    println!("{}", matrix * vec);
}
