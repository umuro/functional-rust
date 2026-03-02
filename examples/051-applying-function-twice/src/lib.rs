//! # Applying a Function Twice
//! OCaml CS3110 — Higher-order function that applies a function twice,
//! demonstrating currying and partial application.

// ── Implementation 1: Idiomatic Rust ────────────────────────────────────────
//
// Generic over the argument type T and any function F that maps T → T.
// `F: Fn(T) -> T` means f can be called repeatedly (shared reference semantics).
// Ownership: x is moved in, the intermediate result of f(x) is moved into f(…).

/// Apply `f` to `x` twice: `f(f(x))`.
pub fn twice<T, F>(f: F, x: T) -> T
where
    F: Fn(T) -> T,
{
    f(f(x))
}

// ── Implementation 2: Partial application (curried style) ────────────────────
//
// Returns a closure that captures `f` by move.
// This matches OCaml's `let quad = twice double` — bind the function,
// get back a new function waiting for the argument.
//
// `impl Fn(T) -> T` in return position: the compiler knows the concrete
// closure type; we hide it behind `impl Trait` so callers stay generic.

/// Partially apply `twice`: bind `f`, return a new `Fn(T) -> T`.
///
/// Example: `let quad = twice_partial(double);` — then `quad(3) == 12`.
pub fn twice_partial<T, F>(f: F) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
{
    // `f` is moved into the closure.  Because F: Fn (not FnOnce), calling
    // the closure multiple times is safe — f itself is never consumed.
    move |x| f(f(x))
}

// ── Implementation 3: Function-pointer variant ───────────────────────────────
//
// `fn(i32) -> i32` is a bare function pointer, not a closure.
// This is less general (no captured environment) but zero-overhead and
// useful when you only need named free functions like `double` / `square`.

/// Apply a bare function pointer twice (no closure capture).
pub fn twice_fp(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// ── Named helpers matching the OCaml originals ───────────────────────────────

pub fn double(x: i32) -> i32 {
    2 * x
}

pub fn square(x: i32) -> i32 {
    x * x
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // --- twice (generic) ---

    #[test]
    fn test_twice_double() {
        // double(double(3)) = double(6) = 12
        assert_eq!(twice(double, 3), 12);
    }

    #[test]
    fn test_twice_square() {
        // square(square(2)) = square(4) = 16
        assert_eq!(twice(square, 2), 16);
    }

    #[test]
    fn test_twice_with_closure() {
        // Works with any Fn(T) -> T, including closures
        let increment = |x: i32| x + 1;
        assert_eq!(twice(increment, 5), 7);
    }

    #[test]
    fn test_twice_identity() {
        assert_eq!(twice(|x: i32| x, 42), 42);
    }

    // --- twice_partial (curried / partial application) ---

    #[test]
    fn test_partial_quad() {
        // quad = twice double — bind double, get a new function
        let quad = twice_partial(double);
        assert_eq!(quad(3), 12);
        assert_eq!(quad(0), 0);
        assert_eq!(quad(-1), -4);
    }

    #[test]
    fn test_partial_fourth() {
        // fourth = twice square — x^4
        let fourth = twice_partial(square);
        assert_eq!(fourth(2), 16);
        assert_eq!(fourth(3), 81);
    }

    #[test]
    fn test_partial_reusable() {
        // The returned closure can be called multiple times
        let quad = twice_partial(|x: i32| 2 * x);
        let results: Vec<i32> = (1..=4).map(|x| quad(x)).collect();
        assert_eq!(results, vec![4, 8, 12, 16]);
    }

    // --- twice_fp (function pointer) ---

    #[test]
    fn test_fp_double() {
        assert_eq!(twice_fp(double, 3), 12);
    }

    #[test]
    fn test_fp_square() {
        assert_eq!(twice_fp(square, 2), 16);
    }
}
