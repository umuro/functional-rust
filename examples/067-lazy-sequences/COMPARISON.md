# Lazy Sequences — OCaml vs Rust Comparison

## Core Insight

OCaml is strict (eager) by default and needs the `Seq` module for laziness. Rust iterators are lazy by default — chaining `.filter().map().take()` builds a pipeline that computes nothing until consumed by `collect()`, `sum()`, or `for_each()`. This makes Rust's approach to lazy sequences more natural.

## OCaml Approach

The `Seq` module (OCaml 4.14+) provides `Seq.ints`, `Seq.unfold`, `Seq.filter`, `Seq.take_while`, etc. Under the hood, `Seq` uses thunks (`unit -> 'a node`) for lazy evaluation. Before `Seq`, OCaml programmers used custom stream types with explicit thunks.

## Rust Approach

Infinite ranges `(0u64..)` are valid iterators. `std::iter::successors` generates sequences from a seed (like `Seq.unfold`). All iterator adaptors (`.filter()`, `.map()`, `.take()`) are lazy — they return new iterator types that wrap the original. The compiler monomorphizes the entire chain into efficient code.

## Comparison Table

| Aspect        | OCaml                          | Rust                                  |
|---------------|--------------------------------|---------------------------------------|
| **Memory**    | Thunk closures (GC'd)         | Zero-alloc iterator state (stack)     |
| **Null safety** | N/A                         | N/A                                   |
| **Errors**    | N/A                           | N/A                                   |
| **Iteration** | `Seq.take` + `Seq.iter`       | `.take(n).collect()` or `.for_each()` |
| **Laziness**  | Explicit (Seq module)          | Default (all iterators are lazy)      |

## Things Rust Learners Should Notice

1. **Ranges are iterators** — `0..` is an infinite iterator, `0..10` is a finite one
2. **`std::iter::successors`** — Rust's equivalent of `Seq.unfold` for stateful generation
3. **Zero-cost abstraction** — the iterator chain compiles to a simple loop, no heap allocation
4. **`impl Iterator<Item = T>`** — return type hides the complex composed iterator type
5. **Consuming adaptors** — `.collect()`, `.sum()`, `.count()` trigger actual computation

## Further Reading

- [Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [std::iter::successors](https://doc.rust-lang.org/std/iter/fn.successors.html)
- [Infinite ranges](https://doc.rust-lang.org/std/ops/struct.RangeFrom.html)
