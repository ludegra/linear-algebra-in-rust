use std::f32::consts::PI;

pub fn degrees_to_radians(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

pub fn radians_to_degrees(angle: f32) -> f32 {
    angle / (PI / 180.0)
}