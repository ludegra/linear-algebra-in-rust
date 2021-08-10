use super::Matrix;
use crate::vector::{Vector, Vector3};
use crate::vector::{ToVector3};

use num_traits::Num;
use std::ops::{Add, Mul, Sub, Index, IndexMut};

use super::Matrix3;

impl<T> Matrix3<T>
where
    T: Num + Copy
{
    pub fn new(values: [[T; 3]; 3]) -> Self {
        Self { values }
    }
    pub fn as_vectors_rows(&self) -> [Vector3<T>; 3] {
        let result = [
            Vector3::new(self[[0, 0]], self[[1, 0]], self[[2, 0]]),
            Vector3::new(self[[0, 1]], self[[1, 1]], self[[2, 1]]),
            Vector3::new(self[[0, 2]], self[[1, 2]], self[[2, 2]])
        ];
        result
    }
    pub fn as_vectors_collumns(&self) -> [Vector3<T>; 3] {
        let result = [
            Vector3::new(self[[0, 0]], self[[0, 1]], self[[0, 2]]),
            Vector3::new(self[[1, 0]], self[[1, 1]], self[[1, 2]]),
            Vector3::new(self[[2, 0]], self[[2, 1]], self[[2, 2]])
        ];
        result
    }
}
impl Matrix3<i32> {
    /// Returns a 3x3 identity matrix
    pub fn identity_matrix() -> Self {
        let values: [[i32; 3]; 3] = [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1]
        ];
        Self { values }
    }
}
impl Matrix3<u32> {
    /// Returns a 3x3 identity matrix
    pub fn identity_matrix() -> Self {
        let values: [[u32; 3]; 3] = [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1]
        ];
        Self { values }
    }
}
impl Matrix3<f32> {
    /// Returns a 3x3 identity matrix
    pub fn identity_matrix() -> Self {
        let values: [[f32; 3]; 3] = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ];
        Self { values }
    }
    pub fn rotation_matrix(degrees: f32, u: Vector3<f32>) -> Self {
        let (cos, sin) = (degrees.cos(), degrees.sin());
        
        let mut values = [
            [cos + u.x.powi(2) * (1.0 - cos), u.x * u.y * (1.0 - cos) - u.z * sin, u.x * u.z * (1.0 - cos) + u.y * sin],
            [u.y * u.x * (1.0 - cos) + u.z * sin, cos + u.y.powi(2) * (1.0 - cos), u.y * u.z * (1.0 - cos) - u.x * sin],
            [u.z * u.x * (1.0 - cos) - u.y * sin, u.z * u.y * (1.0 - cos) + u.x * sin, cos + u.z.powi(2) * (1.0 - cos)],
        ];

        for row in values.iter_mut() {
            for element in row.iter_mut() {
                if *element < 0.01 {
                    *element = 0.0;
                }
                else if *element > 0.99 {
                    *element = 1.0;
                } 
            }
        }

        Self { values }
    }
    pub fn rotate(self, degrees: f32, revultion_vector: Vector3<f32>) -> Self {
        self * Self::identity_matrix()
    }
}

impl<T> Matrix for Matrix3<T>
where
    T: Num + Copy
{
    const SIZE: [usize; 2] = [3, 3];
}

impl<T> Add for Matrix3<T>
where
    T: Num + Copy
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let values: [[T; 3]; 3] = [
            [self[[0, 0]] + other[[0, 0]], self[[1, 0]] + other[[1, 0]], self[[2, 0]] + other[[2, 0]]],
            [self[[0, 1]] + other[[0, 1]], self[[1, 1]] + other[[1, 1]], self[[2, 1]] + other[[2, 1]]],
            [self[[0, 2]] + other[[0, 2]], self[[1, 2]] + other[[1, 2]], self[[2, 2]] + other[[2, 2]]],
        ];
        Self { values }
    }
}

impl<T> Sub for Matrix3<T>
where
    T: Num + Copy
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let values: [[T; 3]; 3] = [
            [self[[0, 0]] - other[[0, 0]], self[[1, 0]] - other[[1, 0]], self[[2, 0]] - other[[2, 0]]],
            [self[[0, 1]] - other[[0, 1]], self[[1, 1]] - other[[1, 1]], self[[2, 1]] - other[[2, 1]]],
            [self[[0, 2]] - other[[0, 2]], self[[1, 2]] - other[[1, 2]], self[[2, 2]] - other[[2, 2]]],
        ];
        Self { values }
    }
}

impl<T> Mul<Matrix3<T>> for Matrix3<T>
where
    T: Num + Copy
{
    type Output = Matrix3<T>;

    fn mul(self, other: Matrix3<T>) -> Self::Output {
        let rows1 = self.as_vectors_rows();
        let collumns2 = self.as_vectors_collumns();

        let values: [[T; 3]; 3] = [
            [(rows1[0] * collumns2[0]).sum(), (rows1[0] * collumns2[1]).sum(), (rows1[0] * collumns2[2]).sum()],
            [(rows1[1] * collumns2[0]).sum(), (rows1[1] * collumns2[1]).sum(), (rows1[1] * collumns2[2]).sum()],
            [(rows1[2] * collumns2[0]).sum(), (rows1[2] * collumns2[1]).sum(), (rows1[2] * collumns2[2]).sum()]
        ];
        Matrix3 { values }
    }
}
impl<T> Mul<Vector3<T>> for Matrix3<T>
where
    T: Num + Copy
{
    type Output = Vector3<T>;

    fn mul(self, other: Vector3<T>) -> Self::Output {
        let rows = self.as_vectors_rows();
        let result: Vector3<T> = Vector3::new(
            (rows[0] * other).sum(),
            (rows[1] * other).sum(),
            (rows[2] * other).sum(),
        );
        result
    }
}

impl<T> Index<[usize; 2]> for Matrix3<T>
where
    T: Num + Copy
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.values[index[1]][index[0]]
    }
}
impl<T> IndexMut<[usize; 2]> for Matrix3<T>
where
    T: Num + Copy
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.values[index[1]][index[0]]
    }
}