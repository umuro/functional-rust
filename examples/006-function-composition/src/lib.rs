/// Function Composition: building complex transformations from simple parts.
///
/// OCaml doesn't have a built-in composition operator (though `|>` serves similarly).
/// Rust also lacks one, but closures and method chaining achieve the same goal.

// ── Compose two functions ───────────────────────────────────────────────────

/// Compose f and g: returns a new function that applies g first, then f.
/// Equivalent to mathematical (f ∘ g)(x) = f(g(x))
pub fn compose<A, B, C>(
    f: impl Fn(B) -> C + 'static,
    g: impl Fn(A) -> B + 'static,
) -> Box<dyn Fn(A) -> C> {
    Box::new(move |x| f(g(x)))
}

/// Pipe-style compose: applies f first, then g. More natural reading order.
/// Equivalent to OCaml's `x |> f |> g`
pub fn pipe<A, B, C>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
) -> Box<dyn Fn(A) -> C> {
    Box::new(move |x| g(f(x)))
}

// ── Idiomatic Rust: method chaining via iterators ───────────────────────────

/// Process a list: double, keep evens, sum. Demonstrates iterator composition.
pub fn process(data: &[i64]) -> i64 {
    data.iter().map(|x| x * 2).filter(|x| x % 2 == 0).sum()
}

/// Build a transformation pipeline using fold over functions
pub fn pipeline(value: i64, steps: &[&dyn Fn(i64) -> i64]) -> i64 {
    steps.iter().fold(value, |acc, f| f(acc))
}

// ── Compose multiple functions ──────────────────────────────────────────────

/// Compose a vector of functions into a single function (right-to-left)
pub fn compose_all(funcs: Vec<Box<dyn Fn(i64) -> i64>>) -> Box<dyn Fn(i64) -> i64> {
    Box::new(move |x| funcs.iter().rev().fold(x, |acc, f| f(acc)))
}

/// Pipe a vector of functions (left-to-right)
pub fn pipe_all(funcs: Vec<Box<dyn Fn(i64) -> i64>>) -> Box<dyn Fn(i64) -> i64> {
    Box::new(move |x| funcs.iter().fold(x, |acc, f| f(acc)))
}

// ── Practical: string processing pipeline ───────────────────────────────────

/// Compose string transformations
pub fn process_string(s: &str) -> String {
    // Rust's method chaining IS composition
    s.trim()
        .to_lowercase()
        .replace("  ", " ")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose() {
        let double = |x: i64| x * 2;
        let inc = |x: i64| x + 1;
        let double_then_inc = compose(inc, double); // inc(double(x))
        assert_eq!(double_then_inc(5), 11); // 5*2=10, +1=11
    }

    #[test]
    fn test_pipe() {
        let double = |x: i64| x * 2;
        let inc = |x: i64| x + 1;
        let inc_then_double = pipe(inc, double); // double(inc(x))
        assert_eq!(inc_then_double(5), 12); // 5+1=6, *2=12
    }

    #[test]
    fn test_process_iterator_chain() {
        assert_eq!(process(&[1, 2, 3, 4, 5]), 30); // all doubled are even
        assert_eq!(process(&[]), 0);
        assert_eq!(process(&[7]), 14);
    }

    #[test]
    fn test_pipeline() {
        let add1: &dyn Fn(i64) -> i64 = &|x| x + 1;
        let mul2: &dyn Fn(i64) -> i64 = &|x| x * 2;
        let sub3: &dyn Fn(i64) -> i64 = &|x| x - 3;
        assert_eq!(pipeline(5, &[add1, mul2, sub3]), 9); // (5+1)*2-3=9
    }

    #[test]
    fn test_compose_all() {
        let funcs: Vec<Box<dyn Fn(i64) -> i64>> = vec![
            Box::new(|x| x + 1),
            Box::new(|x| x * 2),
            Box::new(|x| x - 3),
        ];
        // Right-to-left: (x-3)*2+1; for x=10: 7*2+1=15
        let f = compose_all(funcs);
        assert_eq!(f(10), 15);
    }

    #[test]
    fn test_pipe_all() {
        let funcs: Vec<Box<dyn Fn(i64) -> i64>> = vec![
            Box::new(|x| x + 1),
            Box::new(|x| x * 2),
            Box::new(|x| x - 3),
        ];
        // Left-to-right: ((x+1)*2)-3; for x=10: 11*2-3=19
        let f = pipe_all(funcs);
        assert_eq!(f(10), 19);
    }

    #[test]
    fn test_process_string() {
        assert_eq!(process_string("  Hello  WORLD!  "), "hello world");
        assert_eq!(process_string(""), "");
        assert_eq!(process_string("Rust 2024"), "rust 2024");
    }
}
