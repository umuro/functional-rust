//! # Church Numerals — Functions as Numbers
//!
//! Lambda calculus encoding where numbers are higher-order functions.
//! OCaml's polymorphic functions map to Rust's `Fn` trait objects or generics.

// ---------------------------------------------------------------------------
// Approach A: Using Box<dyn Fn> for Church numerals
// ---------------------------------------------------------------------------

/// A Church numeral: takes a function and a value, applies the function N times.
pub type Church = Box<dyn Fn(Box<dyn Fn(i64) -> i64>) -> Box<dyn Fn(i64) -> i64>>;

pub fn zero() -> Church {
    Box::new(|_f| Box::new(|x| x))
}

pub fn one() -> Church {
    Box::new(|f: Box<dyn Fn(i64) -> i64>| {
        Box::new(move |x| f(x))
    })
}

pub fn succ(n: &dyn Fn(Box<dyn Fn(i64) -> i64>) -> Box<dyn Fn(i64) -> i64>) -> Church {
    // Can't easily compose due to ownership... see Approach B
    let result = to_int_inner(n) + 1;
    from_int(result as usize)
}

fn to_int_inner(n: &dyn Fn(Box<dyn Fn(i64) -> i64>) -> Box<dyn Fn(i64) -> i64>) -> i64 {
    let f = n(Box::new(|x| x + 1));
    f(0)
}

pub fn to_int(n: &Church) -> i64 {
    let f = n(Box::new(|x| x + 1));
    f(0)
}

pub fn from_int(n: usize) -> Church {
    Box::new(move |f: Box<dyn Fn(i64) -> i64>| {
        Box::new(move |x| {
            let mut result = x;
            for _ in 0..n {
                result = f(result);
            }
            result
        })
    })
}

// ---------------------------------------------------------------------------
// Approach B: Simple integer-backed (practical encoding)
// ---------------------------------------------------------------------------

/// Practical Church numeral — store the count, apply when needed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChurchNum(pub usize);

impl ChurchNum {
    pub fn zero() -> Self { ChurchNum(0) }
    pub fn one() -> Self { ChurchNum(1) }
    pub fn succ(self) -> Self { ChurchNum(self.0 + 1) }
    pub fn add(self, other: Self) -> Self { ChurchNum(self.0 + other.0) }
    pub fn mul(self, other: Self) -> Self { ChurchNum(self.0 * other.0) }

    pub fn apply<T>(&self, f: impl Fn(T) -> T, x: T) -> T {
        let mut result = x;
        for _ in 0..self.0 {
            result = f(result);
        }
        result
    }

    pub fn to_int(&self) -> usize {
        self.apply(|x: usize| x + 1, 0)
    }
}

// ---------------------------------------------------------------------------
// Approach C: Generic function composition
// ---------------------------------------------------------------------------

pub fn church_apply<T>(n: usize, f: impl Fn(T) -> T, x: T) -> T {
    (0..n).fold(x, |acc, _| f(acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(to_int(&zero()), 0);
    }

    #[test]
    fn test_one() {
        assert_eq!(to_int(&one()), 1);
    }

    #[test]
    fn test_from_int() {
        assert_eq!(to_int(&from_int(5)), 5);
    }

    #[test]
    fn test_church_num_basic() {
        let two = ChurchNum::one().succ();
        let three = ChurchNum::one().add(two);
        assert_eq!(three.to_int(), 3);
    }

    #[test]
    fn test_church_num_mul() {
        let two = ChurchNum(2);
        let three = ChurchNum(3);
        assert_eq!(two.mul(three).to_int(), 6);
    }

    #[test]
    fn test_church_apply() {
        assert_eq!(church_apply(3, |x: i32| x * 2, 1), 8); // 1 -> 2 -> 4 -> 8
    }

    #[test]
    fn test_church_apply_zero() {
        assert_eq!(church_apply(0, |x: i32| x + 1, 42), 42);
    }
}

fn main() {
    println!("{:?}", to_int(&zero()), 0);
    println!("{:?}", to_int(&one()), 1);
    println!("{:?}", to_int(&from_int(5)), 5);
}
