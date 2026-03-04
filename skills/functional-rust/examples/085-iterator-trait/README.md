# 085: Iterator Trait

**Difficulty:** 2  **Level:** Intermediate

Implement the `Iterator` trait for a custom type — define just `next()` and get `map`, `filter`, `take`, `sum`, `collect`, and 70+ other methods for free.

## The Problem This Solves

You have a custom data structure and you want to loop over it, transform it, filter it, sum it. Without implementing `Iterator`, you're stuck writing your own `for_each`, `map`, `filter` methods by hand — or forcing callers to extract a `Vec` first.

Rust's `Iterator` trait is a single-method protocol: implement `fn next(&mut self) -> Option<Self::Item>` and you get the entire iterator toolkit at no extra cost. Every adapter (`map`, `filter`, `enumerate`, `chain`) and every consumer (`sum`, `collect`, `find`, `max`) in the standard library works with your type automatically.

This is one of the clearest examples of Rust's trait system paying off: one small implementation unlocks an enormous amount of functionality.

## The Intuition

In Python, you implement `__iter__` and `__next__` on a class — and then `for`, `list()`, `sum()`, and all list comprehensions work with your type. In Java, you implement `Iterator<T>` with `hasNext()` and `next()`. In OCaml, you implement a `Seq`-compatible generator.

Rust's model is closer to Python's: one method (`next`), infinite power. The difference is that all the adapter methods are defined directly on the trait, so `my_iter.map(f).filter(p).take(5)` chains with zero overhead — each adapter is a zero-cost wrapper that forwards `next()` calls.

The associated type `Item` tells the compiler what type this iterator yields. The iterator itself decides — callers don't need to specify it.

## How It Works in Rust

```rust
// A finite iterator over a range
struct Range { current: i32, end: i32 }

impl Iterator for Range {
    type Item = i32;      // this iterator yields i32s

    fn next(&mut self) -> Option<i32> {
        if self.current >= self.end {
            None           // signals end of iteration
        } else {
            let val = self.current;
            self.current += 1;
            Some(val)      // yield current value, advance state
        }
    }
}

// You get all of this for FREE from one next() implementation:
let nums: Vec<i32> = Range { current: 1, end: 6 }.collect();       // [1,2,3,4,5]
let doubled: Vec<i32> = Range { current: 1, end: 6 }.map(|x| x * 2).collect();
let sum: i32 = Range { current: 1, end: 101 }.sum();               // 5050
```

```rust
// An infinite iterator — returns Some forever
struct Counter { current: u64 }

impl Iterator for Counter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let val = self.current;
        self.current += 1;
        Some(val)   // never returns None — use .take(n) to stop
    }
}

// Safe to chain because nothing is computed until consumed
let odd_squares: Vec<u64> = Counter { current: 1 }
    .map(|x| x * x)
    .filter(|x| x % 2 == 1)
    .take(5)
    .collect();   // [1, 9, 25, 49, 81]
```

```rust
// Generic functions work with any iterator via Iterator bound
fn sum_first_n<I: Iterator<Item = i32>>(iter: I, n: usize) -> i32 {
    iter.take(n).sum()
}
```

## What This Unlocks

- **Custom data structures become first-class**: trees, graphs, circular buffers, lazy generators — anything with a `next()` participates fully in Rust's iterator ecosystem.
- **Zero-cost pipelines**: `my_iter.map(f).filter(p).take(n).collect()` compiles to an efficient loop — no intermediate `Vec` allocations, no overhead.
- **Composability**: your iterator chains with `std::iter::chain`, `zip`, `flat_map` — all the standard machinery works with your type.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Iterator protocol | `Seq.t = unit -> node` (functional, lazy) | `Iterator` trait with `next(&mut self)` (stateful) |
| Item type | Inferred or in module signature | `type Item` (associated type) |
| Infinite sequences | `Seq.unfold` | `impl Iterator` that always returns `Some(...)` |
| Free combinators | `Seq.map`, `Seq.filter` from stdlib | 70+ methods auto-available from the trait |
| Termination | Returns `Seq.Nil` | Returns `None` from `next()` |
