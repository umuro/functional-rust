# 255: Lazy Fibonacci

**Difficulty:** 2  **Level:** Intermediate

Generate an infinite Fibonacci sequence without computing all of it — take only what you need, when you need it.

## The Problem This Solves

The Fibonacci sequence is infinite. You can't compute it all before you use it. But often you need a prefix: the first 10, the first 100, however many satisfy some condition. The challenge is decoupling "how to generate the next value" from "how many values to consume".

Eager languages compute everything upfront. A function returning a list of Fibonacci numbers must know in advance how many to generate — hard-code a limit, or blow the stack with naive recursion. Lazy evaluation solves this by deferring each computation until it's actually needed.

This example shows two Rust approaches: the idiomatic `Iterator` (lazy by design, zero overhead, understood by the entire standard library) and a thunk-based `Stream` that mirrors OCaml's coinductive stream type exactly. Both are safe with infinite sequences as long as you only force a finite prefix.

## The Intuition

A lazy sequence is like a vending machine: you press the button to get the next item. The machine doesn't pre-fill all the items — it generates each one on demand. Pressing the button 10 times gives you 10 items; the machine could keep going forever.

OCaml's stream stores each tail as a *thunk* — a `fun () -> ...` — which is a suspended computation. The tail doesn't exist until you call it. Rust's `Iterator` trait works the same way: `next()` produces one value on each call, and the struct holds just enough state (two numbers) to compute the next.

A `move` closure in Rust captures its variables by ownership. Each thunk owns its two Fibonacci numbers independently — no shared state, no borrowing complications, stack-safe for any depth.

## How It Works in Rust

```rust
// Style 1: Idiomatic — implement Iterator, use the standard library for free
pub struct FibIter { a: u64, b: u64 }

impl Iterator for FibIter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let value = self.a;
        let next_b = self.a + self.b;  // compute next before updating
        self.a = self.b;
        self.b = next_b;
        Some(value)  // never returns None — infinite iterator
    }
}

// Use: take first 10 Fibonacci numbers
let fibs: Vec<u64> = FibIter { a: 0, b: 1 }.take(10).collect();
// → [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

// Style 2: Thunk-based — mirrors OCaml's stream type exactly
struct Stream<T> {
    head: T,
    tail: Box<dyn Fn() -> Stream<T>>,  // Box required: Stream contains itself
}

fn fibs_stream(a: u64, b: u64) -> Stream<u64> {
    Stream {
        head: a,
        tail: Box::new(move || fibs_stream(b, a + b)),  // move captures a, b by value
    }
}

fn take_stream<T: Clone>(n: usize, s: Stream<T>) -> Vec<T> {
    if n == 0 { return vec![]; }
    let mut result = vec![s.head.clone()];
    result.extend(take_stream(n - 1, (s.tail)()));
    result
}
```

## What This Unlocks

- **Infinite sequences** — primes, factorials, random numbers, sensor readings: any sequence where you don't know the length upfront.
- **On-demand computation** — combine with `take_while`, `filter`, `zip` to compute only what a downstream consumer needs.
- **Memory efficiency** — `FibIter` holds two `u64` values regardless of how many elements you take; no list grows in memory.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy abstraction | Custom `'a stream` type with thunks | `Iterator` trait — standard library aware |
| Heap indirection | GC handles recursive type transparently | `Box<dyn Fn()>` required for recursive type |
| Closure capture | By reference (GC-managed) | `move` closure — takes ownership |
| Infinite safety | Safe if only `take n` is forced | Safe — `Iterator::take` is finite |
| Standard library | Custom `take`, `map` needed | `take`, `map`, `filter`, `zip` all built in |
