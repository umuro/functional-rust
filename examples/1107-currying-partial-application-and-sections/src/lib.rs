//! Currying, partial application, and operator sections in Rust.
//!
//! OCaml functions are curried by default — `let add x y = x + y` accepts
//! one argument and returns a function waiting for the second. Rust functions
//! take all arguments at once, but the same patterns emerge naturally with
//! `move` closures and higher-order functions.

// ---------------------------------------------------------------------------
// 1. Partial application via closures — idiomatic Rust
// ---------------------------------------------------------------------------

/// Returns an adder function with `x` fixed as the first operand.
///
/// OCaml: `let add5 = add 5` — automatic partial application.
/// Rust: explicit `move` closure that captures `x`.
pub fn partial_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Tupled variant — takes both arguments as a pair.
///
/// OCaml: `let add_tup (x, y) = x + y` — destructuring in the parameter.
/// Rust: identical syntax via irrefutable pattern in the argument position.
pub fn add_tup((x, y): (i32, i32)) -> i32 {
    x + y
}

// ---------------------------------------------------------------------------
// 2. curry / uncurry — converting between calling conventions
// ---------------------------------------------------------------------------

/// Converts a tupled function `(A, B) → C` into a curried function `A → (B → C)`.
///
/// OCaml: `let curry f x y = f (x, y)`
///
/// The inner closure is heap-allocated (`Box<dyn Fn>`) because returning
/// `impl Fn` from inside a `move` closure is not stable without boxing.
/// Each call to the outer closure clones `f` so the inner box can own it.
pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Clone + 'static,
    B: 'static,
    C: 'static,
    F: Fn((A, B)) -> C + Clone + 'static,
{
    move |a: A| {
        let f = f.clone();
        Box::new(move |b: B| f((a.clone(), b)))
    }
}

/// Converts a curried function `A → (B → C)` into a tupled function `(A, B) → C`.
///
/// OCaml: `let uncurry f (x, y) = f x y`
///
/// `G` captures the concrete (but opaque) type returned by `f(a)`.
/// The resulting closure is `Fn` because neither `f` nor the temporary `G`
/// value are consumed between calls.
pub fn uncurry<A, B, C, F, G>(f: F) -> impl Fn((A, B)) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |(a, b)| f(a)(b)
}

// ---------------------------------------------------------------------------
// 3. Operator sections — Rust equivalent of OCaml's `( * ) 2`, `( + ) 1`
// ---------------------------------------------------------------------------

/// Doubles its argument. OCaml: `let double = ( * ) 2`.
pub fn double(x: i32) -> i32 {
    x * 2
}

/// Adds 1 to its argument. OCaml: `let increment = ( + ) 1`.
pub fn increment(x: i32) -> i32 {
    x + 1
}

/// Halves its argument using integer division.
/// OCaml: `let halve = Fun.flip ( / ) 2` — `flip` swaps argument order so 2
/// becomes the *divisor*, not the dividend.
pub fn halve(x: i32) -> i32 {
    x / 2
}

// ---------------------------------------------------------------------------
// 4. Labeled-argument partial application
// ---------------------------------------------------------------------------

/// Builds a linear transform `x * scale + shift` with fixed `scale` and `shift`.
///
/// OCaml uses labeled arguments for any-order partial application:
/// ```text
/// let scale_and_shift ~scale ~shift x = x * scale + shift
/// let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)
/// ```
/// Rust achieves the same result with a closure capturing both parameters.
pub fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}

// ---------------------------------------------------------------------------
// 5. Pipeline helper
// ---------------------------------------------------------------------------

/// Applies a sequence of transformations to an initial value, left to right.
///
/// OCaml: `List.fold_left (fun acc f -> f acc) init pipeline`
pub fn apply_pipeline(init: i32, pipeline: &[fn(i32) -> i32]) -> i32 {
    pipeline.iter().fold(init, |acc, f| f(acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_add_specialises_adder() {
        let add5 = partial_add(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-3), 2);
    }

    #[test]
    fn test_partial_add_zero_and_negative_base() {
        assert_eq!(partial_add(0)(42), 42);
        assert_eq!(partial_add(-7)(7), 0);
        assert_eq!(partial_add(100)(100), 200);
    }

    #[test]
    fn test_add_tup_various_pairs() {
        assert_eq!(add_tup((3, 4)), 7);
        assert_eq!(add_tup((0, 0)), 0);
        assert_eq!(add_tup((-1, 1)), 0);
        assert_eq!(add_tup((-5, -5)), -10);
    }

    #[test]
    fn test_curry_wraps_tupled_function() {
        let curried = curry(|(x, y): (i32, i32)| x + y);
        assert_eq!(curried(3)(4), 7);
        assert_eq!(curried(0)(0), 0);
        assert_eq!(curried(-1)(1), 0);
    }

    #[test]
    fn test_curry_creates_reusable_partial() {
        let curried = curry(|(x, y): (i32, i32)| x * y);
        let triple = curried(3);
        assert_eq!(triple(4), 12);
        assert_eq!(triple(10), 30);
        assert_eq!(triple(0), 0);
    }

    #[test]
    fn test_uncurry_wraps_curried_function() {
        let tupled = uncurry(|x: i32| move |y: i32| x + y);
        assert_eq!(tupled((3, 4)), 7);
        assert_eq!(tupled((0, 5)), 5);
        assert_eq!(tupled((-1, 1)), 0);
    }

    #[test]
    fn test_operator_sections_double() {
        assert_eq!(double(7), 14);
        assert_eq!(double(0), 0);
        assert_eq!(double(-3), -6);
    }

    #[test]
    fn test_operator_sections_increment_and_halve() {
        assert_eq!(increment(9), 10);
        assert_eq!(increment(-1), 0);
        assert_eq!(halve(20), 10);
        assert_eq!(halve(7), 3); // integer division truncates
    }

    #[test]
    fn test_pipeline_fold() {
        // 6 →*2→ 12 →+1→ 13 →/2→ 6 (integer division)
        assert_eq!(apply_pipeline(6, &[double, increment, halve]), 6);
        // 1 →*2→ 2 →*2→ 4 →*2→ 8
        assert_eq!(apply_pipeline(1, &[double, double, double]), 8);
        // empty pipeline is identity
        assert_eq!(apply_pipeline(42, &[]), 42);
    }

    #[test]
    fn test_scale_and_shift_partial_application() {
        // celsius_of_fahrenheit via partial application: f * 5 - 160
        // (note: full Celsius conversion requires a subsequent / 9 step;
        //  the focus here is the partial-application pattern, not accuracy)
        let celsius_of_fahrenheit = scale_and_shift(5, -160);
        assert_eq!(celsius_of_fahrenheit(32), 0); // 32*5 - 160 = 0
        assert_eq!(celsius_of_fahrenheit(212), 900); // 212*5 - 160 = 900
    }

    #[test]
    fn test_scale_and_shift_general() {
        let double_plus_one = scale_and_shift(2, 1);
        assert_eq!(double_plus_one(0), 1); // 0*2 + 1
        assert_eq!(double_plus_one(5), 11); // 5*2 + 1
        let negate = scale_and_shift(-1, 0);
        assert_eq!(negate(7), -7);
        assert_eq!(negate(-3), 3);
    }
}
