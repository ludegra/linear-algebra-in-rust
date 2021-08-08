use super::Matrix;
use crate::vector::{Vector, Vector4};
use crate::vector::{ToVector3};
use crate::Axis;

use num_traits::Num;
use std::ops::{Add, Mul, Sub, Index, IndexMut};

use super::Matrix4;

impl<T> Matrix4<T>
    where T: Num + Copy
{
    pub fn new(values: [[T; 4]; 4]) -> Self {
        Matrix4 { values }
    }
    pub fn as_vectors_rows(&self) -> [Vector4<T>; 4] {
        let rows = &self.values;
        let result = [
            Vector4::new(rows[0][0], rows[0][1], rows[0][2], rows[0][3]),
            Vector4::new(rows[1][0], rows[1][1], rows[1][2], rows[1][3]),
            Vector4::new(rows[2][0], rows[2][1], rows[2][2], rows[2][3]),
            Vector4::new(rows[3][0], rows[3][1], rows[3][2], rows[3][3]),
        ];
        result
    }
    pub fn as_vectors_collumns(&self) -> [Vector4<T>; 4] {
        let rows = self.values;
        let result = [
            Vector4::new(rows[0][0], rows[1][0], rows[2][0], rows[3][0]),
            Vector4::new(rows[0][1], rows[1][1], rows[2][1], rows[3][1]),
            Vector4::new(rows[0][2], rows[1][2], rows[2][2], rows[3][2]),
            Vector4::new(rows[0][3], rows[1][3], rows[2][3], rows[3][3]),
        ];
        result
    }
}
impl Matrix4<i32> {
    /// Returns a 4x4 identity matrix
    pub fn identity_matrix() -> Matrix4<i32> {
        let values: [[i32; 4]; 4] = [
            [1, 0, 0, 0], 
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]
        ];
        Matrix4::<i32> { values }
    }
    /// Returns a 4x4 translation matrix from the given vector
    pub fn translate<V: ToVector3<i32>>(vector: V) -> Self {
        let mut matrix = Self::identity_matrix();
        let vector = vector.to_vec_3();
        for i in 0..3 {
            matrix[[3, i]] = vector[i];
        }
        matrix
    }
}
impl Matrix4<u32> {
    /// Returns a 4x4 identity matrix
    pub fn identity_matrix() -> Matrix4<u32> {
        let values: [[u32; 4]; 4] = [
            [1, 0, 0, 0], 
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]
        ];
        Matrix4::<u32> { values }
    }
    /// Returns a 4x4 translation matrix from the given vector
    pub fn translate<V: ToVector3<u32>>(vector: V) -> Self {
        let mut matrix = Self::identity_matrix();
        let vector = vector.to_vec_3();
        for i in 0..3 {
            matrix[[3, i]] = vector[i];
        }
        matrix
    }
}
impl Matrix4<f32> {
    /// Returns a 4x4 identity matrix
    pub fn identity_matrix() -> Matrix4<f32> {
        let values: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0], 
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];
        Matrix4::<f32> { values }
    }
    /// Returns a 4x4 translation matrix from the given vector
    pub fn translate<V: ToVector3<f32>>(vector: V) -> Self {
        let mut matrix = Self::identity_matrix();
        let vector = vector.to_vec_3();
        for i in 0..3 {
            matrix[[3, i]] = vector[i];
        }
        matrix
    }
    /// Returns a 4x4 rotation matrix
    /// 
    /// axis: axis around witch the rotation takes place
    /// degrees: degrees for rotation in radians
    pub fn rotation(axis: Axis, degrees: f32) -> Self {
        let (cos, sin) = (degrees.cos(), degrees.sin());
        let values = match axis {
            Axis::X => {
                let matrix: [[f32; 4]; 4] = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, cos, -sin, 0.0],
                    [0.0, sin, cos, 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ];
                matrix
            }
            Axis::Y => {
                let matrix: [[f32; 4]; 4] = [
                    [cos, 0.0, sin, 0.0], 
                    [0.0, 1.0, 0.0, 0.0],
                    [-sin, 0.0, cos, 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ];
                matrix
            }
            Axis::Z => {
                let matrix: [[f32; 4]; 4] = [
                    [cos, -sin, 0.0, 0.0],
                    [sin, cos, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ];
                matrix
            }
        };
        Self { values }
    }
}

