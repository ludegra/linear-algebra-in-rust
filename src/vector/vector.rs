mod vector1;
mod vector2;
mod vector3;
mod vector4;
pub mod vector_iterator;

use num_traits::Num;

pub trait Vector<T>
    where T: Num + Copy
{
    fn sum(&self) -> T;
    fn len(&self) -> usize;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector1<T>
    where T: Num + Copy
{
    pub x: T
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T>
    where T: Num + Copy
{
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T>
    where T: Num + Copy
{
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4<T>
    where T: Num + Copy
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub trait FromVector1<T>
    where T: Num + Copy
{
    fn from_vec_1(vec: Vector1<T>) -> Self;
}
pub trait FromVector2<T>
    where T: Num + Copy
{
    fn from_vec_2(vec: Vector2<T>) -> Self;
}
pub trait FromVector3<T>
    where T: Num + Copy
{
    fn from_vec_3(vec: Vector3<T>) -> Self;
}
pub trait FromVector4<T>
    where T: Num + Copy
{
    fn from_vec_4(vec: Vector4<T>) -> Self;
}

pub trait ToVector1<T>
    where T: Num + Copy
{
    fn to_vec_1(self) -> Vector1<T>;
}
pub trait ToVector2<T>
    where T: Num + Copy
{
    fn to_vec_2(self) -> Vector2<T>;
}
pub trait ToVector3<T>
    where T: Num + Copy
{
    fn to_vec_3(self) -> Vector3<T>;
}
pub trait ToVector4<T>
    where T: Num + Copy
{
    fn to_vec_4(self) -> Vector4<T>;
}