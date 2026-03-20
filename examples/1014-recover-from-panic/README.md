📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1014-recover-from-panic)**

---

# 1014-recover-from-panic — Recover from Panic
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Rust's panics are designed for unrecoverable programming errors, but in server environments you sometimes need to isolate untrusted or user-supplied code: a plugin that panics should not crash the entire process. The `std::panic::catch_unwind` function provides a safety net — it catches a panic before it unwinds past the boundary and converts it to a `Result`.

This is the mechanism that web frameworks, test runners (like the built-in `#[test]` harness), and FFI boundaries use to prevent a single bad computation from bringing down the whole system.

## Learning Outcomes

- Use `std::panic::catch_unwind` to convert a panic into a `Result`
- Understand `UnwindSafe` and why mutable references require `AssertUnwindSafe`
- Extract the panic payload from the returned `Box<dyn Any>` using `downcast_ref`
- Know that `catch_unwind` does not work with `panic = "abort"` builds
- Understand the right contexts for using this API (plugin isolation, test harnesses, FFI)

## Rust Application

`src/lib.rs` demonstrates three approaches. `safe_divide` wraps a closure that may panic in `catch_unwind`, then downcasts the panic payload to a human-readable string. `catch_with_state` wraps a mutable `Vec` in `AssertUnwindSafe` — required because `&mut T` is not `UnwindSafe` by default. `with_quiet_panic` suppresses the default panic output by temporarily installing a no-op panic hook, which is useful in test scenarios where expected panics would otherwise pollute output.

Production use cases include test runners (Rust's `#[test]` harness), wasm sandboxes, and plugin systems where third-party code runs in the host process.

## OCaml Approach

OCaml uses `try ... with` for exception recovery, which is a first-class language feature:

```ocaml
let safe_divide a b =
  try Ok (a / b)
  with Division_by_zero -> Error "division by zero"
```

OCaml exceptions carry structured payloads and can be caught at any level. They are not special: `try` is syntactic sugar for a pattern match on the exception type. There is no `UnwindSafe` concept because OCaml's GC handles all memory.

## Key Differences

1. **First-class vs escape hatch**: OCaml `try/with` is the primary error-handling mechanism for exceptions; Rust's `catch_unwind` is a niche escape hatch not meant for normal control flow.
2. **UnwindSafe**: Rust requires types crossing the unwind boundary to be `UnwindSafe` to prevent unsound state; OCaml has no such restriction.
3. **Panic payload typing**: Rust panics can carry any `'static` value; OCaml exceptions are typed variant constructors.
4. **`panic = "abort"`**: When compiled with `panic = "abort"`, Rust processes terminate immediately on panic and `catch_unwind` has no effect; OCaml always allows exception catching.

## Exercises

1. Write a `run_plugins(plugins: Vec<Box<dyn Fn() -> i32>>) -> Vec<Result<i32, String>>` function that runs each plugin in `catch_unwind` and collects results.
2. Use `std::panic::set_hook` and `take_hook` to capture the panic message (file, line, column) into a string without printing it to stderr.
3. Demonstrate that `catch_unwind` cannot catch panics that cross an FFI boundary by writing a `extern "C"` function that calls a panicking Rust function.
