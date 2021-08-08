use super::Vector;
use super::{Vector1, Vector2, Vector3};
use super::{FromVector1, FromVector2, FromVector3, FromVector4};
use super::{ToVector1, ToVector2, ToVector3, ToVector4};
use super::vector_iterator::Vector4Iterator;

use std::ops::{Add, Sub, Mul, Index, IndexMut};
use std::iter::IntoIterator;
use num_traits::Num;

use super::Vector4;

impl<T> Vector4<T>
    where T: Num + Copy
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T> Vector<T> for Vector4<T> where T: Num + Copy {
    fn sum(&self) -> T {
        self.x + self.y + self.z + self.w
    }
    fn len(&self) -> usize {
        4
    }
}

impl<T> IntoIterator for Vector4<T>
    where T: Num + Copy
{
    type Item = T;
    type IntoIter = Vector4Iterator<T>;

    fn into_iter(self) -> Vector4Iterator<T> {
        Vector4Iterator::new(self)
    }
}

impl<T> FromVector1<T> for Vector4<T>
    where T: Num + Copy
{
    /// Creates a new 4D vector from a 1D vector
    /// 
    /// Since a 4D vector has more elements than a 1D vector, the following elements will default to zero:
    /// - y-value
    /// - z-value
    /// - w-value
    fn from_vec_1(vec: Vector1<T>) -> Self {
        let zero = vec.x - vec.x;
        Self {
            x: vec.x,
            y: zero,
            z: zero,
            w: zero
        }
    }
}
impl<T> FromVector2<T> for Vector4<T>
    where T: Num + Copy
{
    /// Creates a new 4D vector from a 2D vector
    /// 
    /// Since a 4D vector has more elements than a 2D vector, the following elements will default to zero:
    /// - z-value
    /// - w-value
    fn from_vec_2(vec: Vector2<T>) -> Self {
        let zero = vec.x - vec.x;
        Self {
            x: vec.x,
            y: vec.y,
            z: zero,
            w: zero
        }
    }
}
impl<T> FromVector3<T> for Vector4<T>
    where T: Num + Copy
{
    /// Creates a new 4D vector from a 3D vector
    /// 
    /// Since a 4D vector has more elements than a 3D vector, the following elements will default to zero:
    /// - w-value
    fn from_vec_3(vec: Vector3<T>) -> Self {
        let zero = vec.x - vec.x;
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: zero
        }
    }
}
impl<T> FromVector4<T> for Vector4<T>
    where T: Num + Copy
{
    /// Creates a new 4D vector from another 3D vector.
    fn from_vec_4(vec: Vector4<T>) -> Self {
        vec
    }
}

impl<T> ToVector1<T> for Vector4<T>
    where T: Num + Copy
{
    /// Returns a 1D vector with the same elements
    /// 
    /// As a 1D vector has less elements than a 4D vector, the following elements will be lost:
    /// - y-value
    /// - z-value
    /// - w-value
    fn to_vec_1(self) -> Vector1<T> {
        Vector1::<T> {
            x: self.x,
        }
    }
}
impl<T> ToVector2<T> for Vector4<T>
    where T: Num + Copy
{
    /// Returns a 2D vector with the same elements
    /// 
    /// As a 2D vector has less elements than a 4D vector, the following elements will be lost:
    /// - z-value
    /// - w-value
    fn to_vec_2(self) -> Vector2<T> {
        Vector2::<T> {
            x: self.x,
            y: self.y,
        }
    }
}
impl<T> ToVector3<T> for Vector4<T>
    where T: Num + Copy
{
    /// Returns a 3D vector with the same elements
    /// 
    /// As a 3D vector has less elements than a 4D vector, the following elements will be lost:
    /// - w-value
    fn to_vec_3(self) -> Vector3<T> {
        Vector3::<T> {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
impl<T> ToVector4<T> for Vector4<T>
    where T: Num + Copy
{
    /// Returns self
    fn to_vec_4(self) -> Self {
        self
    }
}

impl<T> Add<Vector1<T>> for Vector4<T>
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
impl<T> Add<Vector2<T>> for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            ..self
        }
    }
}
impl<T> Add<Vector3<T>> for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            ..self
        }
    }
}
impl<T> Add<Vector4<T>> for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl<T> Sub<Vector1<T>> for Vector4<T>
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
impl<T> Sub<Vector2<T>> for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            ..self
        }
    }
}
impl<T> Sub<Vector3<T>> for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            ..self
        }
    }
}
impl<T> Sub for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl<T> Mul<Vector1<T>> for Vector4<T>
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
impl<T> Mul<Vector2<T>> for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            ..self
        }
    }
}
impl<T> Mul<Vector3<T>> for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            ..self
        }
    }
}
impl<T> Mul<Vector4<T>> for Vector4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w
        }
    }
}

impl<T> Index<usize> for Vector4<T>
    where T: Num + Copy
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.x
        }
        else if index == 1 {
            &self.y
        }
        else if index == 2 {
            &self.z
        }
        else if index == 3 {
            &self.w
        }
        else {
            panic!("Index out of bounds");
        }
    }
}
impl<T> IndexMut<usize> for Vector4<T>
    where T: Num + Copy
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.x
        }
        else if index == 1 {
            &mut self.y
        }
        else if index == 2 {
            &mut self.z
        }
        else if index == 3 {
            &mut self.w
        }
        else {
            panic!("Index out of bounds");
        }
    }
}