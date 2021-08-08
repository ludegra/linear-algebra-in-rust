use super::Vector;
use super::{Vector2, Vector3, Vector4};
use super::{FromVector1, FromVector2, FromVector3, FromVector4};
use super::{ToVector1, ToVector2, ToVector3, ToVector4};
use super::vector_iterator::Vector1Iterator;

use std::ops::{Add, Sub, Mul, Index, IndexMut};
use num_traits::Num;

use super::Vector1;

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
    fn len(&self) -> usize {
        1
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

impl<T> ToVector1<T> for Vector1<T>
    where T: Num + Copy
{
    /// Returns self
    fn to_vec_1(self) -> Self {
        self
    }
}
impl<T> ToVector2<T> for Vector1<T>
    where T: Num + Copy
{
    /// Returns a 2D vector with the same elements
    /// 
    /// As a 2D vector has more elements than a 1D vector, the following elements wil default to zero:
    /// - y-value
    fn to_vec_2(self) -> Vector2<T> {
        let zero = self.x - self.x;
        Vector2::<T> {
            x: self.x,
            y: zero
        }
    }
}
impl<T> ToVector3<T> for Vector1<T>
    where T: Num + Copy
{
    /// Returns a 3D vector with the same elements
    /// 
    /// As a 3D vector has more elements than a 1D vector, the following elements wil default to zero:
    /// - y-value
    /// - z-value
    fn to_vec_3(self) -> Vector3<T> {
        let zero = self.x - self.x;
        Vector3::<T> {
            x: self.x,
            y: zero,
            z: zero
        }
    }
}
impl<T> ToVector4<T> for Vector1<T>
    where T: Num + Copy
{
    /// Returns a 4D vector with the same elements
    /// 
    /// As a 4D vector has more elements than a 1D vector, the following elements wil default to zero:
    /// - y-value
    /// - z-value
    /// - w-value
    fn to_vec_4(self) -> Vector4<T> {
        let zero = self.x - self.x;
        Vector4::<T> {
            x: self.x,
            y: zero,
            z: zero,
            w: zero
        }
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

impl<T> Index<usize> for Vector1<T>
    where T: Num + Copy
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.x
        }
        else {
            panic!("Index out of bounds");
        }
    }
}
impl<T> IndexMut<usize> for Vector1<T>
    where T: Num + Copy
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.x
        }
        else {
            panic!("Index out of bounds");
        }
    }
}