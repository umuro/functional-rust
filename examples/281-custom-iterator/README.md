📖 **[View on hightechmind.io →](https://hightechmind.io/rust/281-custom-iterator)**

---

# 281: Implementing Iterator Trait from Scratch

**Difficulty:** 3  **Level:** Advanced

Implement `Iterator` on your own struct — one method required, the entire adapter ecosystem unlocked for free.

## The Problem This Solves

Your data structure has a natural traversal order, but it's not a slice or a vec. A graph node's BFS frontier. A custom number sequence — squares, Fibonacci, primes. A lazy generator that computes each value on demand. You need these to work with `map`, `filter`, `zip`, `take`, `collect`, and every other iterator tool without reimplementing them.

The power of Rust's iterator design is the minimal requirement: implement one method — `next()` — and you immediately get the full iterator toolkit for free. No inheritance hierarchy, no magic traits to satisfy, no runtime dispatch required.

The alternative is implementing every transformation yourself, or collecting everything into a `Vec` and using slice iteration — which forces eager evaluation and allocates even when you'd only consume the first few elements.

## The Intuition

Implement `Iterator` by defining `type Item` and `fn next(&mut self) -> Option<Self::Item>`. Return `Some(value)` to yield a value, `None` to signal exhaustion.

```rust
impl Iterator for MyType {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        // compute next value, return Some or None
    }
}
// Now: MyType.map(), .filter(), .zip(), .collect() — all free
```

## How It Works in Rust

```rust
struct Squares { current: u32, max: u32 }

impl Squares {
    fn new(max: u32) -> Self { Squares { current: 0, max } }
}

impl Iterator for Squares {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max { return None; }
        let val = self.current * self.current;
        self.current += 1;
        Some(val)  // yield square of current position
    }
}

// All iterator adapters work immediately
let squares: Vec<u32> = Squares::new(6).collect();
// → [0, 1, 4, 9, 16, 25]

let sum_of_squares: u32 = Squares::new(6).sum();
// → 55

let big_squares: Vec<u32> = Squares::new(10)
    .filter(|&x| x > 10)
    .collect();
// → [16, 25, 36, 49, 64, 81]

// Infinite iterator — signals infinity by always returning Some
struct Fibonacci { a: u64, b: u64 }
impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(val)  // never returns None — must use take() to bound
    }
}

let fibs: Vec<u64> = Fibonacci { a: 0, b: 1 }.take(10).collect();
// → [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

// Zip two custom iterators — works because both implement Iterator
let zipped: Vec<(u32, u64)> = Squares::new(5)
    .zip(Fibonacci { a: 0, b: 1 })
    .collect();
```

## What This Unlocks

- **Lazy sequences** — compute values only when consumed; infinite sequences become trivial.
- **Custom data structure traversal** — implement `Iterator` on trees, graphs, or domain objects to integrate with the full Rust ecosystem.
- **Zero-cost abstractions** — custom iterators compile to the same machine code as hand-written loops with no allocation overhead.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom sequence | `Seq.t` (thunk-based) or module | Implement `Iterator` trait |
| Minimum required | `unit -> 'a Seq.node` | `fn next(&mut self) -> Option<Item>` |
| Adapters for free | No — separate `Seq.*` functions | Yes — all `Iterator` methods available |
| Infinite sequences | Natural with `Seq` | Natural — return `Some` forever, bound with `take()` |
| State storage | Closure captures | Struct fields |
