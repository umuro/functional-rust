📖 **[View on hightechmind.io →](https://hightechmind.io/rust/257-iterator-zip)**

---

# 257: Pairing Elements with zip()

**Difficulty:** 1  **Level:** Beginner

Combine two iterators element-by-element into pairs — stops at the shorter one, never panics on length mismatch.

## The Problem This Solves

You have two parallel sequences and you need to process them together: names and scores, keys and values, timestamps and readings. The naive approach is an index loop: `for i in 0..n { process(a[i], b[i]); }` — but you need to know `n`, you need bounds checking, and the intent ("process in parallel") is buried in the mechanics.

`zip()` expresses this directly: "pair up these two sequences, one element at a time." No indices, no bounds, no off-by-one. The result is an iterator of tuples `(a_item, b_item)` that you can pass to any iterator adapter — `map`, `filter`, `collect`, `for_each`.

The length handling is important: `zip()` stops at the shorter iterator. This is the safe choice — you never get an out-of-bounds access or a panic. OCaml's `List.combine` raises `Invalid_argument` on length mismatch. Know which behaviour you need; in most cases, silent truncation is correct.

## The Intuition

Zip is like a zipper: two separate rows of teeth, interlocked one pair at a time. You get as many pairs as the shorter side allows. After that, the zipper stops.

Each call to `next()` on the zip iterator calls `next()` on both inner iterators and pairs the results. If either returns `None`, the zip returns `None`. The whole thing is lazy — nothing is computed until you consume the zip.

Building a `HashMap` from two parallel slices (one of keys, one of values) is idiomatic: `keys.into_iter().zip(values).collect::<HashMap<_,_>>()`. This is the canonical Rust idiom for constructing a map from two independent lists.

## How It Works in Rust

```rust
// Iterate two slices in parallel
let names = ["Alice", "Bob", "Carol"];
let scores = [95u32, 87, 92];
for (name, score) in names.iter().zip(scores.iter()) {
    println!("{}: {}", name, score);
}

// Build a HashMap from two iterators — canonical idiom
let keys   = vec!["a", "b", "c"];
let values = vec![1i32, 2, 3];
let map: HashMap<_, _> = keys.into_iter().zip(values).collect();

// zip truncates at the shorter — no panic, no error
let long  = [1i32, 2, 3, 4, 5];
let short = ["x", "y"];
let pairs: Vec<_> = long.iter().zip(short.iter()).collect();
// pairs.len() == 2  (not 5)

// Enumerate is zip with 0..
let items = ["a", "b", "c"];
for (i, item) in items.iter().enumerate() {
    println!("{}: {}", i, item);  // 0:a  1:b  2:c
}
// enumerate() is equivalent to (0..).zip(items.iter())
```

`zip` is lazy: calling `.zip()` doesn't compute anything. Elements are produced only when consumed by a `for` loop, `.collect()`, or another adapter.

## What This Unlocks

- **Parallel iteration** — process two or more sequences together without index arithmetic or manual bounds checking.
- **HashMap construction** — `keys.into_iter().zip(values).collect()` is the idiomatic way to build a map from two parallel lists.
- **Pairing with indices** — `.enumerate()` is built-in zip with `0..`; no manual counter needed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Syntax | `List.combine xs ys` | `xs.iter().zip(ys.iter())` |
| Result type | `('a * 'b) list` — eager | `Zip<IterA, IterB>` — lazy iterator |
| Length mismatch | Raises `Invalid_argument` | Silent truncation at shorter |
| Laziness | Eager (computes all pairs) | Lazy — produces pairs on demand |
| Index-based | Manual `List.nth` or `Array.get` | `.enumerate()` for `(index, item)` |
