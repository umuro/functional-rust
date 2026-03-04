# 288: Iterator collect()

**Difficulty:** 2  **Level:** Intermediate

Materialize a lazy iterator into any collection type — `Vec`, `HashMap`, `HashSet`, `String`, `BTreeMap`, and more — including collecting `Result<Vec<T>>` from a fallible stream.

## The Problem This Solves

An iterator is lazy — it produces elements on demand but doesn't store them. At some point you need the results in a concrete data structure you can index, share, or pass to APIs that don't accept iterators. Without `collect()`, you'd write a `for` loop pushing elements into a pre-allocated collection — boilerplate that drowns out the intent.

`collect()` generalizes this: any type implementing `FromIterator<T>` can receive the elements. The *type you collect into* determines how elements are assembled. Collect `(K, V)` pairs into a `HashMap`. Collect `char`s into a `String`. Collect `Option<T>` or `Result<T, E>` — the entire collection short-circuits on the first `None` or `Err`, returning a single `Option<Vec<T>>` or `Result<Vec<T>, E>`.

The main challenge is type inference: `collect()` can't infer the target type from the iterator alone. You need either a type annotation on the binding (`let v: Vec<i32> = ...`) or the turbofish (`.collect::<Vec<i32>>()`).

## The Intuition

Tell Rust what collection type you want; `collect()` drives the `FromIterator` machinery to assemble all elements into it.

## How It Works in Rust

```rust
// Vec — the default
let squares: Vec<u32> = (0..5).map(|x| x * x).collect();
// → [0, 1, 4, 9, 16]

// HashSet — automatic deduplication
let set: HashSet<i32> = vec![1, 2, 2, 3, 3, 3].into_iter().collect();
// set.len() == 3

// HashMap — from (K, V) iterator
let map: HashMap<&str, u32> = [("a", 1), ("b", 2)].into_iter().collect();

// String — from chars
let s: String = ['R', 'u', 's', 't'].iter().collect();
// → "Rust"

// BTreeMap — sorted by key
let bmap: BTreeMap<i32, i32> = (0..5).map(|x| (x, x*x)).collect();

// Result<Vec<T>> — fail fast on first parse error
let nums: Result<Vec<i32>, _> = ["1", "2", "3"].iter()
    .map(|s| s.parse::<i32>())
    .collect();  // Ok([1, 2, 3])

let broken: Result<Vec<i32>, _> = ["1", "oops", "3"].iter()
    .map(|s| s.parse::<i32>())
    .collect();  // Err(ParseIntError) — stops at first failure
```

The turbofish syntax: `.collect::<Vec<_>>()` — `_` lets Rust infer the element type, you only specify the container.

## What This Unlocks

- **Fallible batch processing:** `collect::<Result<Vec<_>, _>>()` runs every parse/conversion and either gives you all results or the first error — one expression replaces a try/catch loop.
- **Deduplication:** `.collect::<HashSet<_>>()` removes duplicates with no extra code; `.collect::<BTreeSet<_>>()` does the same with sorted output.
- **String assembly:** Collect `char` iterators, `&str` slices, or `String` segments directly into a `String` — no `push_str` loop needed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| To list | Natural — lists are the base type | `.collect::<Vec<_>>()` |
| To set | `List.sort_uniq compare` (manual) | `.collect::<HashSet<_>>()` |
| To map | `List.fold_left` into Map | `.collect::<HashMap<K,V>>()` |
| Type selection | Inferred from context | Annotation or turbofish required |
| Fallible collect | Manual fold with error check | `.collect::<Result<Vec<_>, _>>()` |
