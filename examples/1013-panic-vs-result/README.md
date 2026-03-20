📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1013-panic-vs-result)**

---

# 1013-panic-vs-result — Panic vs Result
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Every language with explicit error handling faces the same design question: when should an error abort the program versus return a recoverable value? In Go, `panic` and `error` serve different roles. In Rust, `panic!` signals a programming bug (an invariant violation), while `Result<T, E>` represents an expected failure that callers should handle. Conflating the two leads to either over-cautious `unwrap`-heavy code or libraries that silently swallow errors.

The rule of thumb: use `panic!` for logic errors that indicate the program is in an unrecoverable state; use `Result` for operations that legitimately fail in production (file not found, network timeout, bad input from an untrusted source).

## Learning Outcomes

- Distinguish programming bugs (panic) from recoverable failures (Result)
- Use `expect` instead of `unwrap` to give panics meaningful messages
- Know when `debug_assert!` versus `assert!` is appropriate
- Design library APIs that return `Result` and let callers decide how to handle failures
- Understand that `panic!` unwinds the stack and cannot be caught in normal code paths

## Rust Application

`src/lib.rs` shows the contrast directly. `divide_or_panic` panics on zero — a programming contract violation that the caller should never trigger. `divide` returns `Result<i64, String>` — appropriate for a public API where the caller controls the inputs. `first_element` uses `expect` with a descriptive message rather than bare `unwrap`. `parse_positive` chains two checks with `?` and returns `Result`, suitable for parsing user input. `process_data` uses `debug_assert!` for a check that is expensive enough to remove in release builds.

## OCaml Approach

OCaml uses exceptions for recoverable errors and `failwith`/`assert` for bugs:

```ocaml
exception Division_by_zero_user of string

let divide a b =
  if b = 0 then Error "division by zero"
  else Ok (a / b)

let divide_or_raise a b =
  if b = 0 then failwith "programming error: divide by zero"
  else a / b
```

OCaml exceptions are more integrated into the type system than Rust panics — they can be caught with `try ... with` — but idiomatic modern OCaml prefers `Result`.

## Key Differences

1. **Catchability**: OCaml exceptions are always catchable with `try ... with`; Rust panics can only be caught with `std::panic::catch_unwind` and are not recommended for control flow.
2. **Library convention**: Rust library crates are expected to return `Result` for all user-facing failures; panicking in a library is considered poor practice unless it signals a bug.
3. **`expect` messages**: Rust's `expect("reason")` is a convention for explaining why a value should never be `None`/`Err`; OCaml's `Option.value_exn ~message:` fills the same role.
4. **Debug vs release**: Rust's `debug_assert!` disappears in release builds; OCaml has no direct equivalent in the standard library.

## Exercises

1. Refactor `divide_or_panic` into a safe `divide` function and a thin `divide_unchecked` that panics — document the invariant clearly with a comment.
2. Write a function that reads a port number from a string, using `Result` for parsing errors and `panic!` if the port is outside 1–65535 (treat that as a configuration bug).
3. Use `std::panic::catch_unwind` to call `first_element` on an empty slice and verify that the panic is recovered without crashing the test.
