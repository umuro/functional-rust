📖 **[View on hightechmind.io →](https://hightechmind.io/rust/280-hamming-distance)**

---

# Example 280: Hamming Distance — Generic Zip
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Compute the Hamming distance between two strings: the number of positions where corresponding characters differ. Return an error if the strings have different lengths.

## Learning Outcomes

- Using `Result<T, E>` for fallible computations (mirrors OCaml's `result` type)
- Iterator `zip` + `filter` + `count` as a clean functional pipeline
- Comparing imperative (`mut` counter) vs functional (fold/filter) styles
- Early return with `?` operator vs explicit match

## OCaml Approach

OCaml offers two styles: an imperative version using `ref` (mutable reference) with `String.iteri`, and a pure functional version using `Seq.zip` with `Seq.fold_left`. Both return `result` (Ok/Error) for the length check.

## Rust Approach

Rust's idiomatic version chains `.chars().zip().filter().count()` — no mutable state needed. The `Result` type maps directly to OCaml's `result`. An imperative version with `mut dist` and a fold version are also shown for comparison.

## Key Differences

1. **Result type:** OCaml's `Ok/Error` maps directly to Rust's `Ok/Err` — nearly identical usage
2. **Zip:** OCaml's `Seq.zip` and Rust's `Iterator::zip` are functionally equivalent; Rust's is a method on any iterator
3. **String iteration:** OCaml uses `String.iteri` (index + char) or `String.to_seq`; Rust uses `.chars()` which returns a char iterator directly
4. **Filter+count vs fold:** Rust's `.filter(predicate).count()` is more readable than `.fold(0, |acc, ...| ...)` for counting — both work, but filter+count communicates intent better

## Exercises

1. Generalize `hamming_distance` to work on any two `Iterator<Item: PartialEq>` of the same length, returning `Err` if lengths differ.
2. Implement `nearest_neighbor` that takes a query sequence and a list of candidates, returning the candidate with the smallest Hamming distance.
3. Build a simple error-correcting code using Hamming(7,4): encode a 4-bit data word into a 7-bit codeword, introduce a single-bit error, and implement the decoder that detects and corrects the error using parity checks.
