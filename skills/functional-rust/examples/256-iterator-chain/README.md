# 256: Chaining Iterators with chain()

**Difficulty:** 1  **Level:** Beginner

Combine multiple sequences into one seamless iterator — lazily, with zero extra allocation.

## The Problem This Solves

You have two lists and you want to process them together: all the items from list A, then all the items from list B. The obvious approach is to create a new combined list — but that allocates memory just to iterate. For large datasets, that's wasteful.

Maybe you want to iterate over the default config values followed by the user-provided overrides. Or process log entries from multiple files. Or combine the results of two database queries. The naive approach concatenates them into a new `Vec` first, but you only need the combination long enough to iterate once.

Rust's `.chain()` is the elegant solution: it creates a single iterator that goes through the first sequence, then the second, without allocating any storage for the combined result. The two original iterators stay exactly where they are; `.chain()` just connects them. This is a key example of how Rust iterators are *lazy* — they describe what to do, not the result.

## The Intuition

In Python you'd use `itertools.chain()` for exactly this:
```python
from itertools import chain
combined = list(chain([1, 2, 3], [4, 5, 6]))
```

In JavaScript you might use spread syntax: `[...arr1, ...arr2]` — but that creates a new array immediately.

Rust's `.chain()` is closer to Python's `itertools.chain`: nothing is materialized until you ask for it. You have to call `.collect()` (or loop with `for`) to actually run the computation:

```rust
// Nothing runs yet — this just describes the computation
let lazy = first.iter().chain(second.iter());

// NOW it runs — both iterators are consumed in sequence
let combined: Vec<i32> = lazy.collect();
```

## How It Works in Rust

```rust
let first  = [1, 2, 3];
let second = [4, 5, 6];

// .chain() connects two iterators — lazy, no allocation
let chained: Vec<i32> = first.iter()
    .chain(second.iter())
    .copied()    // turn &i32 into i32 (copy the value out of the reference)
    .collect();  // THIS is when the work actually happens
// [1, 2, 3, 4, 5, 6]

// Works with any iterators — not just slices
let greetings = vec!["hello", "hi", "hey"];
let farewells = vec!["bye", "goodbye", "ciao"];
let all: Vec<_> = greetings.iter().chain(farewells.iter()).collect();

// Works with computed iterators too
let evens = (0..10i32).filter(|x| x % 2 == 0);
let odds  = (0..10i32).filter(|x| x % 2 != 0);
let combined: Vec<i32> = evens.chain(odds).collect();
// [0, 2, 4, 6, 8, 1, 3, 5, 7, 9]

// Chain three iterators by chaining twice
let a = vec![1i32];
let b = vec![2i32];
let c = vec![3i32];
let abc: Vec<i32> = a.into_iter().chain(b).chain(c).collect();
// [1, 2, 3]

// chain() is just an iterator adapter — you can keep applying more operations
let sum: i32 = first.iter().chain(second.iter()).copied().sum();
// 21 — no Vec ever created
```

Note `into_iter()` vs `.iter()`: `.iter()` borrows the collection (giving `&T`), `into_iter()` consumes it (giving `T`). For the chain with three `Vec`s above, we use `into_iter()` because the Vecs are consumed into the chain.

## What This Unlocks

- **Multi-source processing** — combine events from multiple queues, merge log files, union query results without creating intermediate collections
- **Fallback sequences** — iterate through primary results, then fallback results, stopping at the first match (with `.find()` after `.chain()`)
- **Lazy pipelines** — chain multiple sources and then `.map()`, `.filter()`, `.take()` on the combined stream — all without materializing intermediate data

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Concatenate lists | `List.append xs ys` or `xs @ ys` | `xs.iter().chain(ys.iter())` |
| Laziness | Eager (allocates immediately) | Lazy — nothing runs until consumed |
| Memory cost | New list allocated | Zero allocation in the iterator |
| Result type | `'a list` | `Chain<IterA, IterB>` — a new iterator type |
| Materialize | Already materialized | `.collect()` when you need a `Vec` |
