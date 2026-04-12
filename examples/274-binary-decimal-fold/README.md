📖 **[View on hightechmind.io →](https://hightechmind.io/rust/274-binary-decimal-fold)**

---

# Example 274: Binary ↔ Decimal Fold
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Convert a binary string (e.g. `"1010"`) to its decimal value using a left fold, and convert a decimal integer back to a binary string using recursion or iteration.

## Learning Outcomes

- How OCaml's `String.fold_left` maps directly to Rust's `Iterator::try_fold`
- Using `try_fold` to accumulate results that may fail (error propagation in folds)
- Translating an OCaml accumulator-based recursion into idiomatic Rust
- Returning `Result<T, E>` instead of `failwith` for error handling

## OCaml Approach

OCaml uses `String.fold_left` to accumulate the decimal value character by character, doubling the accumulator and adding 1 for `'1'` digits. `decimal_to_binary` uses a tail-recursive helper `go` that prepends remainder bits to a string accumulator.

## Rust Approach

Rust's `chars().try_fold()` is the direct equivalent of `String.fold_left` with error support — it short-circuits on `Err` just as `failwith` aborts in OCaml. The recursive `decimal_to_binary` mirrors the OCaml `go` helper using an inner function. An iterative version collects bits into a `Vec` then reverses, which is more cache-friendly.

## Key Differences

1. **Error handling:** OCaml raises an exception (`failwith`); Rust returns `Result<u64, String>` — no panics at the library boundary.
2. **Fold with fallibility:** OCaml `fold_left` has no built-in short-circuit; Rust `try_fold` stops on `Err` immediately.
3. **String building:** OCaml uses `^` (string concatenation) in recursion; Rust uses `format!` or `Vec<u8>` + `collect`.
4. **Integer types:** OCaml uses polymorphic `int`; Rust uses explicit `u64`, preventing negative inputs by type.

## Exercises

1. Generalize the fold-based conversion to support arbitrary base conversions (not just binary↔decimal): implement `to_base_n` and `from_base_n` for any base 2–36.
2. Implement big-integer addition by representing numbers as little-endian `Vec<u8>` digits and using a fold to add digit-by-digit with carry propagation.
3. Write a fold-based parser for simple integer literals that handles decimal, hexadecimal (`0x` prefix), and binary (`0b` prefix) notation, returning `Result<u64, ParseError>`.
