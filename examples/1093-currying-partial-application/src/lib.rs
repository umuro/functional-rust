// # Currying and Partial Application
//
// OCaml: `let add x y = x + y` — all functions are automatically curried.
// Rust has no auto-currying, but closures make partial application natural.

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — closures for partial application
// ---------------------------------------------------------------------------

/// A plain two-argument function — Rust's default style.
pub fn add(x: i64, y: i64) -> i64 {
    x + y
}

/// Returns a closure that adds `x` to its argument.
/// This is how Rust developers do partial application: return a closure.
pub fn add_partial(x: i64) -> impl Fn(i64) -> i64 {
    move |y| x + y
}

// ---------------------------------------------------------------------------
// Solution 2: Curried style — mirrors OCaml's `let add x y = x + y`
// ---------------------------------------------------------------------------

/// Fully curried: each argument returns a closure expecting the next.
/// Closest to OCaml's automatic currying, but explicit in Rust.
pub fn add_curried(x: i64) -> impl Fn(i64) -> i64 {
    move |y| x + y
}

/// A generic curried add for any type supporting `Add`.
/// Shows how Rust generics replace OCaml's polymorphism here.
pub fn add_curried_generic<T>(x: T) -> impl Fn(T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    move |y| x + y
}

// ---------------------------------------------------------------------------
// Solution 3: Higher-order — curry any two-argument function
// ---------------------------------------------------------------------------

/// Transforms a two-argument function into a curried chain.
/// `curry(f)` returns `|x| |y| f(x, y)`.
pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Copy + 'static,
    B: 'static,
    C: 'static,
    F: Fn(A, B) -> C + Copy + 'static,
{
    move |a: A| Box::new(move |b: B| f(a, b))
}

/// The inverse: uncurry a curried function back to a two-argument function.
pub fn uncurry<A, B, C, F, G>(f: F) -> impl Fn(A, B) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |a, b| f(a)(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_direct() {
        assert_eq!(add(3, 4), 7);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_partial_application() {
        let add5 = add_partial(5);
        assert_eq!(add5(3), 8);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-5), 0);
    }

    #[test]
    fn test_curried() {
        let add10 = add_curried(10);
        assert_eq!(add10(1), 11);
        assert_eq!(add10(-10), 0);

        // Call in one shot: add_curried(2)(3)
        assert_eq!(add_curried(2)(3), 5);
    }

    #[test]
    fn test_curried_generic() {
        let add_f64 = add_curried_generic(1.5_f64);
        assert!((add_f64(2.5) - 4.0).abs() < f64::EPSILON);

        let add_i32 = add_curried_generic(100_i32);
        assert_eq!(add_i32(23), 123);
    }

    #[test]
    fn test_curry_combinator() {
        let curried_add = curry(add);
        let add7 = curried_add(7);
        assert_eq!(add7(3), 10);
        assert_eq!(add7(0), 7);
    }

    #[test]
    fn test_uncurry_combinator() {
        let uncurried = uncurry(add_curried);
        assert_eq!(uncurried(3, 4), 7);
        assert_eq!(uncurried(0, 0), 0);
    }

    #[test]
    fn test_curry_with_multiply() {
        let mul = |a: i64, b: i64| a * b;
        let curried_mul = curry(mul);
        let double = curried_mul(2);
        assert_eq!(double(5), 10);
        assert_eq!(double(0), 0);
    }
}
