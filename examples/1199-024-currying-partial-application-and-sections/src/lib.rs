#![allow(dead_code)]

//! Currying, partial application, and operator sections in Rust vs OCaml.
//!
//! OCaml curries ALL functions by default — `let add x y = x + y` is sugar
//! for `fun x -> fun y -> x + y`. Rust requires explicit closures, but reaches
//! the same expressiveness. This module shows every pattern side-by-side.

// ── 1. Curried add: direct OCaml parallel ──────────────────────────────────
//
// OCaml: let add x y = x + y
//        let add5 = add 5
//
// Rust cannot auto-curry, so we return a closure explicitly.

/// Curried add: `add(5)` returns a function that adds 5 to any i32.
pub fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// ── 2. Tupled style ────────────────────────────────────────────────────────
//
// OCaml: let add_tup (x, y) = x + y
//
// OCaml tupled functions are NOT curried; Rust tuple-arg functions are the
// same — one call, no partial application.

/// Tupled add: takes `(i32, i32)` as a single argument.
pub fn add_tupled((x, y): (i32, i32)) -> i32 {
    x + y
}

// ── 3. curry / uncurry ─────────────────────────────────────────────────────
//
// OCaml: let curry   f x y = f (x, y)   (* tupled -> curried *)
//        let uncurry f (x, y) = f x y   (* curried -> tupled *)
//
// Rust can't express `impl Fn(A) -> impl Fn(B) -> C` in stable syntax, so we
// use `Box<dyn Fn>` for the inner step. Function items implement `Copy`, which
// lets us avoid Arc/Rc for the common case.

/// Convert a tupled function into a curried form.
///
/// Returns a closure `A -> Box<dyn Fn(B) -> C>`.
/// The `Box` is needed because stable Rust can't name the inner closure type.
pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    F: Fn((A, B)) -> C + Copy + 'static,
    A: Copy + 'static,
    B: 'static,
    C: 'static,
{
    move |x: A| Box::new(move |y: B| f((x, y)))
}

/// Convert a curried function (returning `Box<dyn Fn>`) into a tupled form.
///
/// Works with the output of [`curry`].
/// OCaml: `let uncurry f (x, y) = f x y`
pub fn uncurry<A, B, C>(f: impl Fn(A) -> Box<dyn Fn(B) -> C> + 'static) -> impl Fn((A, B)) -> C {
    move |(x, y)| f(x)(y)
}

// ── 4. Operator sections ────────────────────────────────────────────────────
//
// OCaml: let double    = ( * ) 2         (* section: fix left arg of * *)
//        let increment = ( + ) 1
//        let halve     = Fun.flip ( / ) 2  (* flip fixes RIGHT arg *)
//
// Rust: partial application via closures. `Fun.flip` is just argument reorder.

/// Multiply-by-n factory — equivalent to OCaml's `( * ) n` operator section.
pub fn multiply(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

/// Divide-into-n — equivalent to OCaml's `Fun.flip ( / ) n`.
///
/// `divide_by(2)(20) == 10`  (divides the *argument* by n, not n by argument)
pub fn divide_by(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x / n
}

// Usage:
//   let double    = multiply(2);
//   let increment = add(1);
//   let halve     = divide_by(2);

// ── 5. Labeled arguments → curried factories ───────────────────────────────
//
// OCaml has labeled `~param` syntax so you can partially apply in any order:
//   let scale_and_shift ~scale ~shift x = x * scale + shift
//   let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)
//
// Rust has no labeled args. We emulate ordered partial application instead.

/// Curried scale-and-shift: fixes `scale` and `shift`, returns a transformer.
///
/// OCaml: `let scale_and_shift ~scale ~shift x = x * scale + shift`
pub fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}

