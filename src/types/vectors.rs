/* Cymbalum, Molecular Simulation in Rust - Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
 */

//! 3-dimmensional vector type

use std::ops::{Add, Sub, Neg, Mul, Div, BitXor, Index, IndexMut};
use std::cmp::PartialEq;
use super::matrix::Matrix3;

/// 3 dimensional vector type, implementing all usual operations
#[derive(Copy, Clone, Debug)]
pub struct Vector3D {
    /// First component of the vector
    pub x: f64,
    /// Second component of the vector
    pub y: f64,
    /// Third component of the vector
    pub z: f64,
}

impl Vector3D {
    /// Create a new Vector3D with components `x`, `y`, `z`
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D{x: x, y: y, z: z}
    }
    /// Return the squared euclidean norm of a Vector3D
    #[inline] pub fn norm2(&self) -> f64 {
        (*self) * (*self)
    }
    /// Return the euclidean norm of a Vector3D
    #[inline] pub fn norm(&self) -> f64 {
        f64::sqrt(self.norm2())
    }
    /// Normalize a Vector3D
    #[inline] pub fn normalized(&self) -> Vector3D {
        *self / self.norm()
    }
    /// Tensorial product between vectors
    pub fn tensorial(&self, other: &Vector3D) -> Matrix3 {
        Matrix3::new(self.x * other.x, self.x * other.y, self.x * other.z,
                     self.y * other.x, self.y * other.y, self.y * other.z,
                     self.z * other.x, self.z * other.y, self.z * other.z)
    }
}

/// Add two vectors
impl Add for Vector3D {
    type Output = Vector3D;
    #[inline] fn add(self, other: Vector3D) -> Vector3D {
        Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Substract two vectors
impl Sub for Vector3D {
    type Output = Vector3D;
    #[inline] fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Unary - operator
impl Neg for Vector3D {
    type Output = Vector3D;
    #[inline] fn neg(self) -> Vector3D {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

/// Multiply by a scalar on the right hand side
impl Mul<f64> for Vector3D {
    type Output = Vector3D;
    #[inline] fn mul(self, other: f64) -> Vector3D {
        Vector3D::new(self.x * other, self.y * other, self.z * other)
    }
}

/// Multiply by a scalar on the left hand side
impl Mul<Vector3D> for f64 {
    type Output = Vector3D;
    #[inline] fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D::new(self * other.x, self * other.y, self * other.z)
    }
}

/// Scalar product between vectors
impl Mul<Vector3D> for Vector3D {
    type Output = f64;
    #[inline] fn mul(self, other: Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// Vectorial product will use the a^b notation.
impl BitXor<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn bitxor(self, other: Vector3D) -> Vector3D {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vector3D::new(x, y, z)
    }
}

/// Dividing a vector by a scalar
impl Div<f64> for Vector3D {
    type Output = Vector3D;
    #[inline] fn div(self, other: f64) -> Vector3D {
        Vector3D::new(self.x / other, self.y / other, self.z / other)
    }
}

/// Comparing two vectors
impl PartialEq for Vector3D {
    #[inline] fn eq(&self, other: &Vector3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

/// This is provided for convenience only, and is slower than direct field access
impl Index<usize> for Vector3D {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bonds indexing!")
        }
    }
}

/// This is provided for convenience only, and is slower than direct field access
impl IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of bonds indexing!")
        }
    }
}

/******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vector3D::new(2.0, 3.5, 4.8);
        let b = Vector3D::new(6.1, -8.5, 7.3);

        let c = a + b;
        assert_eq!(c, Vector3D::new(8.1, -5.0, 12.1));
    }

    #[test]
    fn sub() {
        let a = Vector3D::new(2.0, 3.5, 4.8);
        let b = Vector3D::new(6.1, -8.5, 7.3);

        let c = a - b;
        assert_eq!(c, Vector3D::new(-4.1, 12.0, -2.5));

        let d = -c;
        assert_eq!(d, Vector3D::new(4.1, -12.0, 2.5));
    }

    #[test]
    fn mul() {
        let a = Vector3D::new(2.0, 3.5, 4.8);
        let b = 2.0;

        let c = b * a;
        assert_eq!(c, Vector3D::new(4.0, 7.0, 9.6));

        let b = 1.5;
        let c = a * b;
        assert_eq!(c, Vector3D::new(3.0, 5.25, 7.199999999999999));
    }

    #[test]
    fn dot_product() {
        let a = Vector3D::new(2.1, 3.5, 4.8);
        let b = Vector3D::new(6.1, -8.5, 7.3);

        let c = a * b;
        assert_eq!(c, 18.1);
    }

    #[test]
    fn cross_product() {
        let a = Vector3D::new(2.1, 3.5, 4.8);
        let b = Vector3D::new(6.1, -8.5, 7.3);

        let c = a ^ b;
        assert_eq!(c*a, 0.0);

        let a = Vector3D::new(1.0, 0.0, 0.0);
        let b = Vector3D::new(0.0, 1.0, 0.0);

        let c = a ^ b;
        assert_eq!(c, Vector3D::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn index() {
        let mut a = Vector3D::new(2.1, 3.5, 4.8);

        assert_eq!(a[0], a.x);
        assert_eq!(a[1], a.y);
        assert_eq!(a[2], a.z);

        a[0] = 1.0;
        a[1] = 1.0;
        a[2] = 1.0;

        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 1.0);
        assert_eq!(a.z, 1.0);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let mut a = Vector3D::new(2.1, 3.5, 4.8);
        a[3] += 4.0;
    }
}
