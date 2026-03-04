# 075: Difference List

**Difficulty:** 2  **Level:** Intermediate

Represent a list as a function from "the rest" to "the full list" — making append O(1) through function composition instead of copying.

## The Problem This Solves

Repeated list concatenation is quadratic. If you append N single-element lists together naively — `[1] ++ [2] ++ [3] ++ ...` — each `++` copies the left side. Appending to a list of length k takes O(k), so building a list of N elements by right-folding takes O(N²).

The difference list (or Hughes list) solves this with a clever indirection: instead of storing `[1, 2, 3]`, store a function `fn(rest) -> [1, 2, 3] + rest`. Appending two difference lists is just function composition: `f ∘ g`. No copying, no allocation — just one more function call in the chain. You only pay to materialize the result when you call `to_vec()` at the end.

In OCaml and Haskell, this pattern appears frequently in pretty-printers, parser combinators, and log builders. In Rust, `Vec::extend` is already amortized O(1) per element — so difference lists are mostly an exercise in understanding higher-order functions and closure composition.

## The Intuition

A regular list `[1, 2, 3]` is a value. A difference list `f` is a function: `f(rest) = [1, 2, 3] ++ rest`. To append two difference lists `f` and `g`, you create `h` where `h(rest) = f(g(rest))` — just compose them. No data is touched until you finally call `h([])` to materialize the full list.

In Python terms: instead of `list1 + list2`, you're composing `lambda rest: list1 + list2 + rest`. The cost moves from append-time to materialize-time.

In Rust, closures compose naturally — but because each closure has a unique type, you need `Box<dyn Fn>` to store and chain them. This makes Rust's difference list slightly less ergonomic than Haskell's but structurally identical.

## How It Works in Rust

```rust
pub struct DList<T> {
    f: Box<dyn Fn(Vec<T>) -> Vec<T>>,  // "given a tail, return our elements + tail"
}

impl<T: 'static + Clone> DList<T> {
    pub fn empty() -> Self {
        DList { f: Box::new(|rest| rest) }  // identity: nothing to prepend
    }

    pub fn singleton(x: T) -> Self {
        DList { f: Box::new(move |mut rest| { rest.insert(0, x.clone()); rest }) }
    }

    // O(1) append — just compose the two functions
    pub fn append(self, other: DList<T>) -> DList<T> {
        DList {
            f: Box::new(move |rest| (self.f)((other.f)(rest)))
            //                       ^^^^^^^^  apply other first, then self
        }
    }

    // Materialize: apply the composed function to an empty list
    pub fn to_vec(&self) -> Vec<T> {
        (self.f)(vec![])
    }
}

// Usage: N appends at O(1) each, one O(N) materialization at the end
let result = DList::from_vec(vec![1, 2])
    .append(DList::from_vec(vec![3, 4]))
    .append(DList::singleton(5));
assert_eq!(result.to_vec(), vec![1, 2, 3, 4, 5]);
```

```rust
// Rust's practical alternative: Vec with pre-allocated capacity
fn concat_many(lists: &[Vec<i32>]) -> Vec<i32> {
    let total = lists.iter().map(|l| l.len()).sum();
    let mut result = Vec::with_capacity(total);  // one allocation
    for list in lists { result.extend(list); }   // O(N) total, no copies
    result
}
```

## What This Unlocks

- **Pretty-printer builders**: accumulate document fragments with O(1) concat, render once at the end — the classic use in Haskell's `showsPrec`.
- **Log and string builders**: in functional code where you can't mutate a buffer, difference lists let you build strings by composition without quadratic cost.
- **Understanding HOF composition**: tracing how `f ∘ g` chains and what "deferred computation" means in a strict language — a mental model that transfers to monads, continuation-passing style, and parser combinators.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Representation | `type 'a dlist = 'a list -> 'a list` | `struct DList<T> { f: Box<dyn Fn(Vec<T>) -> Vec<T>> }` |
| Empty | `fun x -> x` (identity) | `Box::new(\|rest\| rest)` |
| Append | `fun rest -> f (g rest)` — function composition | `Box::new(move \|rest\| (self.f)((other.f)(rest)))` |
| Materialize | `f []` | `(self.f)(vec![])` |
| Practical use | Common in pretty-printers, parsers | Rarely needed — `Vec::extend` is already efficient |
| Closure storage | First-class functions, no boxing needed | `Box<dyn Fn>` required (closures have unique types) |