impl<T> Matrix for Matrix4<T>
    where T: Num + Copy
{
    fn size(&self) -> [usize; 2] {
        [4, 4]
    }
}
impl<T> Add for Matrix4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (values1, values2) = (self.values, other.values);
        let result: [[T; 4]; 4] = [
            [values1[0][0] + values2[0][0], values1[0][1] + values2[0][1], values1[0][2] + values2[0][2], values1[0][3] + values2[0][3]],
            [values1[1][0] + values2[1][0], values1[1][1] + values2[1][1], values1[1][2] + values2[1][2], values1[1][3] + values2[1][3]],
            [values1[2][0] + values2[2][0], values1[2][1] + values2[2][1], values1[2][2] + values2[2][2], values1[2][3] + values2[2][3]],
            [values1[3][0] + values2[3][0], values1[3][1] + values2[3][1], values1[3][2] + values2[3][2], values1[3][3] + values2[3][3]],
        ];

        Matrix4 { values: result }
    }
}
impl<T> Sub for Matrix4<T>
    where T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (values1, values2) = (self.values, other.values);
        let result: [[T; 4]; 4] = [
            [values1[0][0] - values2[0][0], values1[0][1] - values2[0][1], values1[0][2] - values2[0][2], values1[0][3] - values2[0][3]],
            [values1[1][0] - values2[1][0], values1[1][1] - values2[1][1], values1[1][2] - values2[1][2], values1[1][3] - values2[1][3]],
            [values1[2][0] - values2[2][0], values1[2][1] - values2[2][1], values1[2][2] - values2[2][2], values1[2][3] - values2[2][3]],
            [values1[3][0] - values2[3][0], values1[3][1] - values2[3][1], values1[3][2] - values2[3][2], values1[3][3] - values2[3][3]],
        ];

        Matrix4 { values: result }
    }
}
impl<T> Mul<Matrix4<T>> for Matrix4<T>
    where T: Num + Copy
{
    type Output = Matrix4<T>;

    fn mul(self, other: Matrix4<T>) -> Matrix4<T> {
       let rows1 = self.as_vectors_rows();
       let collumns2 = other.as_vectors_collumns();
       
       let result: [[T; 4]; 4] = [
           [(rows1[0] * collumns2[0]).sum(), (rows1[0] * collumns2[1]).sum(), (rows1[0] * collumns2[2]).sum(), (rows1[0] * collumns2[3]).sum()],
           [(rows1[1] * collumns2[0]).sum(), (rows1[1] * collumns2[1]).sum(), (rows1[1] * collumns2[2]).sum(), (rows1[1] * collumns2[3]).sum()],
           [(rows1[2] * collumns2[0]).sum(), (rows1[2] * collumns2[1]).sum(), (rows1[2] * collumns2[2]).sum(), (rows1[2] * collumns2[3]).sum()],
           [(rows1[3] * collumns2[0]).sum(), (rows1[3] * collumns2[1]).sum(), (rows1[3] * collumns2[2]).sum(), (rows1[3] * collumns2[3]).sum()],
       ];
       Matrix4 { values: result }
    }
}
impl<T> Mul<Vector4<T> > for Matrix4<T>
    where T: Num + Copy
{
    type Output = Vector4<T>;

    fn mul(self, other: Vector4<T>) -> Vector4<T> {
        let rows = self.as_vectors_rows();
        let result: Vector4<T> = Vector4::new(
            (rows[0] * other).sum(),
            (rows[1] * other).sum(),
            (rows[2] * other).sum(),
            (rows[3] * other).sum(),
        );
        result
    }
}

impl<T> Index<[usize; 2]> for Matrix4<T>
    where T: Num + Copy
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.values[index[1]][index[0]]
    }
}
impl<T> IndexMut<[usize; 2]> for Matrix4<T>
    where T: Num + Copy
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.values[index[1]][index[0]]
    }
}