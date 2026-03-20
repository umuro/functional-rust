📖 **[View on hightechmind.io →](https://hightechmind.io/rust/268-iterator-unzip)**

---

# 268: Splitting Pairs with unzip()

## Problem Statement

Data often arrives paired — key-value entries, coordinate pairs, or associated data — but needs to be split into separate collections for independent processing. The `unzip()` adapter is the exact inverse of `zip()`: it consumes an iterator of pairs and distributes them into two separate collections simultaneously, in a single pass. This is more efficient than collecting all pairs first and then splitting.

## Learning Outcomes

- Understand `unzip()` as the inverse of `zip()` — splitting `Iterator<(A, B)>` into `(Vec<A>, Vec<B>)`
- Recognize that `unzip()` operates in a single pass without intermediate storage
- Use `unzip()` to separate keys from values, x-coordinates from y-coordinates
- Apply `unzip()` after `map()` to transform and simultaneously split data

## Rust Application

`Iterator::unzip()` collects into two separate `FromIterator` collections simultaneously. It requires specifying the output types, which Rust infers from context:

```rust
let pairs = vec![(1i32, 'a'), (2, 'b'), (3, 'c')];
let (nums, chars): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
// nums = [1, 2, 3], chars = ['a', 'b', 'c']

// Roundtrip: zip then unzip recovers original collections
let a = vec![1i32, 2, 3];
let b = vec![4i32, 5, 6];
let (a2, b2): (Vec<i32>, Vec<i32>) = a.iter().copied().zip(b.iter().copied()).unzip();
assert_eq!(a, a2);
```

## OCaml Approach

OCaml provides `List.split` which is exactly `unzip` for lists of pairs:

```ocaml
let (firsts, seconds) = List.split [(1,'a'); (2,'b'); (3,'c')]
(* firsts = [1;2;3], seconds = ['a';'b';'c'] *)
```

This is strict (builds both lists in one pass) and is a standard library function, unlike `zip` / `combine` in OCaml.

## Key Differences

1. **Name**: Rust calls it `unzip()`; OCaml calls it `List.split`; Haskell calls it `unzip` — all are identical in semantics.
2. **Generic output**: Rust's `unzip()` works into any `FromIterator`-implementing collection; OCaml's `List.split` is list-specific.
3. **Single pass**: Both Rust and OCaml implement unzip in a single pass over the pairs, building both outputs simultaneously.
4. **Transform then split**: The common pattern is `map(...).unzip()` — transform pairs and split in one composed operation.

## Exercises

1. Parse a list of `"key=value"` strings, splitting on `=`, and use `unzip()` to collect keys and values into separate `Vec<String>` collections.
2. Given a list of 2D points as `(f64, f64)` pairs, use `unzip()` to get separate `x` and `y` coordinate vectors for plotting.
3. Use `enumerate()` followed by `unzip()` to simultaneously extract indices and values from a slice.
