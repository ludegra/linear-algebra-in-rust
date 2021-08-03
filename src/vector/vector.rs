pub mod vector1;
pub mod vector2;
pub mod vector3;
pub mod vector4;
pub mod vector_iterator;

use vector1::Vector1;
use vector2::Vector2;
use vector3::Vector3;
use vector4::Vector4;

use num_traits::Num;

pub trait Vector<T> {
    fn sum(&self) -> T;
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