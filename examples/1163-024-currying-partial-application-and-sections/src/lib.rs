#![allow(clippy::all)]
//! # Currying, Partial Application, and Operator Sections
//!
//! OCaml functions are curried by default: `add : int -> int -> int` can be
//! partially applied as `add 5 : int -> int`. Rust functions are NOT curried —
//! partial application is achieved explicitly by returning closures.
//!
//! This module shows:
//! 1. Partial application via closures (idiomatic Rust)
//! 2. `curry`/`uncurry` converters between tupled and sequential styles
//! 3. `flip` for swapping argument order (like `Fun.flip`)
//! 4. Operator sections as closures / function definitions
//! 5. Function pipelines via iterator fold

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — explicit partial application with closures
// ---------------------------------------------------------------------------

/// Binary addition — takes both arguments at once (Rust default style).
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Partial application: fix the first argument, return a closure for the second.
///
/// OCaml equivalent: `let add5 = add 5`  (automatic currying)
/// Rust requires an explicit closure that captures `x`.
pub fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Tupled version — takes a pair explicitly.
///
/// Not idiomatic in either OCaml or Rust; shown as the counterpart to `curry`.
pub fn add_tup((x, y): (i32, i32)) -> i32 {
    x + y
}

// ---------------------------------------------------------------------------
// Solution 2: curry / uncurry converters (generic, using Box<dyn Fn>)
//
// Rust does not yet stabilise `impl Fn(A) -> impl Fn(B) -> C` as a return
// type, so we return `Box<dyn Fn>` for the inner function.  This is the
// idiomatic way to express "higher-order function that returns a function"
// when the concrete closure type cannot be named.
// ---------------------------------------------------------------------------

/// Convert a tupled function into one that accepts arguments one at a time.
///
/// OCaml: `let curry f x y = f (x, y)`
///
/// `A: Clone + 'static` lets the captured `x` be used multiple times inside
/// the heap-allocated inner closure.
pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    F: Fn((A, B)) -> C + Clone + 'static,
    A: Clone + 'static,
    B: 'static,
    C: 'static,
{
    move |x: A| {
        let f = f.clone();
        let x = x.clone();
        // Box the inner closure so callers get a consistent dyn Fn type
        Box::new(move |y: B| f((x.clone(), y)))
    }
}

/// Convert a closure-returning function into one that takes a tuple.
///
/// OCaml: `let uncurry f (x, y) = f x y`
pub fn uncurry<A, B, C, G, F>(f: F) -> impl Fn((A, B)) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    // Destructure the tuple argument — idiomatic Rust pattern matching
    move |(x, y)| f(x)(y)
}

// ---------------------------------------------------------------------------
// Solution 3: flip — swap argument order (like OCaml's Fun.flip)
// ---------------------------------------------------------------------------

/// Swap the first two arguments of a binary function.
///
/// OCaml: `Fun.flip : ('a -> 'b -> 'c) -> 'b -> 'a -> 'c`
///
/// Usage: `flip(|a, b| a / b)(2)` gives a closure `|x| x / 2`.
pub fn flip<A, B, C, F>(f: F) -> impl Fn(B) -> Box<dyn Fn(A) -> C>
where
    F: Fn(A, B) -> C + Clone + 'static,
    A: 'static,
    B: Clone + 'static,
    C: 'static,
{
    move |b: B| {
        let f = f.clone();
        let b = b.clone();
        Box::new(move |a: A| f(a, b.clone()))
    }
}

// ---------------------------------------------------------------------------
// Operator sections — closures partially applying operators
// ---------------------------------------------------------------------------

/// Equivalent to OCaml's `( * ) 2` — "multiply by 2" section.
pub fn double(x: i32) -> i32 {
    x * 2
}

/// Equivalent to OCaml's `( + ) 1` — "add 1" section.
pub fn increment(x: i32) -> i32 {
    x + 1
}

/// Equivalent to OCaml's `Fun.flip ( / ) 2` — "divide by 2" section.
/// Integer division, matching OCaml's `/` on integers.
pub fn halve(x: i32) -> i32 {
    x / 2
}

// ---------------------------------------------------------------------------
// scale_and_shift — labeled parameters become explicit Rust parameters
// ---------------------------------------------------------------------------

/// General linear transform: `x * scale + shift`.
///
/// OCaml uses labeled arguments (`~scale`, `~shift`) for named partial
/// application in any order. Rust uses positional parameters; partial
/// application wraps the call in a closure.
///
/// OCaml: `let scale_and_shift ~scale ~shift x = x * scale + shift`
pub fn scale_and_shift(scale: i32, shift: i32, x: i32) -> i32 {
    x * scale + shift
}

