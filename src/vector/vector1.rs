use super::Vector;
use super::{vector2::Vector2, vector3::Vector3, vector4::Vector4};
use super::{FromVector1, FromVector2, FromVector3, FromVector4};
use super::vector_iterator::Vector1Iterator;

use std::ops::{Add, Sub, Mul};
use num_traits::Num;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector1<T>
    where T: Num + Copy
{
    pub x: T
}

impl<T> Vector1<T>
    where T: Num + Copy
{
    pub fn new(x: T) -> Self {
        Self { x }
    }
}

impl<T> Vector<T> for Vector1<T> where T: Num + Copy {
    fn sum(&self) -> T {
        self.x
    }
}

impl<T> IntoIterator for Vector1<T>
    where T: Num + Copy
{
    type Item = T;
    type IntoIter = Vector1Iterator<T>;

    fn into_iter(self) -> Vector1Iterator<T> {
        Vector1Iterator::new(self)
    }
}

impl<T> FromVector1<T> for Vector1<T>
    where T: Num + Copy
{
    /// Creates a new 1D vector from another 1D vector.
    fn from_vec_1(vec: Vector1<T>) -> Self {
        vec
    }
}
impl<T> FromVector2<T> for Vector1<T>
    where T: Num + Copy
{
    /// Creates a new 1D vector from a 2D vector
    /// 
    /// ### Varning!
    /// Since a 1D vector has less elements than a 2D vector, it will lose following elements:
    /// - y-value
    fn from_vec_2(vec: Vector2<T>) -> Self {
        Self { x: vec.x }
    }
}
impl<T> FromVector3<T> for Vector1<T>
    where T: Num + Copy
{
    /// Creates a new 1D vector from a 3D vector
    /// 
    /// ### Varning!
    /// Since a 1D vector has less elements than a 3D vector, it will lose following elements:
    /// - y-value
    /// - z-value
    fn from_vec_3(vec: Vector3<T>) -> Self {
        Self { x: vec.x }
    }
}
impl<T> FromVector4<T> for Vector1<T>
    where T: Num + Copy
{
    /// Creates a new 1D vector from a 4D vector
    /// 
    /// ### Varning!
    /// Since a 1D vector has less elements than a 4D vector, it will lose following elements:
    /// - y-value
    /// - z-value
    /// - w-value
    fn from_vec_4(vec: Vector4<T>) -> Self {
        Self { x: vec.x }
    }
}

impl<T> Add<Vector1<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector1<T>) -> Self {
        Self {
            x: self.x + other.x,
        }
    }
}
impl<T> Add<Vector2<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x + other.x,
        }
    }
}
impl<T> Add<Vector3<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x + other.x,
        }
    }
}
impl<T> Add<Vector4<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x + other.x,
        }
    }
}

impl<T> Sub<Vector1<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector1<T>) -> Self {
        Self {
            x: self.x - other.x,
        }
    }
}
impl<T> Sub<Vector2<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x - other.x,
        }
    }
}
impl<T> Sub<Vector3<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x - other.x,
        }
    }
}
impl<T> Sub<Vector4<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x - other.x,
        }
    }
}

impl<T> Mul<Vector1<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector1<T>) -> Self {
        Self {
            x: self.x * other.x,
        }
    }
}
impl<T> Mul<Vector2<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x * other.x,
        }
    }
}
impl<T> Mul<Vector3<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x * other.x,
        }
    }
}
impl<T> Mul<Vector4<T>> for Vector1<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x * other.x,
        }
    }
}