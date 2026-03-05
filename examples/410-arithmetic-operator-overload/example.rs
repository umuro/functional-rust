// Operator overloading with Add/Sub/Mul in Rust
use std::ops::{Add, Sub, Mul, Neg, AddAssign};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 { x: f64, y: f64 }

impl Vec2 {
    fn new(x: f64, y: f64) -> Self { Vec2 { x, y } }
    fn magnitude(self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    fn dot(self, other: Vec2) -> f64 { self.x * other.x + self.y * other.y }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 { Vec2::new(self.x + o.x, self.y + o.y) }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 { Vec2::new(self.x - o.x, self.y - o.y) }
}
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 { Vec2::new(self.x * s, self.y * s) }
}
impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 { Vec2::new(self * v.x, self * v.y) }
}
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Vec2) { self.x += o.x; self.y += o.y; }
}
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2({:.2}, {:.2})", self.x, self.y)
    }
}

fn main() {
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(1.0, 2.0);
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("2 * a = {}", 2.0 * a);
    println!("|a| = {:.2}", a.magnitude());
    println!("-a = {}", -a);
    println!("a . b = {:.2}", a.dot(b));
    let mut c = a;
    c += b;
    println!("a += b: {}", c);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() { assert_eq!(Vec2::new(1.0,2.0) + Vec2::new(3.0,4.0), Vec2::new(4.0,6.0)); }
    #[test]
    fn test_neg() { assert_eq!(-Vec2::new(1.0,2.0), Vec2::new(-1.0,-2.0)); }
    #[test]
    fn test_magnitude() { assert!((Vec2::new(3.0,4.0).magnitude() - 5.0).abs() < 1e-9); }
}
