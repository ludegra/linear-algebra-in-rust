use super::Matrix;
use crate::vector::{Vector, Vector3, Vector4};
use crate::vector::{ToVector3};
use crate::utils::Pad;

use num_traits::Num;
use std::ops::{Add, Mul, Sub, Index, IndexMut};
use std::fmt::{Display, Formatter};

use super::Matrix4;

impl<T> Matrix4<T>
    where T: Num + Copy
{
    pub fn new(values: [[T; 4]; 4]) -> Self {
        Matrix4 { values }
    }
    pub fn as_vectors_rows(&self) -> [Vector4<T>; 4] {
        let result = [
            Vector4::new(self[[0, 0]], self[[1, 0]], self[[2, 0]], self[[3, 0]]),
            Vector4::new(self[[0, 1]], self[[1, 1]], self[[2, 1]], self[[3, 1]]),
            Vector4::new(self[[0, 2]], self[[1, 2]], self[[2, 2]], self[[3, 2]]),
            Vector4::new(self[[0, 3]], self[[1, 3]], self[[2, 3]], self[[3, 3]]),
        ];
        result
    }
    pub fn as_vectors_collumns(&self) -> [Vector4<T>; 4] {
        let result = [
            Vector4::new(self[[0, 0]], self[[0, 1]], self[[0, 2]], self[[0, 3]]),
            Vector4::new(self[[1, 0]], self[[1, 1]], self[[1, 2]], self[[1, 3]]),
            Vector4::new(self[[2, 0]], self[[2, 1]], self[[2, 2]], self[[2, 3]]),
            Vector4::new(self[[3, 0]], self[[3, 1]], self[[3, 2]], self[[3, 3]]),
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
    pub fn translation_matrix<V: ToVector3<i32>>(vector: V) -> Self {
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
    pub fn translation_matrix<V: ToVector3<u32>>(vector: V) -> Self {
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
    pub fn translation_matrix<V: ToVector3<f32>>(vector: V) -> Self {
        let mut matrix = Self::identity_matrix();
        let vector = vector.to_vec_3();
        for i in 0..3 {
            matrix[[3, i]] = vector[i];
        }
        matrix
    }
    /// Returns self multiplied by a transformation matrix from the given vector
    pub fn translate<V>(self, vector: V) -> Self
        where V: Vector<f32> + ToVector3<f32>
    {
        self * Self::translation_matrix(vector)
    }
    /// Returns a 4x4 rotation matrix
    /// 
    /// axis: axis around witch the rotation takes place
    /// degrees: degrees for rotation in radians
    pub fn rotation_matrix(degrees: f32, u: Vector3<f32>) -> Self {
        let (cos, sin) = (degrees.cos(), degrees.sin());
        
        let mut values = [
            [cos + u.x.powi(2) * (1.0 - cos), u.x * u.y * (1.0 - cos) - u.z * sin, u.x * u.z * (1.0 - cos) + u.y * sin, 0.0],
            [u.y * u.x * (1.0 - cos) + u.z * sin, cos + u.y.powi(2) * (1.0 - cos), u.y * u.z * (1.0 - cos) - u.x * sin, 0.0],
            [u.z * u.x * (1.0 - cos) - u.y * sin, u.z * u.y * (1.0 - cos) + u.x * sin, cos + u.z.powi(2) * (1.0 - cos), 0.0],
            [0.0, 0.0, 0.0, 1.0]
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
    /// Returns self multiplied by a rotation vector with a rotation of the given angle around the given vector.
    /// 
    /// axis: axis around witch the rotation takes place
    /// degrees: degrees for rotation in radians
    pub fn rotate(self, angle: f32, revultion_vector: Vector3<f32>) -> Self {
        self * Self::rotation_matrix(angle, revultion_vector)
    }
    pub fn scaling_matrix<V: ToVector3<f32>>(vector: V) -> Self {
        let vector = vector.to_vec_3();
        let mut output = Self::identity_matrix();

        for i in 0..3 {
            output[[i, i]] = vector[i];
        }
        output
    }
    pub fn scale<V: ToVector3<f32>>(self, vector: V) -> Self {
        self * Self::scaling_matrix(vector)
    }
}

impl<T> Matrix for Matrix4<T>
    where T: Num + Copy
{
    const SIZE: [usize; 2] = [4, 4];
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
impl<T> Mul<Vector4<T>> for Matrix4<T>
where
    T: Num + Copy
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

impl<T> Display for Matrix4<T>
where
    T: Num + Copy + Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let collumns = self.as_vectors_collumns();
        let mut des_lenghts = [0; 4];

        for i in 0..collumns.len() {
            let mut longest = 0usize;

            for element in collumns[i] {
                let len = element.to_string().len();
                if len > longest {
                    longest = len;
                }
            }
            des_lenghts[i] = longest;
        }
        let rows = self.as_vectors_rows();
        let mut rows_strings = vec![String::new(); 4];

        for i in 0..rows.len() {
            for j in 0..rows[i].len() {
                rows_strings[i].push_str(&format!("{} ", rows[i][j].to_string().pad_c(des_lenghts[j])))
            }
            rows_strings[i] = String::from(rows_strings[i].trim());
        }

        write!(
            f,
            "\n⎡{}⎤\n⎢{}⎥\n⎢{}⎥\n⎣{}⎦\n",
            rows_strings[0],
            rows_strings[1],
            rows_strings[2],
            rows_strings[3],
        )
    }
}