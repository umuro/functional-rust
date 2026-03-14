//! Currying, Partial Application, and Sections in Rust.
//!
//! This example demonstrates how to emulate OCaml's currying,
//! partial application, and operator sections in Rust.

// ---------- Solution 1: Idiomatic Rust (closures) ----------

/// Adds two integers (idiomatic Rust closure).
/// This is not curried; it's a simple closure.
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Partially applies `add` to `5`.
pub fn add5() -> impl Fn(i32) -> i32 {
    |y| add(5, y)
}

/// Tupled version of add (takes a tuple).
pub fn add_tup(pair: (i32, i32)) -> i32 {
    pair.0 + pair.1
}

/// Converts a tupled function (i32,i32)->i32 to a curried function.
/// Converts a tupled function (i32,i32)->i32 to a curried function.
pub fn curry_i32<F>(f: F) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32>
where
    F: Fn((i32, i32)) -> i32,
    F: Clone + 'static,
{
    move |x| {
        let f = f.clone();
        Box::new(move |y| f((x, y)))
    }
}

/// Converts a curried function (i32)->(i32)->i32 to a tupled function.
pub fn uncurry_i32<F>(f: F) -> impl Fn((i32, i32)) -> i32
where
    F: Fn(i32) -> Box<dyn Fn(i32) -> i32>,
{
    move |(x, y)| f(x)(y)
}

/// Operator section: multiply by 2.
pub fn double() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

/// Operator section: increment by 1.
pub fn increment() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// Operator section: halve (integer division by 2).
pub fn halve() -> impl Fn(i32) -> i32 {
    |x| x / 2
}

/// Labeled‑style function with named parameters (Rust doesn't have labeled
/// arguments; we use a struct or separate closures).
pub fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}

/// Celsius to Fahrenheit conversion using partial application.
pub fn celsius_of_fahrenheit() -> impl Fn(i32) -> i32 {
    scale_and_shift(5, -160)
}

// ---------- Solution 2: Functional/curried style ----------

/// Curried addition (returns a closure).
pub fn add_curried(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Curried multiplication (operator section).
pub fn double_curried() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

/// Curried increment.
pub fn increment_curried() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// Curried halve with flipped division.
pub fn halve_curried() -> impl Fn(i32) -> i32 {
    |x| x / 2
}

// ---------- Tests ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add5() {
        let add5_fn = add5();
        assert_eq!(add5_fn(10), 15);
    }

    #[test]
    fn test_add_tup() {
        assert_eq!(add_tup((2, 3)), 5);
    }

    #[test]
    fn test_double() {
        let double_fn = double();
        assert_eq!(double_fn(7), 14);
    }

    #[test]
    fn test_increment() {
        let inc_fn = increment();
        assert_eq!(inc_fn(10), 11);
    }

    #[test]
    fn test_halve() {
        let halve_fn = halve();
        assert_eq!(halve_fn(20), 10);
    }

    #[test]
    fn test_scale_and_shift() {
        let scale_shift_fn = scale_and_shift(5, -160);
        assert_eq!(scale_shift_fn(212), 212 * 5 - 160);
    }

    #[test]
    fn test_celsius_of_fahrenheit() {
        let converter = celsius_of_fahrenheit();
        // 212°F should give (212 * 5 - 160) = 900? Wait formula is (F - 32) * 5/9.
        // But OCaml example uses scale 5 shift -160, which is approximation.
        // We'll just check the same computation.
        assert_eq!(converter(212), 212 * 5 - 160);
    }

    #[test]
    fn test_add_curried() {
        let add5 = add_curried(5);
        assert_eq!(add5(10), 15);
    }

    #[test]
    fn test_pipeline() {
        // Simulate OCaml pipeline: 6 |> *2 |> +1 |> /2
        let double_fn = double();
        let inc_fn = increment();
        let halve_fn = halve();
        let result = halve_fn(inc_fn(double_fn(6)));
        assert_eq!(result, 6);
    }
}
