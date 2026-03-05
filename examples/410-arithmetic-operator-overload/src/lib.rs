//! Arithmetic Operator Overloading
//!
//! Using Add, Sub, Mul, Div, Neg traits for custom types.

use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// A 2D vector with operator overloading.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Creates a new vector.
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    /// Creates a zero vector.
    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    /// Creates a unit vector in the X direction.
    pub fn unit_x() -> Self {
        Vec2 { x: 1.0, y: 0.0 }
    }

    /// Creates a unit vector in the Y direction.
    pub fn unit_y() -> Self {
        Vec2 { x: 0.0, y: 1.0 }
    }

    /// Calculates the magnitude (length) of the vector.
    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculates the dot product with another vector.
    pub fn dot(self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Returns a normalized (unit) vector.
    pub fn normalized(self) -> Vec2 {
        let mag = self.magnitude();
        if mag > 0.0 {
            self / mag
        } else {
            Vec2::zero()
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

// Scalar * Vec2
impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        Vec2::new(self * v.x, self * v.y)
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2({:.2}, {:.2})", self.x, self.y)
    }
}

/// A complex number with arithmetic operators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    pub fn from_polar(r: f64, theta: f64) -> Self {
        Complex {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn magnitude(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn conjugate(self) -> Complex {
        Complex::new(self.re, -self.im)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex::new(-self.re, -self.im)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{:.2} + {:.2}i", self.re, self.im)
        } else {
            write!(f, "{:.2} - {:.2}i", self.re, -self.im)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_vec2_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a + b, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_sub() {
        let a = Vec2::new(5.0, 7.0);
        let b = Vec2::new(2.0, 3.0);
        assert_eq!(a - b, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_vec2_mul_scalar() {
        let v = Vec2::new(2.0, 3.0);
        assert_eq!(v * 2.0, Vec2::new(4.0, 6.0));
        assert_eq!(2.0 * v, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_div() {
        let v = Vec2::new(4.0, 6.0);
        assert_eq!(v / 2.0, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_vec2_neg() {
        let v = Vec2::new(1.0, -2.0);
        assert_eq!(-v, Vec2::new(-1.0, 2.0));
    }

    #[test]
    fn test_vec2_add_assign() {
        let mut v = Vec2::new(1.0, 2.0);
        v += Vec2::new(3.0, 4.0);
        assert_eq!(v, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_magnitude() {
        let v = Vec2::new(3.0, 4.0);
        assert!(approx_eq(v.magnitude(), 5.0));
    }

    #[test]
    fn test_vec2_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert!(approx_eq(a.dot(b), 11.0)); // 1*3 + 2*4
    }

    #[test]
    fn test_vec2_normalized() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalized();
        assert!(approx_eq(n.magnitude(), 1.0));
    }

    #[test]
    fn test_complex_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        assert_eq!(a + b, Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_complex_mul() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        // (1 + 2i)(3 + 4i) = 3 + 4i + 6i + 8i² = 3 + 10i - 8 = -5 + 10i
        assert_eq!(a * b, Complex::new(-5.0, 10.0));
    }

    #[test]
    fn test_complex_conjugate() {
        let c = Complex::new(3.0, 4.0);
        let conj = c.conjugate();
        assert_eq!(conj, Complex::new(3.0, -4.0));
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert!(approx_eq(c.magnitude(), 5.0));
    }
}
