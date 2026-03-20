📖 **[View on hightechmind.io →](https://hightechmind.io/rust/284-fused-iterator)**

---

# 284: FusedIterator for Stable Termination

## Problem Statement

The `Iterator` specification in Rust says that after `next()` returns `None`, subsequent behavior is undefined — some iterators might return `Some` again if called further (a "restart"). This makes it unsafe to call `next()` after termination without an explicit check. The `FusedIterator` marker trait solves this by promising that once `next()` returns `None`, all future calls also return `None` — enabling safe and optimized composition in adapters that call `next()` multiple times.

## Learning Outcomes

- Understand `FusedIterator` as a marker trait guaranteeing "once None, always None"
- Implement `FusedIterator` correctly (only when the implementation actually fuses)
- Recognize that standard library adapters like `filter()`, `map()`, and `zip()` require `FusedIterator` for correctness guarantees
- Use `fuse()` on any iterator to wrap it in a guaranteed-fused adapter

## Rust Application

Implementing `FusedIterator` is a marker — it has no methods. You simply declare `impl FusedIterator for Countdown {}` after ensuring your `next()` implementation never returns `Some` after a `None`:

```rust
use std::iter::FusedIterator;

pub struct Countdown { n: i32 }

impl Iterator for Countdown {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.n <= 0 { return None; }
        let val = self.n;
        self.n -= 1;
        Some(val)
    }
}

impl FusedIterator for Countdown {} // safe: n never increases after reaching 0

// fuse() wraps any iterator to guarantee this property:
let fused = some_iterator.fuse(); // FusedIterator regardless of original
```

## OCaml Approach

OCaml's `Seq` type is inherently fused — once a sequence node is `Nil`, there is no way to call `next` again (sequences are values, not objects). The concept of "calling next after None" does not apply to OCaml's immutable sequence type:

```ocaml
(* OCaml Seq terminates structurally — the empty sequence is just Seq.empty *)
let seq = Seq.empty  (* calling next "again" is impossible by construction *)
```

## Key Differences

1. **Structural vs behavioral**: OCaml's `Seq.Nil` terminates the sequence structurally (you can't call it again); Rust requires an explicit contract via `FusedIterator`.
2. **Adapter safety**: Standard library adapters like `peekable()` and `chain()` explicitly document whether they require or provide `FusedIterator`.
3. **`fuse()` adapter**: Rust provides `Iterator::fuse()` to wrap any iterator in a fused version — `FusedIterator` is the opt-in promise, `fuse()` is the runtime enforcement.
4. **Performance**: Well-implemented `FusedIterator` types allow adapter code to skip the "check if already exhausted" bookkeeping.

## Exercises

1. Implement a non-fused iterator (one that sometimes returns `Some` after `None`) and demonstrate the incorrect behavior, then fix it by fusing.
2. Show that `iterator.fuse()` wraps the iterator in a type that implements `FusedIterator` regardless of whether the original did.
3. Implement `FusedIterator` on a bounded random-number generator that stops after generating exactly N values.
