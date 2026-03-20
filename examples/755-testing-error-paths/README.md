📖 **[View on hightechmind.io →](https://hightechmind.io/rust/755-testing-error-paths)**

---

# 755-testing-error-paths — Testing Error Paths
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Happy-path tests are necessary but insufficient. Error paths — malformed input, out-of-range values, empty fields, resource exhaustion — are where bugs hide and security vulnerabilities lurk. Testing error paths requires asserting on specific error variants, not just that an error occurred. Rust's `Result` and rich error enums make error path testing natural and exhaustive, unlike exception-based languages where error type testing requires awkward catch-and-inspect patterns.

## Learning Outcomes

- Assert specific `Err` variants using `assert!(matches!(result, Err(ParseError::Empty)))` or exhaustive pattern matching
- Test every error variant in a `ParseError` enum
- Verify error `Display` messages contain user-friendly text
- Write boundary tests that probe the edges of valid/invalid ranges
- Use `assert_eq!` on `Result` values directly when `PartialEq` is derived

## Rust Application

`parse_positive` returns `Result<u32, ParseError>` where `ParseError` has four variants: `Empty`, `TooLong`, `InvalidChar { ch, pos }`, and `OutOfRange`. Tests cover: empty string, string of length 11 (just over the 10-char limit), non-digit characters at various positions, and zero and u32::MAX+1 as out-of-range values. Each test asserts the specific variant and (for struct variants) the specific field values. Display tests verify error messages are human-readable.

## OCaml Approach

OCaml error testing uses `match` on `result` values or `Alcotest.check (Alcotest.result ...)`. `QCheck` generates random inputs for error paths, not just happy paths. OCaml's variant pattern matching is exhaustive: if you add a new error variant, the compiler forces you to handle it in all match expressions including tests. `Result.get_error` and `Result.is_error` provide imperative-style checks when pattern matching is verbose.

## Key Differences

1. **Pattern matching**: Rust's `assert!(matches!(r, Err(ParseError::TooLong { len: 11, .. })))` is concise; OCaml uses `match r with Error (TooLong 11) -> () | _ -> assert_failure "expected TooLong"`.
2. **Exhaustiveness**: Both languages enforce exhaustive matching on error variants, so adding a new variant forces test updates.
3. **Partial equality**: Rust's `#[derive(PartialEq)]` enables `assert_eq!(parse(""), Err(ParseError::Empty))`; OCaml requires custom equality functions.
4. **Error chaining**: Rust's `anyhow`/`thiserror` support error wrapping chains; OCaml's `Error_monad` (Tezos) does the same.

## Exercises

1. Add a `TooShort { len: usize, min: usize }` error variant to `ParseError` and write tests for the minimum length boundary. Update all `Display` and test code.
2. Write a table-driven test that covers 20+ cases for `parse_positive` using a `Vec<(&str, Result<u32, ParseError>)>` — verify each pair in a loop.
3. Add a `DeprecatedFormat` error that is non-fatal (produces a warning but returns `Ok`), and write tests that verify the result contains both the value and the warning.
