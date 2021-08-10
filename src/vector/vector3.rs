use super::Vector;
use super::{Vector1, Vector2, Vector4};
use super::{FromVector1, FromVector2, FromVector3, FromVector4};
use super::{ToVector1, ToVector2, ToVector3, ToVector4};
use super::vector_iterator::Vector3Iterator;

use std::ops::{Add, Sub, Mul, Rem, Index, IndexMut};
use num_traits::Num;

use super::Vector3;

impl<T> Vector3<T>
    where T: Num + Copy
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
impl Vector3<f32> {
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl<T> Vector<T> for Vector3<T> where T: Num + Copy {
    fn sum(&self) -> T {
        self.x + self.y + self.z
    }
    fn len(&self) -> usize {
        3
    }
}

impl<T> IntoIterator for Vector3<T>
    where T: Num + Copy
{
    type Item = T;
    type IntoIter = Vector3Iterator<T>;

    fn into_iter(self) -> Vector3Iterator<T> {
        Vector3Iterator::new(self)
    }
}

impl<T> FromVector1<T> for Vector3<T>
    where T: Num + Copy
{
    /// Creates a new 3D vector from a 1D vector
    /// 
    /// Since a 3D vector has more elements than a 1D vector, the following elements will default to zero:
    /// - y-value
    /// - z-value
    fn from_vec_1(vec: Vector1<T>) -> Self {
        let zero = vec.x - vec.x;
        Self {
            x: vec.x,
            y: zero,
            z: zero
        }
    }
}
impl<T> FromVector2<T> for Vector3<T>
    where T: Num + Copy
{
    /// Creates a new 3D vector from a 2D vector
    /// 
    /// Since a 3D vector has more elements than a 2D vector, the following elements will default to zero:
    /// - z-value
    fn from_vec_2(vec: Vector2<T>) -> Self {
        let zero = vec.x - vec.x;
        Self {
            x: vec.x,
            y: vec.y,
            z: zero
        }
    }
}
impl<T> FromVector3<T> for Vector3<T>
    where T: Num + Copy
{
    /// Creates a new 3D vector from another 3D vector.
    fn from_vec_3(vec: Vector3<T>) -> Self {
        vec
    }
}
impl<T> FromVector4<T> for Vector3<T>
    where T: Num + Copy
{
    /// Creates a new 3D vector from a 4D vector
    /// 
    /// ### Varning!
    /// Since a 3D vector has less elements than a 4D vector, will lose following elements:
    /// - w-value
    fn from_vec_4(vec: Vector4<T>) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z
        }
    }
}

impl<T> ToVector1<T> for Vector3<T>
    where T: Num + Copy
{
    /// Returns a 1D vector with the same elements
    /// 
    /// As a 1D vector has less elements than a 3D vector, the following elements will be lost:
    /// - y-value
    /// - z-value
    fn to_vec_1(self) -> Vector1<T> {
        Vector1::<T> {
            x: self.x,
        }
    }
}
impl<T> ToVector2<T> for Vector3<T>
    where T: Num + Copy
{
    /// Returns a 2D vector with the same elements
    /// 
    /// As a 2D vector has less elements than a 3D vector, the following elements will be lost:
    /// - z-value
    fn to_vec_2(self) -> Vector2<T> {
        Vector2::<T> {
            x: self.x,
            y: self.y,
        }
    }
}
impl<T> ToVector3<T> for Vector3<T>
    where T: Num + Copy
{
    /// Returns self
    fn to_vec_3(self) -> Self {
        self
    }
}
impl<T> ToVector4<T> for Vector3<T>
    where T: Num + Copy
{
    /// Returns a 4D vector with the same elements
    /// 
    /// As a 4D vector has more elements than a 3D vector, the following elements will default to zero:
    /// - z-value
    /// - w-value
    fn to_vec_4(self) -> Vector4<T> {
        let zero = self.x - self.x;
        Vector4::<T> {
            x: self.x,
            y: self.y,
            z: self.z,
            w: zero
        }
    }
}

impl<T> Add<Vector1<T>> for Vector3<T>
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
impl<T> Add<Vector2<T>> for Vector3<T>
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
impl<T> Add<Vector3<T>> for Vector3<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl<T> Add<Vector4<T>> for Vector3<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub<Vector1<T>> for Vector3<T>
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
impl<T> Sub<Vector2<T>> for Vector3<T>
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
impl<T> Sub<Vector3<T>> for Vector3<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl<T> Sub<Vector4<T>> for Vector3<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<Vector1<T>> for Vector3<T>
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
impl<T> Mul<Vector2<T>> for Vector3<T>
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
impl<T> Mul<Vector3<T>> for Vector3<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl<T> Mul<Vector4<T>> for Vector3<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Vector4<T>) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Rem for Vector3<T>
    where T: Num + Copy
{
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x)
        }
    }
}

impl<T> Index<usize> for Vector3<T>
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
        else {
            panic!("Index out of bounds");
        }
    }
}
impl<T> IndexMut<usize> for Vector3<T>
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
        else {
            panic!("Index out of bounds");
        }
    }
}