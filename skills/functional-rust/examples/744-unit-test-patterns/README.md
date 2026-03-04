# 744: Unit Test Organisation: Modules, Helpers, AAA Pattern

**Difficulty:** 2  **Level:** Intermediate

Tests live in the same file as production code, gated by `#[cfg(test)]`. No external framework required.

## The Problem This Solves

In many languages, testing requires setting up a test runner, installing a framework, and organizing test files separately from production code. In Python you configure pytest; in JavaScript you wire up Jest. This overhead means beginners often skip testing, and even experienced developers delay writing tests for utility functions.

Rust ships its own test runner — no dependencies needed. You annotate a function with `#[test]` and run `cargo test`. Tests compile only in test mode (`#[cfg(test)]`), so they have zero impact on your release binary. The standard macros `assert_eq!`, `assert!`, and `#[should_panic]` cover almost everything.

The bigger challenge is keeping tests readable as a codebase grows. The **Arrange-Act-Assert** pattern (AAA) and shared helper modules solve this. Helpers like `assert_approx_eq` and `assert_sorted` live in a `helpers` sub-module under `#[cfg(test)]`, so they're available to all tests but stripped from production builds.

## The Intuition

Think of Python's `unittest.TestCase` — but without the class boilerplate. In Rust, a test is just a free function annotated with `#[test]`. Tests in the same module can see private functions; tests in a `mod tests { use super::*; }` block see the public API.

The `#[cfg(test)]` attribute is Rust's conditional compilation: that whole module only exists when you run `cargo test`. It's like `if __name__ == "__main__"` in Python, but enforced at the type level.

Tests run in parallel by default. If a test panics, it fails. `assert_eq!(a, b)` panics with a helpful diff message when `a != b`.

## How It Works in Rust

```rust
// Production code — publicly accessible
pub fn clamp(lo: i32, hi: i32, x: i32) -> i32 {
    x.max(lo).min(hi)
}

// Test helpers — only compiled during `cargo test`
#[cfg(test)]
mod helpers {
    pub fn assert_approx_eq(a: f64, b: f64, eps: f64) {
        assert!((a - b).abs() < eps,
            "assert_approx_eq: |{} - {}| >= {}", a, b, eps);
    }
}

// Unit tests — access production code via `use super::*`
#[cfg(test)]
mod tests {
    use super::*;
    use helpers::*;

    // Arrange-Act-Assert pattern with a descriptive name
    #[test]
    fn test_clamp_when_below_lo_returns_lo() {
        let (lo, hi, x) = (0, 10, -5);      // Arrange
        let result = clamp(lo, hi, x);       // Act
        assert_eq!(result, 0);               // Assert
    }

    // For expected panics — specify the panic message substring
    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_integer_division_by_zero_panics() {
        let _ = 5u32 / 0;
    }
}
```

Key points:
- `mod tests` is conventional, but any name works
- `use super::*` brings all items from the parent module into scope
- Test names appear in `cargo test` output — be descriptive
- `assert_eq!` prints both values on failure; `assert!` accepts a message format string
- `#[should_panic(expected = "...")]` checks that the panic message contains the given substring

## What This Unlocks

- **Zero-friction testing**: write a function, add `#[test]` below it, run `cargo test` — done
- **Shared test utilities**: `#[cfg(test)]` helper modules let you define `assert_approx_eq`, fixture builders, and test data generators without polluting production code
- **Parallel test execution by default**: `cargo test -- --test-threads=1` to serialize when tests share global state

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Test declaration | `let () = Alcotest.run ...` or ppx annotation | `#[test]` attribute — built in |
| Test isolation | Separate `_test.ml` files by convention | `#[cfg(test)] mod tests` in same file |
| Assertion | `Alcotest.(check int) "msg" expected actual` | `assert_eq!(actual, expected)` |
| Expected panic | Manual `try`/exception match | `#[should_panic(expected = "...")]` |
| Parallel execution | Sequential by default | Parallel by default |
| Test helpers | Regular functions in test file | `#[cfg(test)] mod helpers` — stripped from release |
