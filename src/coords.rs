use crate::vector::Vector3;

pub struct Axis;

impl Axis {
    pub const X: Vector3<f32> = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Vector3<f32> = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
}