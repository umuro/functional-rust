# Example 087: Series — Sliding Window

**Difficulty:** ⭐⭐
**Category:** String Processing
**OCaml Source:** Classic sliding-window pattern over strings

## Problem Statement

Generate all contiguous substrings of length `n` from a string (sliding window), then find the window whose digits have the largest product.

## Learning Outcomes

- How Rust's `slice::windows()` provides zero-copy sliding windows over byte slices
- Converting byte windows back to `&str` with `std::str::from_utf8`
- Chaining iterator adaptors (`windows`, `map`, `max`) as a direct parallel to OCaml's `List.init` + `List.map` + `List.fold_left`
- Using `Result<T, String>` for domain errors vs. panicking

## OCaml Approach

OCaml uses `List.init` with `String.sub` to build the list of windows eagerly, then pipes through `List.map` and `List.fold_left` to find the maximum product. `String.fold_left` computes each window's product character by character.

## Rust Approach

Rust's standard library provides `slice::windows(n)` directly on `&[u8]`, producing overlapping sub-slices with no allocation. Each sub-slice is re-interpreted as `&str` via `from_utf8`, then mapped to a product via `.chars().map(...).product()`. The max is found with `.max()`.

## Key Differences

1. **Window generation:** OCaml uses `List.init` + `String.sub` (allocates each window); Rust uses `slice::windows()` which yields borrowed sub-slices (zero-copy until `.to_owned()`).
2. **Product computation:** OCaml's `String.fold_left` with manual accumulator vs. Rust's `.product::<u64>()` iterator method — same semantics, more declarative in Rust.
3. **Error handling:** OCaml returns `Ok`/`Error` via variant construction; Rust's `Result<u64, String>` is the idiomatic equivalent with `?` propagation available.
4. **Integer width:** OCaml uses arbitrary-precision integers by default; Rust requires an explicit type (`u64`) to avoid overflow on long digit strings.
