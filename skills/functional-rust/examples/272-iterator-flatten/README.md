# 272: One-Level Flattening with flatten()

**Difficulty:** 2  **Level:** Intermediate

Collapse exactly one level of iterator nesting — `Vec<Vec<T>>` → `Vec<T>`, `Option<Option<T>>` → `Option<T>`.

## The Problem This Solves

After a `map()` that produces a collection per element, or after a series of operations that produce `Option<Option<T>>` or `Vec<Option<T>>`, you end up with a nested structure you need to unwrap by one level. You could `collect()` and then iterate again, but that allocates an intermediate collection. You could use `flat_map` with `|x| x` — which works but looks redundant.

`flatten()` is the explicit, zero-overhead way to say "remove one layer of nesting." It's the monadic `join` operation — if `flat_map` is `bind`, `flatten` is `join`.

It also works on `Option<Option<T>>` and `Result<Result<T, E>, E>` directly — not just iterators — making it useful for nested fallible operations.

## The Intuition

`flatten()` takes an iterator where each item is itself iterable, and concatenates all the inner iterables into one flat sequence. It removes exactly one level — no more.

```rust
let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
let flat: Vec<i32> = nested.into_iter().flatten().collect();
// → [1, 2, 3, 4, 5, 6]
```

## How It Works in Rust

```rust
// Flatten Vec<Vec<T>>
let nested = vec![vec![1i32, 2], vec![3, 4], vec![5, 6]];
let flat: Vec<i32> = nested.into_iter().flatten().collect();
// → [1, 2, 3, 4, 5, 6]

// Flatten iterator of Options — None values are simply skipped
let opts: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
let values: Vec<i32> = opts.into_iter().flatten().collect();
// → [1, 3, 5]

// flatten() vs flat_map — equivalent
let text = "hello world\nfoo bar baz\nrust";
let words_via_flatten: Vec<&str> = text.lines()
    .map(|line| line.split_whitespace())   // Iterator<Item=SplitWhitespace>
    .flatten()                              // flatten one level
    .collect();
// equivalent to: text.lines().flat_map(|line| line.split_whitespace())

// Option flattening (not iterator — on Option itself)
let nested_opt: Option<Option<i32>> = Some(Some(42));
assert_eq!(nested_opt.flatten(), Some(42));

let none_inner: Option<Option<i32>> = Some(None);
assert_eq!(none_inner.flatten(), None);
```

`flatten()` only removes *one* level. For `Vec<Vec<Vec<T>>>` you'd call `flatten()` twice or restructure your data.

## What This Unlocks

- **Post-map flattening** — when `map` produces collections (iterators, vecs, options), `flatten` collapses the extra level.
- **Option chain unwrapping** — `Some(Some(x)).flatten()` collapses nested fallible operations into one `Option`.
- **Collecting multi-line or multi-field data** — turn `lines().map(split)` into a flat word/token stream.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Flatten one level | `List.flatten` / `List.concat` | `iter.flatten()` |
| Lazy | No (strict) | Yes |
| Works on `Option` | `Option.join` (not in stdlib) | `Option::flatten()` built-in |
| vs. `flat_map` | `concat_map f = map f >> flatten` | `flat_map(f)` = `.map(f).flatten()` |
| Depth | Exactly one level | Exactly one level |