/// Partial application of `scale_and_shift` for Fahrenheit → °C numerator.
///
/// OCaml: `let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)`
///
/// Formula: `F*5 - 160`. Dividing by 9 gives exact integer °C.
/// (32°F → 0, 212°F → 900, 900/9 = 100°C.)
pub fn celsius_of_fahrenheit(fahrenheit: i32) -> i32 {
    scale_and_shift(5, -160, fahrenheit)
}

// ---------------------------------------------------------------------------
// Function pipeline via fold (like OCaml's List.fold_left)
// ---------------------------------------------------------------------------

/// Apply a sequence of functions left-to-right, starting from `start`.
///
/// OCaml: `List.fold_left (fun acc f -> f acc) start pipeline`
///
/// Uses `fn(i32) -> i32` pointers so named functions (`double`, etc.) can
/// be stored in a plain slice without boxing.
pub fn apply_pipeline(fns: &[fn(i32) -> i32], start: i32) -> i32 {
    fns.iter().fold(start, |acc, f| f(acc))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_application_via_closure() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        // Closure is reusable — callable multiple times
        assert_eq!(add5(-3), 2);
        // make_adder(0) is the identity function
        assert_eq!(make_adder(0)(42), 42);
    }

    #[test]
    fn test_add_and_add_tup_agree() {
        assert_eq!(add(3, 4), add_tup((3, 4)));
        assert_eq!(add(0, 7), add_tup((0, 7)));
        assert_eq!(add(-5, 5), add_tup((-5, 5)));
    }

    #[test]
    fn test_curry_converts_tupled_to_sequential() {
        let curried = curry(add_tup);
        assert_eq!(curried(3)(4), 7);
        assert_eq!(curried(5)(0), 5);
        // True partial application: fix first arg, reuse the closure
        let add10 = curried(10);
        assert_eq!(add10(1), 11);
        assert_eq!(add10(90), 100);
    }

    #[test]
    fn test_uncurry_converts_sequential_to_tupled() {
        // Pass a closure that returns a closure — the canonical curried style
        let tupled = uncurry(|x: i32| move |y: i32| x + y);
        assert_eq!(tupled((3, 4)), 7);
        assert_eq!(tupled((10, 0)), 10);
        assert_eq!(tupled((-1, 1)), 0);
    }

    #[test]
    fn test_flip_swaps_argument_order() {
        // subtraction: a - b (non-commutative)
        let sub = |a: i32, b: i32| a - b;
        let flipped_sub = flip(sub);
        // flip(sub)(b)(a) = sub(a, b) = a - b
        // flipped_sub(3)(10) = sub(10, 3) = 7
        assert_eq!(flipped_sub(3)(10), 7);

        // Simulate OCaml's `Fun.flip ( / ) 2` — divide by 2
        let halve_fn = flip(|a: i32, b: i32| a / b)(2);
        assert_eq!(halve_fn(20), 10);
        assert_eq!(halve_fn(7), 3); // integer division truncates
    }

    #[test]
    fn test_operator_sections() {
        assert_eq!(double(7), 14);
        assert_eq!(double(0), 0);
        assert_eq!(increment(41), 42);
        assert_eq!(increment(-1), 0);
        assert_eq!(halve(20), 10);
        assert_eq!(halve(7), 3); // integer division
    }

    #[test]
    fn test_pipeline_fold() {
        let pipeline: &[fn(i32) -> i32] = &[double, increment, halve];
        // 6 → *2=12 → +1=13 → /2=6
        assert_eq!(apply_pipeline(pipeline, 6), 6);
        // 10 → *2=20 → +1=21 → /2=10
        assert_eq!(apply_pipeline(pipeline, 10), 10);
        // Empty pipeline is identity
        assert_eq!(apply_pipeline(&[], 42), 42);
    }

    #[test]
    fn test_celsius_formula_boundary_values() {
        // 32°F = freezing: 32*5 - 160 = 0  (0/9 = 0°C)
        assert_eq!(celsius_of_fahrenheit(32), 0);
        // 212°F = boiling: 212*5 - 160 = 900  (900/9 = 100°C)
        assert_eq!(celsius_of_fahrenheit(212), 900);
        // scale_and_shift is a general linear transform
        assert_eq!(scale_and_shift(2, 3, 5), 13); // 5*2+3=13
    }
}
