# 308: When to panic vs Return Result

**Difficulty:** 2  **Level:** Intermediate

Know the one rule that tells you which to use — every time.

## The Problem This Solves

New Rust developers often get this wrong in both directions: using `panic!` for things callers should handle (input validation, missing files), or returning `Result` for things that literally cannot happen given correct code (post-assertion invariants, unreachable branches). Both mistakes make code harder to use and harder to reason about.

The confusion comes from treating `panic!` as "throws an exception" — but it isn't. A panic in Rust is a signal that the program has encountered a *bug in the calling code*, not a recoverable runtime condition. It's the equivalent of `assert false` in OCaml or `assert` in C — a programmer error, not a user error.

The rule is simple: **panic for bugs, `Result` for expected failure modes.** If a user can trigger it (bad input, missing file, network timeout), it's a `Result`. If only a programmer can trigger it (passing the wrong index, violating a documented precondition), it's a panic. Libraries should almost never panic — they serve callers who can't recover from unexpected panics.

## The Intuition

Panic = "you called this wrong" (a bug in the caller). `Result` = "this might fail for reasons outside your control" (expected failure).

## How It Works in Rust

```rust
// Result: user-triggered failure — caller decides how to handle
fn parse_age(s: &str) -> Result<u8, String> {
    let n: i32 = s.parse().map_err(|_| format!("'{}' is not a number", s))?;
    if n < 0 || n > 150 {
        return Err(format!("age {} out of range", n));
    }
    Ok(n as u8)
}

// Panic: programmer-triggered failure — documents a contract violation
fn get_first(slice: &[i32]) -> i32 {
    assert!(!slice.is_empty(), "get_first: slice must not be empty");
    slice[0]
}

// unreachable!: a branch the programmer has proven can't be reached
fn classify(n: u8) -> &'static str {
    match n {
        0 => "zero",
        1..=9 => "single digit",
        10..=99 => "double digit",
        _ => "triple digit",
    }
}

// When it's OK to use unwrap():
// - In tests (test failures are expected to be loud)
// - In examples and prototypes
// - When you've already checked the condition and the type system can't see it
// - With expect() to document why it should be safe
let val: i32 = "42".parse().expect("hardcoded literal is always valid");
```

The `expect("reason")` pattern over bare `unwrap()` is important in production: it documents *why* the programmer believes this can't fail, making future debugging easier if they're wrong.

## What This Unlocks

- **Clear API contracts** — `Result` in the signature tells callers "this can fail and you should handle it"; no `Result` + documented panics tells callers "don't call this with invalid input"
- **Library safety** — libraries that never panic (except on documented programmer error) are safe to use in any context
- **Test structure** — `#[should_panic]` tests document exactly which programmer errors trigger panics

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recoverable error | Exception or `Result` type | `Result<T, E>` |
| Programmer bug | `assert false` / `failwith` | `panic!`, `assert!`, `unreachable!` |
| Library code | Exceptions acceptable | `Result` strongly preferred — panics are rarely acceptable |
| Pre-condition violation | Manual assertion | `assert!` or `panic!` with message |
| Tests | OUnit assertion | `#[should_panic]` for expected panics |