/// Fahrenheit → approximate Celsius using `scale_and_shift`.
///
/// OCaml: `let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)`
///
/// Note: integer approximation — `(32°F) * 5 - 160 = 0`, `(212°F) * 5 - 160 = 900`.
/// The exact formula is `(F - 32) * 5 / 9`; this example shows the *pattern*,
/// not a production-quality converter.
pub fn celsius_of_fahrenheit() -> impl Fn(i32) -> i32 {
    scale_and_shift(5, -160)
}

// ── 6. Pipeline via fold ────────────────────────────────────────────────────
//
// OCaml: let pipeline = [double; increment; halve]
//        let result = List.fold_left (fun acc f -> f acc) 6 pipeline
//
// OCaml's homogeneous function list maps to `&[&dyn Fn(i32) -> i32]` in Rust.

/// Apply a pipeline of transformations via left-fold.
///
/// Mirrors OCaml's `List.fold_left (fun acc f -> f acc) initial pipeline`.
pub fn pipeline(initial: i32, transforms: &[&dyn Fn(i32) -> i32]) -> i32 {
    transforms.iter().fold(initial, |acc, f| f(acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── add / partial application ──────────────────────────────────────────

    #[test]
    fn test_partial_application_add5() {
        let add5 = add(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-5), 0);
    }

    #[test]
    fn test_add_tupled() {
        assert_eq!(add_tupled((3, 4)), 7);
        assert_eq!(add_tupled((0, 0)), 0);
        assert_eq!(add_tupled((-1, 1)), 0);
        assert_eq!(add_tupled((-3, -4)), -7);
    }

    // ── curry / uncurry ───────────────────────────────────────────────────

    #[test]
    fn test_curry_converts_tupled_to_curried() {
        let curried = curry(add_tupled);
        assert_eq!(curried(3)(4), 7);
        assert_eq!(curried(0)(0), 0);
        assert_eq!(curried(-1)(1), 0);
    }

    #[test]
    fn test_uncurry_converts_back_to_tupled() {
        let tupled = uncurry(curry(add_tupled));
        assert_eq!(tupled((3, 4)), 7);
        assert_eq!(tupled((0, 0)), 0);
        assert_eq!(tupled((-5, 5)), 0);
        assert_eq!(tupled((10, 20)), 30);
    }

    // ── operator sections ─────────────────────────────────────────────────

    #[test]
    fn test_operator_sections_double_increment_halve() {
        let double = multiply(2);
        let increment = add(1);
        let halve = divide_by(2);

        assert_eq!(double(7), 14);
        assert_eq!(increment(9), 10);
        assert_eq!(halve(20), 10);
        assert_eq!(halve(21), 10); // integer division truncates
    }

    // ── pipeline ──────────────────────────────────────────────────────────

    #[test]
    fn test_pipeline_fold() {
        // OCaml: 6 |> double |> increment |> halve = (6*2+1)/2 = 13/2 = 6
        let double = multiply(2);
        let increment = add(1);
        let halve = divide_by(2);

        let result = pipeline(6, &[&double, &increment, &halve]);
        assert_eq!(result, 6); // 6→12→13→6 (integer division)
    }

    #[test]
    fn test_pipeline_empty() {
        assert_eq!(pipeline(42, &[]), 42);
    }

    // ── scale_and_shift / labeled args ────────────────────────────────────

    #[test]
    fn test_celsius_of_fahrenheit_fixed_points() {
        let to_c = celsius_of_fahrenheit();
        // 32°F → 32*5-160 = 0   (water freezing — correct!)
        assert_eq!(to_c(32), 0);
        // 212°F → 212*5-160 = 900  (integer approximation — pattern demo)
        assert_eq!(to_c(212), 900);
    }

    #[test]
    fn test_scale_and_shift_generic() {
        // identity: scale=1, shift=0
        let id = scale_and_shift(1, 0);
        assert_eq!(id(42), 42);

        // double-and-add-three
        let f = scale_and_shift(2, 3);
        assert_eq!(f(5), 13); // 5*2+3
        assert_eq!(f(0), 3);
    }
}
