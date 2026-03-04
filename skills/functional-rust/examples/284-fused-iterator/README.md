# 284: FusedIterator

**Difficulty:** 3  **Level:** Advanced

A zero-cost marker trait that guarantees your iterator returns `None` forever once it first returns `None` — enabling optimizer skips and safe adapter composition.

## The Problem This Solves

The `Iterator` contract says: once `next()` returns `None`, all future calls *should* return `None`. But this is a convention, not enforced by the type system. A buggy or deliberately weird iterator can return `Some` after returning `None` — and some iterator adapters have to guard against this with extra checks.

`FusedIterator` is the compiler-enforced version of that guarantee. Once you implement it on your type, the optimizer knows it can eliminate redundant "is this iterator done?" checks in adapter pipelines. More practically, it's a signal to callers: "this iterator terminates cleanly, you don't need to worry about use-after-done weirdness."

All standard library iterators already implement `FusedIterator`. When writing custom iterators that naturally terminate (counters, finite sequences, data readers), implementing it costs nothing and gives your users the same guarantee.

If you have a non-fused iterator and need a fused wrapper, `.fuse()` wraps it in an adapter that tracks the first `None` and returns `None` forever after.

## The Intuition

A marker trait that promises "once I return `None`, I'll return `None` on every subsequent call" — letting adapters skip redundant termination checks.

## How It Works in Rust

```rust
use std::iter::FusedIterator;

struct Countdown { n: i32 }

impl Iterator for Countdown {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.n <= 0 { return None; }  // once None...
        let val = self.n;
        self.n -= 1;
        Some(val)
    }
}

// Declare that Countdown never returns Some after returning None
impl FusedIterator for Countdown {}  // empty impl — just the marker

// Now callers have the guarantee:
let mut cd = Countdown::new(3);
cd.next();  // Some(3)
cd.next();  // Some(2)
cd.next();  // Some(1)
cd.next();  // None
cd.next();  // None — guaranteed by FusedIterator
cd.next();  // None — always

// Non-fused iterator: can return Some after None (surprising!)
// Use .fuse() to wrap any iterator with the guarantee:
let safe = weird_iterator.fuse();
while let Some(x) = safe.next() { /* ... */ }
safe.next();  // None, guaranteed — even if weird_iterator would have returned Some

// All std iterators are already fused:
let mut v = vec![1, 2, 3].into_iter();
v.next(); v.next(); v.next();
v.next();  // None
v.next();  // None — fused
```

## What This Unlocks

- **Custom iterator correctness:** Mark your counters, readers, and generators as `FusedIterator` to give callers the standard guarantee — costs nothing to implement.
- **Optimizer hints:** Iterator adapters can skip the "check if done" step for fused iterators, reducing branching in hot loops.
- **Safe composition:** `.fuse()` wraps any iterator as a safety net when you're unsure whether a third-party iterator is well-behaved after termination.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Termination guarantee | Convention only | `FusedIterator` marker trait |
| Enforcement | Not enforced by type system | Compiler-checked via trait |
| Standard iterators | All well-behaved (convention) | All implement `FusedIterator` |
| Custom iterators | Programmer's responsibility | `impl FusedIterator for MyIter {}` — empty impl |
| Safety wrapper | N/A | `.fuse()` adapter |
