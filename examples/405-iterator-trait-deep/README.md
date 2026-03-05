📖 **[View on hightechmind.io →](https://hightechmind.io/rust/405-iterator-trait-deep)**

---

# 405: Iterator Adapters and Combinators

**Difficulty:** 2  **Level:** Intermediate

The full depth of Rust's iterator protocol — implementing custom iterators and composing complex lazy pipelines.

## The Problem This Solves

Processing collections is the bulk of most programs. Without a composable abstraction, you write explicit loops, temporary allocations, and imperative index-tracking code that obscures intent. Even with basic `map`/`filter` you hit a ceiling: you need custom step sizes, stateful transformations, early termination, or generating data on the fly.

Rust's `Iterator` trait is minimal by design — implement just `next()` and you get the entire adapter ecosystem for free. `map`, `filter`, `flat_map`, `take_while`, `scan`, `zip`, `chain`, and 70+ other adapters compose lazily. Nothing allocates until you call a *consuming* adapter like `collect()`, `sum()`, or `for_each()`. The entire pipeline is a single zero-allocation state machine.

Building your own iterator is how you express domain-specific iteration: Fibonacci sequences, file-line readers, tree traversals, tokenizers. Once it implements `Iterator`, every consumer in the language can use it.

## The Intuition

Implement `next()` on your type and you get the whole iterator machinery for free — every adapter, every consumer, all lazy.

## How It Works in Rust

```rust
// The entire Iterator trait — just one required method
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ...100+ provided methods built on next()
}

// Custom iterator: Fibonacci
struct Fib { a: u64, b: u64 }
impl Fib { fn new() -> Self { Fib { a: 0, b: 1 } } }

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a)  // infinite — never returns None
    }
}

// Now it works with the whole ecosystem
let sum: u64 = Fib::new().take(10).sum();  // 143
let evens: Vec<u64> = Fib::new().take(20).filter(|x| x % 2 == 0).collect();

// Lazy pipeline — nothing runs until consumed
let pipeline = (1..=1000)
    .filter(|x| x % 3 == 0)
    .map(|x| x * x)
    .take_while(|&x| x < 10_000);
// No work done yet ↑

let result: Vec<i32> = pipeline.collect(); // work happens here
```

1. Define a struct with iteration state.
2. `impl Iterator for YourStruct` — implement only `next()`.
3. Return `Some(value)` to continue, `None` to end.
4. Compose with any adapter: `.map()`, `.filter()`, `.flat_map()`, `.scan()`.

## What This Unlocks

- **Domain iterators**: Express any sequence — ranges, tree nodes, parsed tokens — as a first-class iterator.
- **Zero-allocation pipelines**: Chain 10 adapters; only the final `collect()` allocates.
- **Infinite iterators**: `Fib::new().take(n)` — generate lazily, consume exactly what you need.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sequence abstraction | `Seq.t` (lazy), `List.t` (strict) | `Iterator` trait — always lazy |
| Custom sequence | `let rec gen () = fun () -> Seq.Cons(...)` | `impl Iterator for MyStruct` |
| Adapter composition | `Seq.map`, `Seq.filter` (limited) | 70+ adapters, all composable |
| Allocation | `List` allocates; `Seq` defers | Adapters zero-alloc; `collect()` allocates |
| Infinite sequences | `Seq` supports infinite | `Iterator` supports infinite naturally |
