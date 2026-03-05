📖 **[View on hightechmind.io →](https://hightechmind.io/rust/746-doc-test-patterns)**

---

# 746: Documentation Tests: rustdoc Examples

**Difficulty:** 2  **Level:** Intermediate

Code examples in `///` doc comments are compiled and executed as tests — documentation can never go out of date.

## The Problem This Solves

API documentation that lies is worse than no documentation. In most languages, code examples in docs are prose — they drift as the API evolves and nobody notices until a confused user opens an issue. Rust solves this at the tooling level: every ```` ```rust ```` block in a `///` comment is compiled and run as a test by `cargo test`. If the example breaks, the build breaks.

This makes doc tests the ideal place to show intended usage — they serve as both human-readable documentation and machine-verified examples. The pattern is especially valuable for public library APIs where the examples in the crate documentation become the first thing users try to copy-paste.

Doc tests also catch a subtle class of regression: API that compiles but produces wrong output. By `assert_eq!`-ing the expected values, you're testing behavior, not just compilation. Combined with regular unit tests in `#[cfg(test)]` modules, doc tests provide a complementary layer focused on user-facing behavior.

## The Intuition

Rust's `rustdoc` parses `///` comments, extracts code blocks, wraps each one in a test harness, and compiles them as if they were in a `#[test]` function. The `# use example::func;` lines (prefixed with `#`) are included in compilation but hidden in rendered documentation — a clean way to add setup without cluttering the visible example. `cargo test` runs doc tests alongside unit tests.

## How It Works in Rust

```rust
/// Clamps `x` to the inclusive range `[lo, hi]`.
///
/// # Examples
///
/// ```
/// # use example::clamp;          // hidden in docs, needed for compilation
/// assert_eq!(clamp(0, 10, -5), 0);
/// assert_eq!(clamp(0, 10,  5), 5);
/// assert_eq!(clamp(0, 10, 15), 10);
/// ```
pub fn clamp(lo: i32, hi: i32, x: i32) -> i32 {
    x.max(lo).min(hi)
}

/// Panics on invalid input — shown with `should_panic`.
///
/// ```should_panic
/// example::factorial(0); // this line panics — test passes if it does!
/// ```
pub fn factorial(n: u64) -> u64 {
    if n == 0 { panic!("factorial(0) is undefined") }
    (1..=n).product()
}

/// Returns `Err` if divisor is zero.
///
/// ```
/// # use example::safe_div;
/// assert_eq!(safe_div(10, 2), Ok(5));
/// assert_eq!(safe_div(10, 0), Err("division by zero"));
/// ```
pub fn safe_div(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 { Err("division by zero") } else { Ok(a / b) }
}
```

Key patterns: `# use ...` imports are hidden but required; `should_panic` tests that panicking behavior is documented AND verified; `Ok(5)` and `Err(...)` in assertions test the full return type shape.

## What This Unlocks

- **Self-verifying API docs** — every public function's usage example is a live test; refactoring is safe because broken examples fail `cargo test`.
- **Complementary test layers** — doc tests cover "does this do what it says on the tin?"; unit tests in `#[cfg(test)]` cover edge cases and internals; together they give full coverage of behavior.
- **`should_panic` and `compile_fail`** — doc tests support special attributes (`should_panic`, `compile_fail`, `no_run`) that let you document and verify panic conditions, type errors, and untestable-at-runtime behavior.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Executable doc examples | `odoc` doesn't run examples by default | `cargo test` always runs doc tests |
| Hidden setup lines | Not supported | `# use crate::foo;` — `#` prefix hides line in rendered docs |
| Expected-panic testing | Manual `try`/`catch` in tests | ```` ```should_panic ```` attribute on the code block |
| Doc test discovery | Requires explicit `ocamldoc` setup | Automatic — any `///` block is found by `rustdoc` |
