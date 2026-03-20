📖 **[View on hightechmind.io →](https://hightechmind.io/rust/011-modified-rle)**

---

# 011 — Modified Run-Length Encoding

## Problem Statement

Standard run-length encoding always emits `(count, element)` pairs, but this is wasteful for singleton runs: `(1, 'a')` is less clear than just `'a'`. The modified encoding (OCaml 99 Problems #11) uses a sum type: elements with count > 1 are represented as `Many(count, element)`, while singletons are represented as `One(element)`. This avoids redundancy and makes the encoding self-describing.

This problem demonstrates how algebraic data types (enums with data) replace verbose class hierarchies. The `RleItem<T>` enum is the Rust/OCaml equivalent of a sealed interface with two implementations in Java. Modified RLE is used in fax transmission (CCITT T.4), BMP file compression, and PCX image format.

## Learning Outcomes

- Use a generic enum to represent a sum type (`One` vs `Many`)
- Implement encoding in both iterative (stateful loop) and recursive styles
- Apply the run-detection pattern: scan for runs, then emit the appropriate variant
- Understand how enums with data replace class hierarchies from OOP languages
- Use `PartialEq + Clone` trait bounds for generic types

## Rust Application

The `RleItem<T>` enum with `One(T)` and `Many(usize, T)` variants is the central data type. `encode_modified` uses a stateful loop: `count` tracks the current run length, and at each boundary it emits either `One` or `Many`. The recursive `encode_modified_recursive` uses a helper `pack_run` that finds the extent of each run using `position()`, then recursively encodes the remainder. Both approaches are O(n) time and O(n) space.

## OCaml Approach

OCaml defines `type 'a rle = One of 'a | Many of int * 'a`. The encoding function matches on lists: `let rec encode = function | [] -> [] | x :: xs -> let (run, rest) = span (fun y -> y = x) xs in let item = if run = [] then One x else Many (1 + List.length run, x) in item :: encode rest`. The `span` function splits the list at the first element that does not match — a common functional idiom.

## Key Differences

1. **Generic enum**: Rust's `RleItem<T>` is a generic enum with a type parameter. OCaml's `'a rle` uses a type variable directly in the variant definition — the same concept, different syntax.
2. **Clone bound**: Rust requires `T: Clone` when the encode function clones elements into results. OCaml's GC shares structure automatically; no explicit cloning.
3. **Run detection**: Rust uses indexed access (`list[i] == list[i-1]`) in the loop. OCaml's `span` (not in stdlib but easily defined) splits a list by predicate — a higher-level abstraction.
4. **Memory layout**: Rust's `Vec<RleItem<T>>` stores enum variants inline with a discriminant tag. OCaml boxes variant values on the heap.

## Exercises

1. **Round-trip**: Write a `decode_modified(encoded: &[RleItem<T>]) -> Vec<T>` function and verify that `decode_modified(encode_modified(v)) == v` for any input.
2. **Compression ratio**: Write `compression_ratio(original: &[i32]) -> f64` that returns `original.len() as f64 / encode_modified(original).len() as f64`. What input maximizes/minimizes compression?
3. **Stream encode**: Rewrite `encode_modified` to accept `impl Iterator<Item=T>` and return `impl Iterator<Item=RleItem<T>>` without collecting to a `Vec` first.
