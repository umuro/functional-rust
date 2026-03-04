# 091: Zip and Unzip

**Difficulty:** 3  **Level:** Intermediate

Pair two iterators element-by-element with `.zip()`, split paired data back apart with `.unzip()`.

## The Problem This Solves

You often have two parallel sequences — keys and values, x-coordinates and y-coordinates, names and scores — and need to process them together or apart. Doing this manually with index loops is noisy and error-prone when lengths differ.

`.zip()` brings two iterators into lockstep. `.unzip()` tears a sequence of pairs apart into two separate collections. Together they make parallel-data manipulation explicit and safe.

Without these, you're writing `for i in 0..a.len()` guards and hoping both slices are the same length.

## The Intuition

Python programmers know `zip(a, b)` — it pairs elements by position and stops when the shorter one runs out. Rust's `.zip()` does the same. `zip(*pairs)` to unzip in Python corresponds to `.unzip()` in Rust.

One gotcha: Rust's `.zip()` silently stops at the shorter iterator. If you need to detect length mismatches, check lengths explicitly beforehand or use a manual `zip_longest` implementation.

## How It Works in Rust

```rust
// Pair two slices → Vec of tuples
fn zip_demo(a: &[i32], b: &[&str]) -> Vec<(i32, String)> {
    a.iter()
     .zip(b.iter())
     .map(|(&n, &s)| (n, s.to_string()))
     .collect()
}

// Split pairs back into two Vecs — unzip()
fn unzip_demo(pairs: &[(i32, &str)]) -> (Vec<i32>, Vec<String>) {
    pairs.iter()
         .map(|&(n, s)| (n, s.to_string()))
         .unzip()  // returns (Vec<i32>, Vec<String>) in one pass
}

// zip as map2 — dot product
fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

// zip_longest without external crates
fn zip_longest<T: Clone>(a: &[T], b: &[T], da: T, db: T) -> Vec<(T, T)> {
    let len = a.len().max(b.len());
    (0..len).map(|i| {
        let x = a.get(i).cloned().unwrap_or_else(|| da.clone());
        let y = b.get(i).cloned().unwrap_or_else(|| db.clone());
        (x, y)
    }).collect()
}
```

`.enumerate()` is `.zip(0..)` with index first — use it when you need position alongside value.

## What This Unlocks

- **Pairwise operations**: dot products, string interpolation from two lists, scoring tables.
- **Configuration unpacking**: split a `Vec<(Key, Value)>` back into separate maps in one pass.
- **Lookahead patterns**: `iter.zip(iter.skip(1))` to compare consecutive elements without `.windows(2)`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Zip | `List.combine` (raises on uneven) | `.zip()` (stops at shorter) |
| Unzip | `List.split` | `.unzip()` |
| zip_with / map2 | `List.map2 f` | `.zip().map(\|(a,b)\| f(a,b))` |
| Enumerate | `List.mapi` | `.enumerate()` |
| zip_longest | Manual | Manual or `itertools` crate |
