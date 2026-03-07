// Pure functions for integers
pub fn double(x: i32) -> i32 {
    2 * x
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Pure functions for strings
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn add_exclaim(s: &str) -> String {
    format!("{}!", s)
}

// Idiomatic Rust: direct function composition
// This is the natural way to compose functions in Rust.
// We apply functions left-to-right by nesting function calls.
pub fn compute_result_idiomatic() -> i32 {
    add_one(double(5)) // 11
}

pub fn compute_greeting_idiomatic() -> String {
    add_exclaim(&shout("hello")) // "HELLO!"
}

// Functional Rust: pipe function (mimics OCaml's |>)
// Takes a value and applies a function to it.
// In OCaml: let (|>) x f = f x
// This shows the explicit function application order.
pub fn pipe<T, U>(value: T, f: impl FnOnce(T) -> U) -> U {
    f(value)
}

// Using pipe with nested calls to show the transformation chain
pub fn compute_result_with_pipe() -> i32 {
    pipe(pipe(5, double), add_one) // 11
}

pub fn compute_greeting_with_pipe() -> String {
    let shouted = pipe("hello", shout);
    pipe(&shouted, |s| add_exclaim(s)) // "HELLO!"
}

// Composition function: creates a new function from two functions
// This shows function composition, another way to chain transformations
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

pub fn compute_result_with_composition() -> i32 {
    let double_then_add_one = compose(double, add_one);
    double_then_add_one(5) // 11
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        assert_eq!(double(5), 10);
        assert_eq!(double(0), 0);
    }

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(5), 6);
        assert_eq!(add_one(0), 1);
    }

    #[test]
    fn test_compute_result_idiomatic() {
        assert_eq!(compute_result_idiomatic(), 11);
    }

    #[test]
    fn test_compute_result_with_pipe() {
        assert_eq!(compute_result_with_pipe(), 11);
    }

    #[test]
    fn test_compute_result_with_composition() {
        assert_eq!(compute_result_with_composition(), 11);
    }

    #[test]
    fn test_compute_greeting_idiomatic() {
        assert_eq!(compute_greeting_idiomatic(), "HELLO!");
    }

    #[test]
    fn test_compute_greeting_with_pipe() {
        assert_eq!(compute_greeting_with_pipe(), "HELLO!");
    }

    #[test]
    fn test_pipe_with_closures() {
        let result = pipe(5, |x| x * 2);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_shout() {
        assert_eq!(shout("hello"), "HELLO");
        assert_eq!(shout("world"), "WORLD");
    }

    #[test]
    fn test_add_exclaim() {
        assert_eq!(add_exclaim("HELLO"), "HELLO!");
    }
}
