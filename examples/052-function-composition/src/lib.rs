#![allow(clippy::all)]
//! # Function Composition
//! OCaml CS3110 — Compose two functions into a pipeline: `compose f g x = f(g(x))`.

/// Idiomatic Rust: generic `compose` returns a closure, mirroring OCaml's `let compose f g x = f (g x)`.
///
/// The returned closure captures `f` and `g` by move, so they must be `'static` if stored.
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

/// Pipeline style: `pipe(g, f)` applies `g` first, then `f` — argument order mirrors a data pipeline.
///
/// This is `compose` with the arguments flipped, matching Rust's left-to-right reading convention.
pub fn pipe<A, B, C, F, G>(g: G, f: F) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

/// Trait-based composition: extend any `Fn` with a `.compose_with` combinator.
pub trait Compose<A, B>: Fn(A) -> B + Sized {
    /// Returns a new closure that applies `self` first, then `next`.
    fn then_apply<C, H: Fn(B) -> C>(self, next: H) -> impl Fn(A) -> C {
        move |x| next(self(x))
    }
}

impl<A, B, F: Fn(A) -> B> Compose<A, B> for F {}

// Standalone helpers used in tests and OCaml comparison.
pub fn double(x: i32) -> i32 {
    2 * x
}

pub fn square(x: i32) -> i32 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- compose ---

    #[test]
    fn test_compose_square_then_double() {
        // OCaml: let square_then_double = compose double square
        let square_then_double = compose(double, square);
        assert_eq!(square_then_double(3), 18); // square(3)=9, double(9)=18
        assert_eq!(square_then_double(4), 32); // square(4)=16, double(16)=32
    }

    #[test]
    fn test_compose_double_then_square() {
        let double_then_square = compose(square, double);
        assert_eq!(double_then_square(3), 36); // double(3)=6, square(6)=36
    }

    #[test]
    fn test_compose_identity() {
        // Composing with identity leaves the function unchanged.
        let identity = |x: i32| x;
        let id_then_double = compose(double, identity);
        assert_eq!(id_then_double(5), 10);
    }

    // --- pipe ---

    #[test]
    fn test_pipe_square_then_double() {
        // pipe(g, f) = apply g first, then f — same result, clearer reading order.
        let square_then_double = pipe(square, double);
        assert_eq!(square_then_double(3), 18);
        assert_eq!(square_then_double(4), 32);
    }

    // --- trait-based ---

    #[test]
    fn test_then_apply_square_then_double() {
        let square_then_double = square.then_apply(double);
        assert_eq!(square_then_double(3), 18);
        assert_eq!(square_then_double(4), 32);
    }

    #[test]
    fn test_compose_with_closures() {
        // Works with anonymous closures, not just named functions.
        let add_one = |x: i32| x + 1;
        let times_three = |x: i32| x * 3;
        let f = compose(times_three, add_one); // add_one first, then times_three
        assert_eq!(f(4), 15); // add_one(4)=5, times_three(5)=15
    }
}
