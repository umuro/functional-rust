📖 **[View on hightechmind.io →](https://hightechmind.io/rust/257-iterator-zip)**

---

# 257: Pairing Elements with zip()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Many algorithms operate on two parallel sequences simultaneously: pairing keys with values, computing dot products, correlating time-series data, or combining two streams element-by-element. The naive approach uses index-based access (`a[i]`, `b[i]`) which is error-prone and requires bounds checking. The `zip()` combinator solves this by producing pairs `(a_i, b_i)` lazily, stopping when the shorter source is exhausted — eliminating off-by-one errors entirely.

## Learning Outcomes

- Understand how `zip()` pairs elements from two iterators, stopping at the shorter one
- Use `zip()` to build `HashMap` from key and value iterators
- Recognize the truncation behavior as a feature for handling mismatched-length sources
- Combine `zip()` with `map()` to compute pairwise operations like dot products

## Rust Application

`Iterator::zip()` takes any `IntoIterator` and returns a `Zip<A, B>` struct. Each `next()` call advances both inner iterators and returns `Some((a, b))` — or `None` if either is exhausted.

```rust
let keys = vec!["a", "b", "c"];
let vals = vec![1i32, 2, 3];
let map: HashMap<_, _> = keys.into_iter().zip(vals).collect();
```

The dot product pattern is particularly clean: `a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()`.

## OCaml Approach

OCaml provides `List.combine` for zipping two lists into a list of pairs, and `List.map2` for applying a binary function pairwise. These are strict and raise `Invalid_argument` on length mismatch, unlike Rust's truncation:

```ocaml
let pairs = List.combine [1;2;3] ["a";"b";"c"]
(* [(1,"a"); (2,"b"); (3,"c")] — strict, raises on unequal lengths *)
let dot = List.fold_left (+) 0 (List.map2 ( * ) a b)
```

## Key Differences

1. **Length mismatch**: Rust's `zip()` silently truncates to the shorter iterator; OCaml's `List.map2` raises `Invalid_argument`.
2. **Laziness**: Rust's `zip()` is lazy; OCaml's `List.combine` immediately creates a new list.
3. **Pair type**: Rust yields Rust tuples `(A, B)` which are unnamed; OCaml produces named-field or anonymous tuples identically.
4. **Unzip**: Rust has `Iterator::unzip()` as the inverse; OCaml uses `List.split`.

## Exercises

1. Zip a list of student names with their grades and compute the average grade using `zip()` and `map()`.
2. Implement a dot product function `dot(a: &[f64], b: &[f64]) -> f64` using `zip()` and `sum()`.
3. Use `zip()` with `enumerate()` to pair every element with both its index and a label from a separate labels slice.
