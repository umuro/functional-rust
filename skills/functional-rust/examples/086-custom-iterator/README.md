# 086: Custom Iterators

**Difficulty:** 2  **Level:** Intermediate

Build your own iterators by implementing the `Iterator` trait — then use the entire standard library's iterator toolkit for free.

## The Problem This Solves

The standard library gives you iterators over slices, ranges, and collections. But what about a Fibonacci sequence? A custom step range? A mathematical sequence like Collatz? These don't fit into any existing collection — they compute the next value from the current state.

In Python you'd write a generator with `yield`. In Java you'd implement the `Iterator` interface. In Rust, you implement the `Iterator` trait — and that's where it gets interesting. Implementing just **one method** (`next`) unlocks `.map()`, `.filter()`, `.take()`, `.sum()`, `.collect()`, `.zip()`, and dozens more, for free. The whole standard library becomes your toolkit.

This is Rust's trait system at work: define the core contract once (what does `next()` return?), and everything built on top of that contract becomes available automatically.

## The Intuition

Python's generator protocol:
```python
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

list(itertools.islice(fibonacci(), 10))  # [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

Rust doesn't have `yield` (yet), but a struct with `impl Iterator` is just as expressive:
```rust
// State lives in the struct, next() updates it and returns the next value
let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
```

The key insight: **state lives in the struct, not in function-local variables.** The struct is the iterator's "pause point" — every time `next()` is called, it reads the state, computes the next value, updates the state, and returns.

## How It Works in Rust

```rust
// Struct holds all the state the iterator needs between calls
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self { Fibonacci { a: 0, b: 1 } }
}

// Implementing Iterator requires only ONE method: next()
impl Iterator for Fibonacci {
    type Item = u64;  // what type does this iterator produce?

    fn next(&mut self) -> Option<u64> {
        let val = self.a;            // remember current value to return
        let next = self.a + self.b;  // compute the next one
        self.a = self.b;             // advance state
        self.b = next;
        Some(val)                    // always Some — this is an infinite iterator
    }
}

// Finite iterator: returns None when done
struct StepRange<T> { current: T, end_: T, step: T }

impl Iterator for StepRange<i64> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.current >= self.end_ {
            None  // signal: no more values
        } else {
            let val = self.current;
            self.current += self.step;
            Some(val)
        }
    }
}

// Terminating iterator: stops when it reaches a known endpoint
struct Collatz { current: u64, done_: bool }

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.done_ { return None; }
        let val = self.current;
        if val == 1 {
            self.done_ = true;                 // next call will return None
        } else if val % 2 == 0 {
            self.current = val / 2;
        } else {
            self.current = 3 * val + 1;
        }
        Some(val)
    }
}

// One impl gives you the entire standard iterator API for free:
let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

let evens: Vec<i64> = StepRange::new(0, 10, 2).collect();
// [0, 2, 4, 6, 8]

let collatz: Vec<u64> = Collatz::new(6).collect();
// [6, 3, 10, 5, 16, 8, 4, 2, 1]

// Combine with standard adapters — no extra work required
let even_fibs: Vec<u64> = Fibonacci::new()
    .filter(|x| x % 2 == 0)
    .take(5)
    .collect();
// [0, 2, 8, 34, 144]

let fib_sum: u64 = Fibonacci::new().take(10).sum();
// 88
```

Use `None` to signal the end of a finite sequence. For infinite sequences (like Fibonacci), always return `Some` and let the caller decide when to stop with `.take(n)`.

## What This Unlocks

- **Domain-specific sequences** — iterate over time intervals, sensor readings, game states, pagination cursors — anything with a "give me the next one" structure
- **Lazy infinite streams** — generate as many values as needed without computing them all upfront; combine with `.take()`, `.take_while()`, or `.find()`
- **Full iterator interop** — your custom iterator works seamlessly with `.zip()`, `.enumerate()`, `.flat_map()`, and everything else in the standard library

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom iteration | `Seq` thunks or mutable closures | Struct + `impl Iterator` |
| State storage | `ref` cells in closures | Struct fields |
| Mutation | Explicit `ref` / `:=` | `&mut self` in `next()` |
| End of sequence | Return `Seq.Nil` | Return `None` |
| Free adapters | `Seq.map`, `Seq.filter`, etc. | Everything in `std::iter` — `.map()`, `.filter()`, `.take()`, `.sum()`, ... |
