use super::Vector;
use super::{vector1::Vector1, vector3::Vector3, vector4::Vector4};
use super::{FromVector1, FromVector2, FromVector3, FromVector4};
use super::vector_iterator::Vector2Iterator;

use std::ops::{Add, Sub, Mul, Rem};
use num_traits::Num;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T>
    where T: Num + Copy
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
    where T: Num + Copy
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}

impl<T> Vector<T> for Vector2<T> where T: Num + Copy {
    fn sum(&self) -> T {
        self.x + self.y
    }
}

impl<T> IntoIterator for Vector2<T>
    where T: Num + Copy
{
    type Item = T;
    type IntoIter = Vector2Iterator<T>;

    fn into_iter(self) -> Vector2Iterator<T> {
        Vector2Iterator::new(self)
    }
}

impl<T> FromVector1<T> for Vector2<T>
    where T: Num + Copy
{
    /// Creates a new 2D vector from a 1D vector
    /// 
    /// Since a 2D vector has more elements than a 1D vector, the following elements will default to zero:
    /// - y-value
    fn from_vec_1(vec: Vector1<T>) -> Self {
        let zero = vec.x - vec.x;
        Self {
            x: vec.x,
            y: zero
        }
    }
}
impl<T> FromVector2<T> for Vector2<T>
    where T: Num + Copy
{
    /// Creates a new 2D vector from another 2D vector.
    fn from_vec_2(vec: Vector2<T>) -> Self {
        vec
    }
}
impl<T> FromVector3<T> for Vector2<T>
    where T: Num + Copy
{
    /// Creates a new 2D vector from a 3D vector
    /// 
    /// ### Varning!
    /// Since a 2D vector has less elements than a 3D vector, will lose following elements:
    /// - z-value
    fn from_vec_3(vec: Vector3<T>) -> Self {
        Self {
            x: vec.x,
            y: vec.y
        }
    }
}
impl<T> FromVector4<T> for Vector2<T>
    where T: Num + Copy 
{
    /// Creates a new 2D vector from a 4D vector
    /// 
    /// ### Varning!
    /// Since a 2D vector has less elements than a 4D vector, will lose following elements:
    /// - z-value
    /// - w-value
    fn from_vec_4(vec: Vector4<T>) -> Self {
        Self { 
            x: vec.x,
            y: vec.y
        }
    }
}

impl<T> Add<Vector1<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector1<T>) -> Self {
        Self {
            x: self.x + other.x,
            ..self
        }
    }
}
impl<T> Add<Vector2<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T> Add<Vector3<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T> Add<Vector4<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub<Vector1<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector1<T>) -> Self {
        Self {
            x: self.x - other.x,
            ..self
        }
    }
}
impl<T> Sub<Vector2<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T> Sub<Vector3<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T> Sub<Vector4<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<Vector1<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector1<T>) -> Self {
        Self {
            x: self.x * other.x,
            ..self
        }
    }
}
impl<T> Mul<Vector2<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl<T> Mul<Vector3<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl<T> Mul<Vector4<T>> for Vector2<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Rem for Vector2<T>
    where T: Num + Copy
{
    type Output = Vector3<T>;

    fn rem(self, other: Self) -> Vector3<T> {
        Vector3::<T> {
            x: self.x - self.x,
            y: self.y - self.y,
            z: (self.x * other.y) - (self.y * other.x)
        }
    }
}