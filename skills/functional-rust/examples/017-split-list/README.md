# 017: Split List

**Difficulty:** 1  **Level:** Foundations

Split a list into two parts at a given index — like Python's `lst[:n]` and `lst[n:]`.

## The Problem This Solves

You receive a stream of records and need to separate the first N items from the rest — maybe the first page of results vs. the overflow, or the training set vs. the test set. The task: given `['a','b','c','d','e','f','g','h','i','k']` and `n=3`, produce `(['a','b','c'], ['d','e','f','g','h','i','k'])`.

Python makes this trivial with slice syntax: `lst[:n], lst[n:]`. But what if `n` is larger than the list? Python silently returns an empty second part — fine if you know it, a silent bug if you don't.

Rust's slice indexing does the same thing, and the bounds check is *built in*. You can't accidentally go out of bounds — the `.min(lst.len())` guard ensures `n` never exceeds what's available, and the compiler enforces that you handle both parts.

## The Intuition

In Python: `left, right = lst[:n], lst[n:]`

In JavaScript: `[lst.slice(0, n), lst.slice(n)]`

Rust slices work identically. `lst[..n]` is everything before index `n`, and `lst[n..]` is everything from `n` onward. The `.to_vec()` call creates an owned copy — necessary here because the function returns new `Vec`s, not references into the original.

The key safety detail: `n.min(lst.len())` clamps `n` so we never try to index past the end. In Python that's implicit; in Rust you make it explicit — and that explicitness is a feature, not a burden.

## How It Works in Rust

```rust
fn split<T: Clone>(lst: &[T], n: usize) -> (Vec<T>, Vec<T>) {
    let n = n.min(lst.len());   // clamp: never go past end
    (lst[..n].to_vec(), lst[n..].to_vec())
}
```

- `lst[..n]` — slice from start up to (not including) index `n`
- `lst[n..]` — slice from index `n` to the end
- `.to_vec()` — clone the slice into an owned `Vec<T>`
- Returns a tuple `(Vec<T>, Vec<T>)` — both parts together

The recursive version (`split_rec`) builds the left side element by element using an accumulator and returns the right side when the counter hits zero. Same result, more steps — useful for understanding how iteration maps to recursion.

## What This Unlocks

- **Pagination** — split a result list into current page + remaining.
- **Train/test splits** — divide datasets at a fixed index.
- **Chunked processing** — split once, process left, pass right to the next stage.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Slice syntax | `List.filteri` / recursive take+drop | `lst[..n]` and `lst[n..]` |
| Bounds safety | Runtime exception on bad index | `.min(lst.len())` clamp, then safe slice |
| Return type | `'a list * 'a list` (tuple) | `(Vec<T>, Vec<T>)` (tuple) |
| Copying elements | `List.map` with identity | `.to_vec()` — clones the slice |
| Destructuring | `let (l, r) = split ...` | `let (left, right) = split(...)` |
