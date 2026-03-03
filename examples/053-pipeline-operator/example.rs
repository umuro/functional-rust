//! # Pipeline Operator
//! CS3110 — Chaining transformations left-to-right without nesting.

// ---------------------------------------------------------------------------
// 1. Method chaining (native Rust style)
// ---------------------------------------------------------------------------

/// Double a number.
pub fn double(x: i32) -> i32 {
    2 * x
}

/// Add one to a number.
pub fn add1(x: i32) -> i32 {
    x + 1
}

/// Convert a string to uppercase.
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

/// Append an exclamation mark.
pub fn exclaim(s: String) -> String {
    s + "!"
}

// ---------------------------------------------------------------------------
// 2. Trait-based pipe (generic, reusable)
// ---------------------------------------------------------------------------

/// Extension trait that adds `.pipe(f)` to any value.
///
/// Equivalent to OCaml's `|>` operator: `x.pipe(f)` is `f(x)`.
pub trait Pipe: Sized {
    fn pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B {
        f(self)
    }
}

impl<T> Pipe for T {}

// ---------------------------------------------------------------------------
// 3. Macro-based pipe
// ---------------------------------------------------------------------------

/// Chain a value through a sequence of functions left-to-right.
///
/// `pipe!(5 => double, add1)` expands to `add1(double(5))`.
#[macro_export]
macro_rules! pipe {
    ($val:expr => $($f:expr),+ $(,)?) => {{
        let mut v = $val;
        $(v = $f(v);)+
        v
    }};
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Method chaining (free functions called left-to-right) ---------------

    #[test]
    fn test_numeric_pipeline_functions() {
        // 5 |> double |> add1  →  11
        let result = add1(double(5));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_string_pipeline_functions() {
        // "hello" |> shout |> exclaim  →  "HELLO!"
        let result = exclaim(shout("hello"));
        assert_eq!(result, "HELLO!");
    }

    // -- Trait-based pipe ----------------------------------------------------

    #[test]
    fn test_pipe_trait_numeric() {
        let result = 5.pipe(double).pipe(add1);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_pipe_trait_string() {
        let result = "hello".pipe(shout).pipe(exclaim);
        assert_eq!(result, "HELLO!");
    }

    #[test]
    fn test_pipe_trait_closure() {
        let result = 10.pipe(|x| x * 3).pipe(|x| x - 5).pipe(|x| x.to_string());
        assert_eq!(result, "25");
    }

    // -- Macro pipe ----------------------------------------------------------

    #[test]
    fn test_pipe_macro_numeric() {
        let result = pipe!(5 => double, add1);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_pipe_macro_multi_step() {
        // Matches OCaml: 5 |> double |> add1 |> double  →  22
        let result = pipe!(5 => double, add1, double);
        assert_eq!(result, 22);
    }
}

fn main() {
    println!("{:?}", result, 11);
    println!("{:?}", result, "HELLO!");
    println!("{:?}", result, 11);
}
