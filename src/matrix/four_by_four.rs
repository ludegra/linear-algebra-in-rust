use super::Matrix;
use crate::vector::{Vector,vector4::Vector4};

use num_traits::Num;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FourByFourMatrix<T: Num + Copy> {
    pub values: [[T; 4]; 4]
}
impl<T> FourByFourMatrix<T>
    where T: Num + Copy
{
    pub fn new(values: [[T; 4]; 4]) -> Self {
        Self { values }
    }
    pub fn as_vectors_rows(&self) -> Vec<Vector4<T>> {
        let mut result = Vec::new();
        for row in &self.values {
            result.push(Vector4::new(row[0], row[1], row[2], row[3]));
        }
        result
    }
    pub fn as_vectors_collumns(&self) -> Vec<Vector4<T>> {
        let values = self.values;
        let mut result = Vec::new();
        for i in 0..4 {
            result.push(Vector4::new(values[0][i], values[1][i], values[2][i], values[3][i]))
        }
        result
    }
}
impl FourByFourMatrix<i32> {
    pub fn identity_matrix_int32() -> FourByFourMatrix<i32> {
        let values: [[i32; 4]; 4] = [
            [1, 0, 0, 0], 
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]
        ];
        FourByFourMatrix::<i32> { values }
    }
}
impl FourByFourMatrix<u32> {
    pub fn identity_matrix_uint32() -> FourByFourMatrix<u32> {
        let values: [[u32; 4]; 4] = [
            [1, 0, 0, 0], 
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]
        ];
        FourByFourMatrix::<u32> { values }
    }
}
impl FourByFourMatrix<f32> {
    pub fn identity_matrix_float32() -> FourByFourMatrix<f32> {
        let values: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0], 
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];
        FourByFourMatrix::<f32> { values }
    }
}

impl<T> Matrix for FourByFourMatrix<T>
    where T: Num + Copy
{
    fn size(&self) -> [usize; 2] {
        [4, 4]
    }
}
impl<T> Add for FourByFourMatrix<T>
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

        FourByFourMatrix { values: result }
    }
}
impl<T> Sub for FourByFourMatrix<T>
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

        FourByFourMatrix { values: result }
    }
}
impl<T> Mul<FourByFourMatrix<T>> for FourByFourMatrix<T>
    where T: Num + Copy
{
    type Output = FourByFourMatrix<T>;

    fn mul(self, other: FourByFourMatrix<T>) -> FourByFourMatrix<T> {
       let rows1 = self.as_vectors_rows();
       let collumns2 = other.as_vectors_collumns();
       
       let result: [[T; 4]; 4] = [
           [(rows1[0] * collumns2[0]).sum(), (rows1[0] * collumns2[1]).sum(), (rows1[0] * collumns2[2]).sum(), (rows1[0] * collumns2[3]).sum()],
           [(rows1[1] * collumns2[0]).sum(), (rows1[1] * collumns2[1]).sum(), (rows1[1] * collumns2[2]).sum(), (rows1[1] * collumns2[3]).sum()],
           [(rows1[2] * collumns2[0]).sum(), (rows1[2] * collumns2[1]).sum(), (rows1[2] * collumns2[2]).sum(), (rows1[2] * collumns2[3]).sum()],
           [(rows1[3] * collumns2[0]).sum(), (rows1[3] * collumns2[1]).sum(), (rows1[3] * collumns2[2]).sum(), (rows1[3] * collumns2[3]).sum()],
       ];
       FourByFourMatrix { values: result }
    }
}
impl<T> Mul<Vector4<T> > for FourByFourMatrix<T>
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