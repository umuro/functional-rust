# 016: Drop Every Nth

**Difficulty:** 1  **Level:** Foundations

Remove every nth element from a list — keeping all the rest.

## The Problem This Solves

Imagine you're sampling audio data and need to downsample by removing every 3rd measurement. Or you're paginating records and want to discard every nth row as a separator. The task: given `['a','b','c','d','e','f','g','h','i','k']` and `n=3`, produce `['a','b','d','e','g','h','k']` — dropping positions 3, 6, 9.

In Python you might write a list comprehension: `[x for i, x in enumerate(lst, 1) if i % n != 0]`. That works, but it's ad-hoc — every team member writes it slightly differently, and nothing stops you from making an off-by-one error silently.

Rust's iterator approach does the same thing with named, composable steps. The compiler checks types at every stage, and the code reads almost like a sentence: *enumerate the list, filter out every nth position, collect the values*.

## The Intuition

In Python: `[x for i, x in enumerate(lst, 1) if i % n != 0]`

In JavaScript: `lst.filter((_, i) => (i + 1) % n !== 0)`

In Rust, iterators work the same way — they're lazy chains that transform data step by step. The key method here is `.enumerate()`, which gives you `(index, value)` pairs, then `.filter()` to keep only what you want, then `.map()` to peel off the index again.

One important detail: Rust indices are 0-based, but this problem uses 1-based counting ("every 3rd" means positions 1, 2, *3*, 4, 5, *6*…). That's why the filter checks `(i + 1) % n != 0`.

## How It Works in Rust

```rust
fn drop_every<T: Clone>(lst: &[T], n: usize) -> Vec<T> {
    if n == 0 {
        return lst.to_vec(); // edge case: no-op
    }
    lst.iter()
        .enumerate()           // (0,'a'), (1,'b'), (2,'c'), ...
        .filter(|(i, _)| (i + 1) % n != 0)  // keep if NOT a multiple of n
        .map(|(_, x)| x.clone())             // discard the index
        .collect()             // gather into Vec<T>
}
```

The `T: Clone` bound means "T must be copyable" — needed because we're extracting values from a reference into a new owned `Vec`. For simple types like `char` or `i32`, this is trivially cheap.

There's also a recursive version in the code (`drop_every_rec`) that mirrors the OCaml style: pattern-match on the list, count down to n, drop that element, reset the counter. Both produce identical results — the iterator version is more idiomatic Rust.

## What This Unlocks

- **Data downsampling** — remove every nth frame from video timestamps, sensor readings, or log entries.
- **Strided iteration** — the inverse pattern (keep every nth) uses `.step_by(n)` instead.
- **Filtering by position** — any "keep/drop based on index" pattern follows this template: enumerate → filter → map → collect.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Enumerate | `List.mapi (fun i x -> (i, x))` | `.enumerate()` on any iterator |
| Filter by index | List comprehension / recursive | `.filter((i, _)` → condition`)` |
| 1-based index | Explicit counter in recursion | `(i + 1)` offset in filter |
| Result type | `'a list` | `Vec<T>` (owned, heap-allocated) |
| Edge case (n=0) | Guard in pattern match | Early `return` before iterator chain |
