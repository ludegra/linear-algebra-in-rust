use super::{vector1::Vector1, vector2::Vector2, vector3::Vector3, vector4::Vector4};

use num_traits::Num;

pub struct Vector1Iterator<T>
    where T: Num + Copy
{
    values: [T; 1],
    count: usize,
}

impl<T> Vector1Iterator<T>
    where T: Num + Copy
{
    pub fn new(vector: Vector1<T>) -> Self {
        let values = [vector.x];
        Self {
            values,
            count: 0,
        }
    }
}

impl<T> Iterator for Vector1Iterator<T>
    where T: Num + Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result;
        if self.count < 1 {
            result = Some(self.values[self.count]);
            self.count += 1;
        }
        else {
            result = None;
        }
        result
    }
}

pub struct Vector2Iterator<T>
    where T: Num + Copy
{
    values: [T; 2],
    count: usize,
}

impl<T> Vector2Iterator<T>
    where T: Num + Copy
{
    pub fn new(vector: Vector2<T>) -> Self {
        let values = [vector.x, vector.y];
        Self {
            values,
            count: 0,
        }
    }
}

impl<T> Iterator for Vector2Iterator<T>
    where T: Num + Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result;
        if self.count < 2 {
            result = Some(self.values[self.count]);
            self.count += 1;
        }
        else {
            result = None;
        }
        result
    }
}

pub struct Vector3Iterator<T>
    where T: Num + Copy
{
    values: [T; 3],
    count: usize,
}

impl<T> Vector3Iterator<T>
    where T: Num + Copy
{
    pub fn new(vector: Vector3<T>) -> Self {
        let values = [vector.x, vector.y, vector.z];
        Self {
            values,
            count: 0,
        }
    }
}

impl<T> Iterator for Vector3Iterator<T>
    where T: Num + Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result;
        if self.count < 3 {
            result = Some(self.values[self.count]);
            self.count += 1;
        }
        else {
            result = None;
        }
        result
    }
}

pub struct Vector4Iterator<T>
    where T: Num + Copy
{
    values: [T; 4],
    count: usize,
}

impl<T> Vector4Iterator<T>
    where T: Num + Copy
{
    pub fn new(vector: Vector4<T>) -> Self {
        let values = [vector.x, vector.y, vector.z, vector.w];
        Self {
            values,
            count: 0,
        }
    }
}

impl<T> Iterator for Vector4Iterator<T>
    where T: Num + Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result;
        if self.count < 4 {
            result = Some(self.values[self.count]);
            self.count += 1;
        }
        else {
            result = None;
        }
        result
    }
}