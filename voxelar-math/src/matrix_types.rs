use crate::matrix::Matrix;

// Matrix types are defined for all variants of glUniformMatrix...fv
// (https://registry.khronos.org/OpenGL-Refpages/gl4/html/glUniform.xhtml).
//
// These variants match up with the type names in GLSL, e.g. glUniformMatrix2x3fv 
// for mat2x3.

pub type Matrix2xN<T, const ROWS: usize> = Matrix<T, 2, ROWS>;
pub type Matrix2<T> = Matrix2xN<T, 2>;
pub type Matrix2x2<T> = Matrix2xN<T, 2>;
pub type Matrix2x3<T> = Matrix2xN<T, 3>;
pub type Matrix2x4<T> = Matrix2xN<T, 4>;

pub type Matrix3xN<T, const ROWS: usize> = Matrix<T, 3, ROWS>;
pub type Matrix3<T> = Matrix3xN<T, 3>;
pub type Matrix3x2<T> = Matrix3xN<T, 2>;
pub type Matrix3x3<T> = Matrix3xN<T, 3>;
pub type Matrix3x4<T> = Matrix3xN<T, 4>;

pub type Matrix4xN<T, const ROWS: usize> = Matrix<T, 4, ROWS>;
pub type Matrix4<T> = Matrix4xN<T, 4>;
pub type Matrix4x2<T> = Matrix4xN<T, 2>;
pub type Matrix4x3<T> = Matrix4xN<T, 3>;
pub type Matrix4x4<T> = Matrix4xN<T, 4>;
