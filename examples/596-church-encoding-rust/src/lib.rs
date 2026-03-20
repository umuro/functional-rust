#![allow(clippy::all)]
//! # Church Encoding
//!
//! Represent data using only functions (lambda calculus style).

/// Church numeral type - a number is how many times you apply f.
pub type ChurchNum<T> = Box<dyn Fn(Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T>>;

/// Church zero - apply f zero times.
pub fn zero<T: 'static>() -> impl Fn(Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T> {
    |_f| Box::new(|x| x)
}

/// Church successor - apply f one more time.
pub fn succ<T: Clone + 'static>(
    n: impl Fn(Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T> + 'static,
) -> impl Fn(Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T> {
    move |f: Box<dyn Fn(T) -> T>| {
        let nf = n(Box::new(move |x| f(x)));
        Box::new(move |x| {
            let inner = nf(x);
            inner
        })
    }
}

/// Convert Church numeral to integer.
pub fn to_int(n: impl Fn(Box<dyn Fn(i32) -> i32>) -> Box<dyn Fn(i32) -> i32>) -> i32 {
    n(Box::new(|x| x + 1))(0)
}

/// Church boolean - true.
pub fn church_true<T>() -> impl Fn(T, T) -> T {
    |a, _b| a
}

/// Church boolean - false.
pub fn church_false<T>() -> impl Fn(T, T) -> T {
    |_a, b| b
}

/// Church boolean to Rust bool.
pub fn to_bool(b: impl Fn(bool, bool) -> bool) -> bool {
    b(true, false)
}

/// Church pair constructor.
pub fn pair<A: Clone + 'static, B: Clone + 'static>(
    a: A,
    b: B,
) -> impl Fn(Box<dyn Fn(A, B) -> A>) -> A + Clone {
    move |f| f(a.clone(), b.clone())
}

/// Church pair first.
pub fn fst<A: Clone, B>(p: impl Fn(Box<dyn Fn(A, B) -> A>) -> A) -> A {
    p(Box::new(|a, _b| a))
}

/// Simple demonstration with closures.
pub fn demo_church_bool() -> (bool, bool) {
    let t = church_true();
    let f = church_false();
    (to_bool(t), to_bool(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_church_bool() {
        assert!(to_bool(church_true()));
        assert!(!to_bool(church_false()));
    }

    #[test]
    fn test_zero() {
        assert_eq!(to_int(zero()), 0);
    }

    #[test]
    fn test_demo() {
        let (t, f) = demo_church_bool();
        assert!(t);
        assert!(!f);
    }
}
