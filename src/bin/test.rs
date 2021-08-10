use linear_algebra::angles::*;
use linear_algebra::matrix::*;
use linear_algebra::vector::*;
use linear_algebra::coords::Axis;

use std::f32::consts::PI;

fn main() {
    let vec = Vector4::<f32>::new(1.0, 2.0, 3.0, 1.0);

    let matrix = Matrix4::<f32>::scaling_matrix(Vector3::new(30.0, 2.0, 100.0));

    println!("hellu {}", matrix);
}
