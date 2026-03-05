// 774. Const Generics: fn<const N: usize> Fundamentals
// Stable since Rust 1.51

// ── Functions parameterized by const ──────────────────────────────────────────

/// Sum an array of any compile-time-known length
fn sum<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}

/// Dot product of two fixed-size arrays
fn dot<const N: usize>(a: &[f64; N], b: &[f64; N]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Zero-initialize a fixed-size array (compile-time size)
fn zeros<const N: usize>() -> [i32; N] {
    [0; N]
}

/// Check if two fixed-size arrays are equal element-wise
fn array_eq<T: PartialEq, const N: usize>(a: &[T; N], b: &[T; N]) -> bool {
    a == b
}

/// Reverse a fixed-size array
fn reversed<T: Copy + Default, const N: usize>(arr: &[T; N]) -> [T; N] {
    let mut out = [T::default(); N];
    for i in 0..N {
        out[i] = arr[N - 1 - i];
    }
    out
}

// ── Type that carries size in the type system ─────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<const N: usize> {
    data: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub fn new(data: [f64; N]) -> Self { Self { data } }
    pub fn zeros() -> Self { Self { data: [0.0; N] } }

    pub fn norm_sq(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum()
    }

    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum()
    }
}

impl<const N: usize> std::fmt::Display for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.data.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{v:.2}")?;
        }
        write!(f, "]")
    }
}

fn main() {
    // Free functions with const generic
    let a3 = [1, 2, 3];
    let a4 = [1, 2, 3, 4];
    println!("sum([1,2,3])   = {}", sum(&a3));
    println!("sum([1,2,3,4]) = {}", sum(&a4));

    let fa = [1.0_f64, 2.0, 3.0];
    let fb = [4.0_f64, 5.0, 6.0];
    println!("dot([1,2,3],[4,5,6]) = {}", dot(&fa, &fb)); // 32

    let z: [i32; 5] = zeros();
    println!("zeros:5  = {z:?}");

    let rev = reversed(&[1, 2, 3, 4, 5]);
    println!("reversed = {rev:?}");

    // Vector<N> type
    let v3 = Vector::new([3.0_f64, 4.0, 0.0]);
    println!("\nv3 = {v3}");
    println!("norm = {:.2}", v3.norm()); // 5.0

    let v2 = Vector::new([1.0_f64, 0.0]);
    let w2 = Vector::new([0.0_f64, 1.0]);
    println!("dot(e1, e2) = {}", v2.dot(&w2)); // 0 — orthogonal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_fixed() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn dot_product() {
        let a = [1.0_f64, 2.0, 3.0];
        let b = [4.0_f64, 5.0, 6.0];
        assert!((dot(&a, &b) - 32.0).abs() < 1e-10);
    }

    #[test]
    fn zeros_all_zero() {
        let z: [i32; 7] = zeros();
        assert!(z.iter().all(|&x| x == 0));
    }

    #[test]
    fn vector_norm() {
        let v = Vector::new([3.0_f64, 4.0]);
        assert!((v.norm() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn reversed_array() {
        assert_eq!(reversed(&[1, 2, 3]), [3, 2, 1]);
    }
}
